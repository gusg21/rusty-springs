use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::{Canvas, Drawable, DrawParam, GraphicsContext, Rect, Transform};

pub struct Actor {
    pos: Vec2,
    size: Vec2,
    sprite: graphics::Image,
}

impl Drawable for Actor {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        canvas.draw(self.sprite, DrawParam {
            
        });
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        return Option::Some(graphics::Rect { x: self.pos.x, y: self.pos.y, w: self.size.x, h: self.size.y });
    }
}