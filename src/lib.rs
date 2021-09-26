#![allow(clippy::many_single_char_names)]

//! # Shapes
//!
//! Simple 3D objects for your terminal.

mod color;
mod cube;
mod easing;
mod hit;
mod matrix;
mod ray;
mod torus;
mod transform;
mod translate;
mod util;
mod vector;
mod view;

use std::{f64::consts::TAU, time::Instant};

use self::{
    color::Color,
    cube::cube,
    easing::ease_sin_in_out,
    hit::{Hit, Hittable, HittableExt},
    matrix::Matrix,
    ray::Ray,
    torus::Torus,
    transform::{Transformation, TransformationExt},
    translate::Translation,
    vector::Vector,
    view::{Camera, Viewport},
};

/// Runs the program.
pub fn run() {
    let viewport = Viewport {
        camera: Camera {
            focal_len: 0.5,
            width: 1.0,
            height: 1.0,
        },
        width: 80,
        height: 40,
    };

    let start = Instant::now();
    let phase = |dur| (start.elapsed().as_millis() % dur) as f64 / dur as f64;

    println!("\x1b[2J");

    loop {
        println!("\x1b[H");

        let cube = cube()
            .transform(Matrix::scale(Vector(1.3, 1.3, 1.3)))
            .transform(Matrix::rotation_z(ease_sin_in_out(phase(2500)) * TAU))
            .transform(Matrix::rotation_x(0.7));

        let torus = Torus {
            radius_major: 1.5,
            radius_minor: 0.3,
        }
        .transform(Matrix::rotation_x(phase(6000) * TAU))
        .transform(Matrix::rotation_y(phase(29000) * TAU))
        .transform(Matrix::rotation_z(-phase(14000) * TAU));

        let scene = cube
            .and(torus)
            .transform(Translation(Vector(0.0, 3.0, 0.0)));

        viewport.render(scene);
    }
}
