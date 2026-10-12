use egui::{self, Checkbox, Color32, Frame, RichText, Ui};
use log::{info, warn};
use pvz_sdk::Popcapgame;
use pvz_sdk::toggleables::{self, ToggleCheatError, Toggleable};

pub struct MyApp {
    cheats: Vec<CheatRow>,
    popcapgame: Popcapgame,
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

impl MyApp {
    fn render_toggleables(&mut self, ui: &mut Ui) {
        egui::Grid::new("id_salt").striped(true).show(ui, |ui| {
            for cheat in &mut self.cheats {
                cheat.show(ui, &self.popcapgame);
                ui.end_row();
            }
        });
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.4]
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Window::new(toggleables_header_font())
            .auto_sized()
            .show(ui, |ui| self.render_toggleables(ui));

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

impl Default for MyApp {
    fn default() -> Self {
        let popcapgame = Popcapgame::init().unwrap();

        let cheats = toggleables::ALL
            .iter()
            .map(|cheat| CheatRow::new(cheat, &popcapgame))
            .collect();

        Self { cheats, popcapgame }
    }
}
