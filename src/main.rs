use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use rtrt::camera::{handle_input, new_camera, raycast, Camera};
use rtrt::math::*;

const WINDOW_TITLE: &str = "Hello Pixels";
const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const SALMON: [u8; 4] = [0xFA, 0x80, 0x72, 0xFF];
const SKYBLUE: [u8; 4] = [0x87, 0xCE, 0xEB, 0xFF];
const TERRACOTTA: [u8; 4] = [0xFF, 0xB3, 0x87, 0xFF];
const TEAL: [u8; 4] = [0x00, 0x87, 0x87, 0xFF];

const AMBIENT_LIGHT: f64 = 0.4;
const SPHERE_RADIUS: f64 = 1.5;

fn signed_distance(p: &[f64; 3]) -> f64 {
    norm(p) - SPHERE_RADIUS
}

fn distance_field_normal(p: &[f64; 3], sdf: fn(&[f64; 3]) -> f64) -> [f64; 3] {
    let eps = 0.01;
    let d = sdf(p);
    let x = sdf(&plus(p, &[eps, 0.0, 0.0]));
    let y = sdf(&plus(p, &[0.0, eps, 0.0]));
    let z = sdf(&plus(p, &[0.0, 0.0, eps]));
    let n = [x - d, y - d, z - d];
    normalize(&n)
}

fn sphere_trace(orig: [f64; 3], dir: &[f64; 3]) -> Option<[f64; 3]> {
    let mut p = orig;
    for _ in 0..10 {
        let d = signed_distance(&p);
        if d < 0.001 {
            return Some(p);
        }
        p = plus(&p, &scale(dir, d));
    }

    None
}

struct Scene {
    camera: Camera,
}

fn draw(frame: &mut [u8], scene: &Scene) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH;
        let y = i / WIDTH;

        let (eye, dir) = raycast(&scene.camera, x as f64, y as f64);

        let fg = if y < HEIGHT / 2 { TERRACOTTA } else { SALMON };
        let bg = if y < HEIGHT / 2 { SKYBLUE } else { TEAL };

        let rgba = match sphere_trace(eye, &dir) {
            Some(hit) => {
                let n = distance_field_normal(&hit, signed_distance);
                let light_dir = normalize(&[0.5, -1.0, 0.5]);
                let light_intensity = 1.0;
                let intensity = dot(&n, &light_dir) * light_intensity;
                let intensity = intensity.max(AMBIENT_LIGHT);
                color_multiply(&fg, intensity)
            }
            None => bg,
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
        WIDTH,
        HEIGHT,
    )
}

fn run(
    event_loop: EventLoop<()>,
    mut input: WinitInputHelper,
    window: Window,
    mut pixels: Pixels,
) -> Result<(), Error> {
    let mut frame_count = 0;
    let mut last_frame = Instant::now();

    let mut scene = Scene {
        camera: reset_camera(),
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            frame_count += 1;

            if 1.0 <= last_frame.elapsed().as_secs_f64() {
                let fps = frame_count as f64 / last_frame.elapsed().as_secs_f64();
                window.set_title(&format!("{WINDOW_TITLE} ({fps:.1} FPS)"));
                frame_count = 0;
                last_frame = Instant::now();
            }

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

            handle_input(&mut scene.camera, &input);

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
            .with_title(WINDOW_TITLE)
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
