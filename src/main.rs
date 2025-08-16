#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, Rgba, Ui, ViewportBuilder, ViewportCommand};
use game::GameProcess;
use toggleables::{
    FastChomperCheat, FreePlantsCheat, InstantRechargeCheat, InvinciblePlantsCheat,
    PlantAnywhereCheat,
};
use toggleables::{InfiniteLawnmowersCheat, Toggleable};

mod game;
mod toggleables;

struct Cheats {
    toggleables: Vec<Box<dyn Toggleable>>,
    toggled: Vec<bool>,
}

impl Default for Cheats {
    fn default() -> Self {
        let toggleables: Vec<Box<dyn Toggleable>> = vec![
            Box::new(InvinciblePlantsCheat {}),
            Box::new(PlantAnywhereCheat {}),
            Box::new(FreePlantsCheat {}),
            Box::new(InstantRechargeCheat {}),
            Box::new(InfiniteLawnmowersCheat {}),
            Box::new(FastChomperCheat {}),
        ];
        Self {
            toggled: vec![false; toggleables.len()],
            toggleables,
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_always_on_top(),
        // .with_decorations(false),
        // .with_fullscreen(true)
        // .with_mouse_passthrough(true) DOES NOT WORK! Workaround: https://github.com/emilk/egui/issues/2537#issuecomment-2255399952
        // .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // cc.egui_ctx
            // .send_viewport_cmd(ViewportCommand::MousePassthrough(true));
            let thing = Box::<MyApp>::default();
            Ok(thing)
        }),
    )
}

struct MyApp {
    cheats: Cheats,
    popcapgame: GameProcess,
}

impl MyApp {
    fn render_toggleables(&mut self, ui: &mut Ui) {
        for (toggleable, mut toggled) in self
            .cheats
            .toggleables
            .iter_mut()
            .zip(&mut self.cheats.toggled)
        {
            if ui.checkbox(&mut toggled, toggleable.get_name()).changed() {
                toggleable
                    .toggle(&self.popcapgame, *toggled)
                    .expect("What the actual sigma");
                println!(
                    "Detected change in {} to {}",
                    toggleable.get_name(),
                    toggled
                );
            }
        }
    }
}

impl eframe::App for MyApp {
    // fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    // Rgba::TRANSPARENT.to_array()
    // }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_toggleables(ui);

            ()
        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let popcapgame = GameProcess::default();

        let cheats: Cheats = Default::default();
        Self {
            cheats: cheats,
            popcapgame: popcapgame,
        }
    }
}
