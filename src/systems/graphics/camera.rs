use cgmath::{Point2, Matrix4, ortho};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub viewport_size: f32,
    pub aspect_ratio: f32,
    pub center: Point2<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            viewport_size: 5.0,
            aspect_ratio: 1.0,
            center: Point2 { x: 0.0, y: 0.0 },
        }
    }
    
    pub fn matrix(&self) -> Matrix4<f32> {
        ortho(
            self.center.x - self.viewport_size * self.aspect_ratio,
            self.center.x + self.viewport_size * self.aspect_ratio,
            -self.center.y + self.viewport_size,
            -self.center.y - self.viewport_size,
            -1.0,
            1.0,
        )
    }
}
