use std::time::{Duration, Instant};

use egui::{self, Checkbox, Color32, Frame, RichText, Ui};
use log::{info, warn};
use pvz_sdk::Popcapgame;
use pvz_sdk::toggleables::{self, ToggleCheatError, Toggleable};

/// How often to look for the game while it isn't running.
const RETRY_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Default)]
pub struct MyApp {
    connection: Connection,
}

/// Whether the window is attached to the game yet.
enum Connection {
    /// The game isn't running, or couldn't be opened. Tried again every [`RETRY_INTERVAL`].
    Waiting {
        last_attempt: Option<Instant>,
        /// Why the last attempt failed.
        error: Option<String>,
    },
    Attached {
        game: Popcapgame,
        cheats: Vec<CheatRow>,
    },
}

impl Default for Connection {
    fn default() -> Self {
        Self::Waiting {
            last_attempt: None,
            error: None,
        }
    }
}

impl Connection {
    /// Attaches to the game if it's time to try again.
    fn retry(&mut self) {
        let Self::Waiting {
            last_attempt,
            error,
        } = self
        else {
            return;
        };
        if last_attempt.is_some_and(|at| at.elapsed() < RETRY_INTERVAL) {
            return;
        }
        *last_attempt = Some(Instant::now());

        match Popcapgame::init() {
            Ok(game) => {
                info!("Attached to the game");
                let cheats = toggleables::ALL
                    .iter()
                    .map(|cheat| CheatRow::new(cheat, &game))
                    .collect();
                *self = Self::Attached { game, cheats };
            }
            Err(attach_error) => *error = Some(attach_error.to_string()),
        }
    }
}

/// A cheat and its state, as one row in the window.
struct CheatRow {
    cheat: &'static dyn Toggleable,
    /// Whether the cheat is on, or why it can't be used.
    state: Result<bool, ToggleCheatError>,
}

#[inline(always)]
fn toggleables_header_font() -> RichText {
    RichText::new("Toggleables")
        .size(30.)
        .color(Color32::MAGENTA)
}

#[inline(always)]
fn togglecheat_font(name: &'static str) -> RichText {
    RichText::new(name).size(24.).strong()
}

impl CheatRow {
    fn new(cheat: &'static dyn Toggleable, game: &Popcapgame) -> Self {
        let state = cheat.is_activated(game);
        if let Err(error) = &state {
            warn!("{} can't be used: {error}", cheat.name());
        }
        Self { cheat, state }
    }

    fn show(&mut self, ui: &mut Ui, game: &Popcapgame) {
        let label = togglecheat_font(self.cheat.name());
        match &mut self.state {
            Ok(toggled) => {
                if ui.checkbox(toggled, label).changed() {
                    info!("Detected change in {} to {toggled}", self.cheat.name());
                    if let Err(error) = self.cheat.toggle(game, *toggled) {
                        warn!("Couldn't toggle {}: {error}", self.cheat.name());
                        self.state = Err(error);
                    }
                }
            }
            Err(error) => {
                ui.add_enabled(false, Checkbox::new(&mut false, label));
                ui.colored_label(Color32::LIGHT_RED, "⚠")
                    .on_hover_text(error.to_string());
            }
        }
    }
}

fn render_toggleables(ui: &mut Ui, game: &Popcapgame, cheats: &mut [CheatRow]) {
    egui::Grid::new("id_salt").striped(true).show(ui, |ui| {
        for cheat in cheats {
            cheat.show(ui, game);
            ui.end_row();
        }
    });
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.4]
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.connection.retry();
        egui::Window::new(toggleables_header_font())
            .auto_sized()
            .show(ui, |ui| match &mut self.connection {
                Connection::Attached { game, cheats } => render_toggleables(ui, game, cheats),
                Connection::Waiting { error, .. } => {
                    ui.label("Waiting for Plants vs. Zombies Replanted to start…");
                    if let Some(error) = error {
                        ui.weak(error.as_str());
                    }
                    ui.ctx().request_repaint_after(RETRY_INTERVAL);
                }
            });

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ui, |ui| {
                if ui
                    .interact(
                        ui.max_rect(),
                        egui::Id::new("window-drag"),
                        egui::Sense::drag(),
                    )
                    .dragged()
                {
                    ui.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });
    }
}
