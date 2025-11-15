use egui::{self, Color32, Frame, RichText, Ui};
use log::info;
use pvz_sdk::Popcapgame;
use pvz_sdk::toggleables::{self, Toggleable};

pub struct MyApp {
    toggleables: Vec<Box<dyn Toggleable>>,
    toggled: Vec<bool>,
    popcapgame: Popcapgame,
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

impl MyApp {
    fn render_toggleables(&mut self, ui: &mut Ui) {
        egui::Grid::new("id_salt").striped(true).show(ui, |ui| {
            for (toggleable, toggled) in self.toggleables.iter().zip(&mut self.toggled) {
                if ui
                    .checkbox(toggled, togglecheat_font(toggleable.name()))
                    .changed()
                {
                    toggleable.toggle(&self.popcapgame, *toggled).unwrap();
                    info!("Detected change in {} to {toggled}", toggleable.name());
                }
                ui.end_row();
            }
        });
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.4]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new(toggleables_header_font())
            .auto_sized()
            .show(ctx, |ui| self.render_toggleables(ui));

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                if ui
                    .interact(
                        ui.max_rect(),
                        egui::Id::new("window-drag"),
                        egui::Sense::drag(),
                    )
                    .dragged()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let popcapgame = Popcapgame::init().unwrap();

        let toggleables: Vec<Box<dyn Toggleable>> = vec![
            Box::new(toggleables::AutoPickupSunCheat {}),
            Box::new(toggleables::InstantRechargeCheat {}),
            Box::new(toggleables::FreePlantsCheat {}),
            Box::new(toggleables::InvinciblePlantsCheat {}),
            Box::new(toggleables::SeethroughVasesCheat {}),
            Box::new(toggleables::HeadshotMode {}),
            Box::new(toggleables::HungryCompersCheat {}),
            // Box::new(toggleables::PlantAnywhereCheat {}),
            // Box::new(toggleables::NoPauseCheat {}),
        ];

        let toggled: Vec<bool> = toggleables
            .iter()
            .map(|toggleable| toggleable.is_activated(&popcapgame).unwrap())
            .collect();

        Self {
            toggled,
            toggleables,
            popcapgame,
        }
    }
}
