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

fn pattern(x: usize, y: usize) -> f64 {
    let x = x as f64;
    let y = y as f64;

    let a = 0.5 * (x * x + y * y).sqrt();
    let b = 0.5 * (x * x - y * y).sqrt();
    let c = 0.5 * (x * x + y * y).sin();
    let d = 0.5 * (x * x - y * y).cos();

    a * b * c * d
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH;
        let y = i / WIDTH;

        let rgba = match pattern(x, y) {
            x if x < -1000. => SALMON,
            x if x < 0. => SKYBLUE,
            x if x < 1000. => TERRACOTTA,
            _ => TEAL,
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn run(
    event_loop: EventLoop<()>,
    mut input: WinitInputHelper,
    window: Window,
    mut pixels: Pixels,
) -> Result<(), Error> {
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame_mut());
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
