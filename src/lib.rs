mod screen_scaler;

use macroquad::prelude::*;
pub use macroquad::*;
use macroquad_text::Fonts;
pub use macroquad_text::*;

pub trait DrawScaledText {
    fn draw_scaled_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        size: u16,
        scale: f32,
        color: Color,
    ) -> TextDimensions;
}

impl DrawScaledText for Fonts<'_> {
    fn draw_scaled_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        size: u16,
        scale: f32,
        color: Color,
    ) -> TextDimensions {
        self.draw_text_ex(
            text,
            &macroquad_text::TextParams {
                x,
                y,
                size: size as f32,
                scale,
                color,
                ..Default::default()
            },
        )
    }
}
