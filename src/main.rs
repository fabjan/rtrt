use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const SALMON: [u8; 4] = [0xFA, 0x80, 0x72, 0xFF];
const SKYBLUE: [u8; 4] = [0x87, 0xCE, 0xEB, 0xFF];
const TERRACOTTA: [u8; 4] = [0xFF, 0xB3, 0x87, 0xFF];
const TEAL: [u8; 4] = [0x00, 0x87, 0x87, 0xFF];

const SPHERE_RADIUS: f64 = 1.5;

const MOVE_SPEED: f64 = 0.1;
const TURN_SPEED: f64 = 0.04;

fn signed_distance(p: &[f64; 3]) -> f64 {
    norm(p) - SPHERE_RADIUS
}

fn sphere_trace(orig: [f64; 3], dir: &[f64; 3]) -> bool {

    let mut p = orig;
    for _ in 0..10 {
        let d = signed_distance(&p);
        if d < 0.001 {
            return true;
        }
        p = plus(&p, &scale(dir, d));
    }

    false
}

struct Camera {
    eye: [f64; 3],
    look_at: [f64; 3],
    up: [f64; 3],
    fov: f64,
    // cached view vectors
    view_dir: [f64; 3],
    view_right: [f64; 3],
    view_up: [f64; 3],
}

fn new_camera(eye: [f64; 3], look_at: [f64; 3], up: [f64; 3], fov: f64) -> Camera {
    let mut cam = Camera {
        eye,
        look_at,
        up,
        fov,
        view_dir: [0.0; 3],
        view_right: [0.0; 3],
        view_up: [0.0; 3],
    };
    recalculate_view(&mut cam);
    cam
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

    let aspect = WIDTH as f64 / HEIGHT as f64;
    let right = scale(&right, aspect * cam.fov.tan());
    let up = scale(&up, cam.fov.tan());

    cam.view_dir = dir;
    cam.view_right = right;
    cam.view_up = up;
}

fn cast(cam: &Camera, x: f64, y: f64) -> [f64; 3] {
    let dir = cam.view_dir;
    let right = cam.view_right;
    let up = cam.view_up;

    let x = x / WIDTH as f64;
    let y = y / HEIGHT as f64;

    let x = scale(&right, x - 0.5);
    let y = scale(&up, y - 0.5);

    let dir = plus(&dir, &x);
    let dir = plus(&dir, &y);

    normalize(&dir)
}

struct Scene {
    camera: Camera,
}

fn draw(frame: &mut [u8], scene: &Scene) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH;
        let y = i / WIDTH;

        let eye = scene.camera.eye;
        let dir = cast(&scene.camera, x as f64, y as f64);

        let fg = if y < HEIGHT / 2 { TERRACOTTA } else { SALMON };
        let bg = if y < HEIGHT / 2 { SKYBLUE } else { TEAL };

        let rgba = if sphere_trace(eye, &dir) {
            fg
        } else {
            bg
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn reset_camera() -> Camera {
    new_camera(
        [0.0, 0.0, 3.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        std::f64::consts::PI / 3.0,
    )
}

fn run(
    event_loop: EventLoop<()>,
    mut input: WinitInputHelper,
    window: Window,
    mut pixels: Pixels,
) -> Result<(), Error> {

    let mut scene = Scene {
        camera: reset_camera(),
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame_mut(), &scene);
            if let Err(err) = pixels.render() {
                eprintln!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::R) {
                scene.camera = reset_camera();
            }

            if input.key_held(VirtualKeyCode::W) {
                zoom(&mut scene.camera, MOVE_SPEED);
            }
            if input.key_held(VirtualKeyCode::S) {
                zoom(&mut scene.camera, -MOVE_SPEED);
            }
            if input.key_held(VirtualKeyCode::A) {
                pan(&mut scene.camera, -MOVE_SPEED, 0.0);
            }
            if input.key_held(VirtualKeyCode::D) {
                pan(&mut scene.camera, MOVE_SPEED, 0.0);
            }
            if input.key_held(VirtualKeyCode::Left) {
                rotate(&mut scene.camera, -TURN_SPEED, 0.0);
            }
            if input.key_held(VirtualKeyCode::Right) {
                rotate(&mut scene.camera, TURN_SPEED, 0.0);
            }
            if input.key_held(VirtualKeyCode::Space) {
                pan(&mut scene.camera, 0.0, -MOVE_SPEED);
            }
            if input.key_held(VirtualKeyCode::LShift) {
                pan(&mut scene.camera, 0.0, MOVE_SPEED);
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            window.request_redraw();
        }
    });
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    run(event_loop, input, window, pixels)
}


fn norm(v: &[f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn minus(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

fn plus(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

fn scale(v: &[f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn normalize(v: &[f64; 3]) -> [f64; 3] {
    let n = norm(v);
    [v[0] / n, v[1] / n, v[2] / n]
}

fn cross(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}
