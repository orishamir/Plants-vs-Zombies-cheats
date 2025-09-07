use egui::{self, Color32, Frame, RichText, Ui};
use log::info;
use pvz_sdk::game::Popcapgame;
use pvz_sdk::toggleables::{
    FastChomperCheat, FreePlantsCheat, InstantRechargeCheat, InvinciblePlantsCheat, NoPauseCheat,
    PlantAnywhereCheat, SeethroughVasesCheat, Toggleable,
};

pub struct MyApp {
    toggleables: Vec<Box<dyn Toggleable>>,
    toggled: Vec<bool>,
    popcapgame: Popcapgame,
}

#[inline]
fn toggleables_header_font() -> RichText {
    RichText::new("Toggleables")
        .size(30.)
        .color(Color32::MAGENTA)
}

#[inline]
fn togglecheat_font(name: &'static str) -> RichText {
    RichText::new(name).size(24.).strong()
}

impl MyApp {
    fn render_toggleables(&mut self, ui: &mut Ui) {
        egui::Grid::new("id_salt").striped(true).show(ui, |ui| {
            for (toggleable, toggled) in self.toggleables.iter_mut().zip(&mut self.toggled) {
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
            Box::new(InvinciblePlantsCheat {}),
            Box::new(PlantAnywhereCheat {}),
            Box::new(FreePlantsCheat {}),
            Box::new(InstantRechargeCheat {}),
            Box::new(FastChomperCheat {}),
            Box::new(NoPauseCheat {}),
            Box::new(SeethroughVasesCheat {}),
        ];

        Self {
            toggled: vec![false; toggleables.len()],
            toggleables,
            popcapgame,
        }
    }
}
