use std::collections::HashMap;

use crate::camera::{raycast, Camera};
use crate::math::*;

const TRACE_DEPTH: usize = 20;
const AMBIENT_LIGHT: f64 = 0.4;

pub enum Shape {
    Sphere {
        center: [f64; 3],
        radius: f64,
        color: [u8; 4],
    },
    Box {
        center: [f64; 3],
        size: [f64; 3],
        color: [u8; 4],
    },
}

fn shape_color(shape: &Shape) -> [u8; 4] {
    match shape {
        Shape::Sphere { color, .. } => *color,
        Shape::Box { color, .. } => *color,
    }
}

fn move_shape(shape: &mut Shape, x: f64, y: f64, z: f64) {
    match shape {
        Shape::Sphere { center, .. } => {
            center[0] = x;
            center[1] = y;
            center[2] = z;
        }
        Shape::Box { center, .. } => {
            center[0] = x;
            center[1] = y;
            center[2] = z;
        }
    }
}

fn distance_to_shape(shape: &Shape, p: &[f64; 3]) -> f64 {
    match shape {
        Shape::Sphere { center, radius, .. } => {
            let d = minus(p, center);
            norm(&d) - radius
        }
        Shape::Box { center, size, .. } => {
            let d = minus(p, center);
            let d = [
                d[0].abs() - size[0] / 2.0,
                d[1].abs() - size[1] / 2.0,
                d[2].abs() - size[2] / 2.0,
            ];
            let d = [d[0].max(0.0), d[1].max(0.0), d[2].max(0.0)];
            norm(&d)
        }
    }
}

fn shape_normal(shape: &Shape, p: &[f64; 3]) -> [f64; 3] {
    let eps = 0.01;
    let d = distance_to_shape(shape, p);
    let x = distance_to_shape(shape, &plus(p, &[eps, 0.0, 0.0]));
    let y = distance_to_shape(shape, &plus(p, &[0.0, eps, 0.0]));
    let z = distance_to_shape(shape, &plus(p, &[0.0, 0.0, eps]));
    let n = [x - d, y - d, z - d];
    normalize(&n)
}

fn sphere_trace<'a>(
    orig: [f64; 3],
    dir: &'a [f64; 3],
    shapes: &'a HashMap<usize, Shape>,
) -> Option<(&'a Shape, [f64; 3])> {
    let mut p = orig;

    for _ in 0..TRACE_DEPTH {
        let mut min_d = std::f64::INFINITY;
        let mut min_shape = None;
        for shape in shapes.values() {
            let d = distance_to_shape(shape, &p);
            if d < min_d {
                min_d = d;
                min_shape = Some(shape);
            }
        }

        if min_d < 0.01 {
            return Some((min_shape.unwrap(), p));
        }

        p = plus(&p, &scale(dir, min_d));
    }

    None
}

pub struct Scene {
    pub camera: Camera,
    shapes: HashMap<usize, Shape>,
}

pub fn new_scene(camera: Camera) -> Scene {
    Scene {
        camera,
        shapes: HashMap::new(),
    }
}

impl Scene {
    pub fn add_shape(&mut self, id: usize, shape: Shape) {
        self.shapes.insert(id, shape);
    }

    pub fn move_shape(&mut self, shape_id: usize, x: f64, y: f64, z: f64) {
        if let Some(shape) = self.shapes.get_mut(&shape_id) {
            move_shape(shape, x, y, z);
        }
    }

    pub fn draw(&self, frame: &mut [u8], width: usize, bg: [u8; 4]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % width;
            let y = i / width;

            let (eye, dir) = raycast(&self.camera, x as f64, y as f64);

            let rgba = match sphere_trace(eye, &dir, &self.shapes) {
                Some((shape, hit)) => {
                    let n = shape_normal(shape, &hit);
                    let light_dir = normalize(&[0.5, -1.0, 0.5]);
                    let light_intensity = 1.0;
                    let intensity = dot(&n, &light_dir) * light_intensity;
                    let intensity = intensity.max(AMBIENT_LIGHT);
                    let fg = shape_color(shape);
                    color_multiply(&fg, intensity)
                }
                None => bg,
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
