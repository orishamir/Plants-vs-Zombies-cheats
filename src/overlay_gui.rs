use crate::game::GameProcess;
use eframe::NativeOptions;
use egui::{self, Color32, Frame, Pos2, Rgba, ViewportBuilder, ViewportCommand};

#[derive(Default)]
pub struct OverlayGui {
    pub popcapgame: GameProcess,
}

impl eframe::App for OverlayGui {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::from_rgba_premultiplied(0., 0., 0., 0.1).to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.3);
        egui::containers::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                // ui.painter().circle_filled(
                // Pos2::new(50., 500.),
                // 500.,
                // Color32::from_rgb(0, 255, 0),
                // );
            });
    }
}

impl OverlayGui {
    pub fn start(self) {
        let rect = self.popcapgame.get_rect_size();
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([width as f32, height as f32])
                .with_position([rect.left as f32, rect.top as f32])
                .with_always_on_top()
                .with_decorations(false)
                // .with_mouse_passthrough(true) DOES NOT WORK! Workaround: https://github.com/emilk/egui/issues/2537#issuecomment-2255399952
                .with_transparent(true),
            ..Default::default()
        };
        eframe::run_native(
            "My egui App",
            options,
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);
                cc.egui_ctx
                    .send_viewport_cmd(ViewportCommand::MousePassthrough(true));
                Ok(Box::new(self))
            }),
        )
        .expect("msg");
    }
}
