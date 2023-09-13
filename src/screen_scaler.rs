use macroquad::prelude::*;

pub struct ScreenScaler<T>
where
    T: ScreenScalerDraw,
{
    pub screen_w: f32,
    pub screen_h: f32,
    pub scene: T,
}

impl<T> ScreenScaler<T>
where
    T: ScreenScalerDraw,
{
    pub fn screen_scale(&self, screen_w: f32, screen_h: f32) -> f32 {
        (screen_w / self.screen_w).min(screen_h / self.screen_h)
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = self.screen_scale(screen_w, screen_h);
        let x_offset = (screen_w - self.screen_w * scale) * 0.5;
        let y_offset = (screen_h - self.screen_h * scale) * 0.5;

        self.scene.draw(
            x_offset,
            y_offset,
            self.screen_w * scale,
            self.screen_h * scale,
            scale,
        );
    }
}

pub trait ScreenScalerDraw {
    fn draw(&self, x_offset: f32, y_offset: f32, width: f32, height: f32, scale: f32);
}
