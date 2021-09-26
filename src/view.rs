use crate::{Color, Hittable, Ray, Vector};

/// A camera in a 3D world.
///
/// The focal point is always located at the origin, and the camera is always
/// pointing forward in the y+ direction, with z+ above and x+ to the right.
pub(crate) struct Camera {
    pub(crate) focal_len: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Camera {
    pub(crate) fn project(&self, scene: impl Hittable, u: f64, v: f64) -> Option<Color> {
        let incidence = Ray {
            origin: Vector::ZERO,
            direction: Vector(
                -self.width / 2.0 + u * self.width,
                self.focal_len,
                -self.height / 2.0 + v * self.height,
            ),
        };
        scene.hit(incidence).map(|hit| hit.color)
    }
}

pub(crate) struct Viewport {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) camera: Camera,
}

impl Viewport {
    pub(crate) fn render(&self, scene: impl Hittable) {
        for j in (0..self.height).rev() {
            let v = (j as f64 + 0.5) / self.height as f64;
            for i in 0..self.width {
                let u = (i as f64 + 0.5) / self.width as f64;
                match self.camera.project(&scene, u, v) {
                    Some(color) => print!("{}█", color),
                    None => print!(" "),
                }
            }
            println!()
        }
        print!("{}", Color::Reset);
    }
}
