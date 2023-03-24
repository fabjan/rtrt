use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use rtrt::camera::{handle_input, new_camera, Camera};
use rtrt::scene::{self, Shape};

const WINDOW_TITLE: &str = "Hello Pixels";
const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const SALMON: [u8; 4] = [0xFA, 0x80, 0x72, 0xFF];
const TERRACOTTA: [u8; 4] = [0xFF, 0xB3, 0x87, 0xFF];
const TEAL: [u8; 4] = [0x00, 0x87, 0x87, 0xFF];
const GRAY: [u8; 4] = [0x87, 0x87, 0x87, 0xFF];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

fn reset_camera() -> Camera {
    new_camera(
        [0.0, 0.0, 9.0],
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

    let mut scene = scene::new_scene(reset_camera());

    scene.add_shape(
        0,
        Shape::Sphere {
            center: [0.0, 1.0, 0.0],
            radius: 0.7,
            color: TERRACOTTA,
        },
    );
    scene.add_shape(
        1,
        Shape::Sphere {
            center: [1.0, 1.0, 0.0],
            radius: 1.1,
            color: SALMON,
        },
    );
    scene.add_shape(
        2,
        Shape::Sphere {
            center: [0.0, 3.0, 1.0],
            radius: 2.0,
            color: TEAL,
        },
    );
    scene.add_shape(
        3,
        Shape::Box {
            center: [0.0, 0.0, -5.0],
            size: [10.0, 5.0, 0.1],
            color: GRAY,
        },
    );
    scene.add_shape(
        4,
        Shape::Box {
            center: [-5.0, 0.0, 0.0],
            size: [0.1, 5.0, 10.0],
            color: GRAY,
        },
    );
    scene.add_shape(
        5,
        Shape::Box {
            center: [5.0, 0.0, 0.0],
            size: [0.1, 5.0, 10.0],
            color: GRAY,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            frame_count += 1;

            let t = last_frame.elapsed().as_secs_f64();
            if 1.0 <= t {
                let fps = frame_count as f64 / t;
                window.set_title(&format!("{WINDOW_TITLE} ({fps:.1} FPS)"));
                frame_count = 0;
                last_frame = Instant::now();
            }

            let sin = (t * std::f64::consts::PI * 2.0).sin();
            let cos = (t * std::f64::consts::PI * 2.0).cos();

            scene.move_shape(0, sin * 5.0, 0.0, cos * 5.0);
            scene.move_shape(1, cos * 1.5, sin * 2.5, 0.0);
            scene.move_shape(2, 0.0, sin * 0.5, cos * 0.5);

            scene.draw(pixels.get_frame_mut(), WIDTH, BLACK);

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
