pub mod screen_scaler;

pub mod macroquad {
    pub use ::macroquad::*;
}

pub mod macroquad_text {
    pub use ::macroquad_text::*;
}

use macroquad::prelude::*;
use macroquad_text::Fonts;

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
