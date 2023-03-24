use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::math::*;

const MOVE_SPEED: f64 = 0.1;
const TURN_SPEED: f64 = 0.02;

pub struct Camera {
    eye: [f64; 3],
    look_at: [f64; 3],
    up: [f64; 3],
    fov: f64,
    raster_width: f64,
    raster_height: f64,
    // cached view vectors
    view_dir: [f64; 3],
    view_right: [f64; 3],
    view_up: [f64; 3],
}

pub fn new_camera(
    eye: [f64; 3],
    look_at: [f64; 3],
    up: [f64; 3],
    fov: f64,
    w: usize,
    h: usize,
) -> Camera {
    let mut cam = Camera {
        eye,
        look_at,
        up,
        fov,
        raster_width: w as f64,
        raster_height: h as f64,
        view_dir: [0.0; 3],
        view_right: [0.0; 3],
        view_up: [0.0; 3],
    };
    recalculate_view(&mut cam);
    cam
}

pub fn raycast(cam: &Camera, x: f64, y: f64) -> ([f64; 3], [f64; 3]) {
    let dir = cam.view_dir;
    let right = cam.view_right;
    let up = cam.view_up;

    let x = x / cam.raster_width;
    let y = y / cam.raster_height;

    let x = scale(&right, x - 0.5);
    let y = scale(&up, y - 0.5);

    let dir = plus(&dir, &x);
    let dir = plus(&dir, &y);
    let dir = normalize(&dir);

    (cam.eye, dir)
}

pub fn handle_input(cam: &mut Camera, input: &WinitInputHelper) {
    if input.key_held(VirtualKeyCode::W) {
        zoom(cam, MOVE_SPEED);
    }
    if input.key_held(VirtualKeyCode::S) {
        zoom(cam, -MOVE_SPEED);
    }
    if input.key_held(VirtualKeyCode::A) {
        pan(cam, -MOVE_SPEED, 0.0);
    }
    if input.key_held(VirtualKeyCode::D) {
        pan(cam, MOVE_SPEED, 0.0);
    }
    if input.key_held(VirtualKeyCode::Left) {
        rotate(cam, -TURN_SPEED, 0.0);
    }
    if input.key_held(VirtualKeyCode::Right) {
        rotate(cam, TURN_SPEED, 0.0);
    }
    if input.key_held(VirtualKeyCode::Space) {
        pan(cam, 0.0, -MOVE_SPEED);
    }
    if input.key_held(VirtualKeyCode::LShift) {
        pan(cam, 0.0, MOVE_SPEED);
    }
}

fn pan(cam: &mut Camera, dx: f64, dy: f64) {
    let right = cam.view_right;
    let up = cam.view_up;

    let right = scale(&right, dx);
    let up = scale(&up, dy);

    cam.eye = plus(&cam.eye, &right);
    cam.eye = plus(&cam.eye, &up);
    cam.look_at = plus(&cam.look_at, &right);
    cam.look_at = plus(&cam.look_at, &up);

    recalculate_view(cam);
}

fn zoom(cam: &mut Camera, dz: f64) {
    let dir = cam.view_dir;

    let dir = scale(&dir, dz);

    cam.eye = plus(&cam.eye, &dir);
    cam.look_at = plus(&cam.look_at, &dir);

    recalculate_view(cam);
}

fn rotate(cam: &mut Camera, dx: f64, dy: f64) {
    let dir = cam.view_dir;
    let right = cam.view_right;
    let up = cam.view_up;

    let right = scale(&right, dx);
    let up = scale(&up, dy);

    cam.look_at = plus(&cam.eye, &dir);
    cam.look_at = plus(&cam.look_at, &right);
    cam.look_at = plus(&cam.look_at, &up);

    recalculate_view(cam);
}

fn recalculate_view(cam: &mut Camera) {
    let eye = cam.eye;
    let look_at = cam.look_at;
    let up = cam.up;

    let dir = minus(&look_at, &eye);
    let dir = normalize(&dir);

    let right = normalize(&cross(&dir, &up));
    let up = normalize(&cross(&right, &dir));

    let aspect = cam.raster_width / cam.raster_height;
    let right = scale(&right, aspect * cam.fov.tan());
    let up = scale(&up, cam.fov.tan());

    cam.view_dir = dir;
    cam.view_right = right;
    cam.view_up = up;
}
