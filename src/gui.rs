use crate::game::GameProcess;
use crate::toggleables::{
    FastChomperCheat, FreePlantsCheat, InstantRechargeCheat, InvinciblePlantsCheat, NoPauseCheat,
    PlantAnywhereCheat, SeethroughVasesCheat, Toggleable,
};
use egui::{self, AtomExt, Rgba, Ui};

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
            Box::new(FastChomperCheat {}),
            Box::new(NoPauseCheat {}),
            Box::new(SeethroughVasesCheat {}),
        ];

        Self {
            toggled: vec![false; toggleables.len()],
            toggleables,
        }
    }
}

pub struct MyApp {
    cheats: Cheats,
    popcapgame: GameProcess,
}

impl MyApp {
    fn render_toggleables(&mut self, ui: &mut Ui) {
        for (toggleable, toggled) in self
            .cheats
            .toggleables
            .iter_mut()
            .zip(&mut self.cheats.toggled)
        {
            if ui
                .checkbox(toggled, toggleable.name().atom_grow(true))
                .changed()
            {
                toggleable
                    .toggle(&self.popcapgame, *toggled)
                    .expect("What the actual sigma");
                println!("Detected change in {} to {toggled}", toggleable.name(),);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::from_rgba_premultiplied(0., 0., 0., 0.5).to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.3);
        egui::Window::new("Toggleables").show(ctx, |ui| self.render_toggleables(ui));
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let popcapgame = GameProcess::default();

        let cheats: Cheats = Default::default();

        Self { cheats, popcapgame }
    }
}
