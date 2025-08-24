#![allow(dead_code, unused)]
use crate::game::Popcapgame;
use egui::{self, Frame, Id, ViewportBuilder, ViewportCommand};

#[derive(Default)]
pub struct OverlayGui {
    pub popcapgame: Popcapgame,
}

impl OverlayGui {
    pub fn start(self, ctx: &egui::Context) {
        let rect = self.popcapgame.get_rect_size();
        let width = (rect.right - rect.left) as f32;
        let height = (rect.bottom - rect.top) as f32;

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            ViewportBuilder::default()
                .with_inner_size([width, height])
                .with_position([rect.left as f32, rect.top as f32])
                .with_always_on_top()
                .with_decorations(false),
            // .with_mouse_passthrough(true) DOES NOT WORK! Workaround: https://github.com/emilk/egui/issues/2537#issuecomment-2255399952
            |ctx, class| {
                ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));
                ctx.send_viewport_cmd(ViewportCommand::Transparent(true));
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show(ctx, |ui| {
                        ui.label("hey!");
                    });
            },
        );
    }
}
