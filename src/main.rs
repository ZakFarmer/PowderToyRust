#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod particle_sim;

pub use crate::particle_sim::particle::Particle;
pub use crate::particle_sim::world::World;

use log::error;
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const HEIGHT: u32 = 400;
const WIDTH: u32 = 400;

const DAMPING: f32 = 1.00004;
const TIMESCALE: f32 = 0.02;

const LIGHT_PINK: [u8; 4] = [0xf2, 0x93, 0xb1, 0xff];
const PINK: [u8; 4] = [0xed, 0x51, 0x81, 0xff];
const RED: [u8; 4] = [0xe8, 0x2c, 0x45, 0xff];
const BLUE: [u8; 4] = [0x34, 0x56, 0x9d, 0xff];
const YELLOW: [u8; 4] = [0xff, 0xf9, 0x75, 0xff];
const DARK_YELLOW: [u8; 4] = [0xff, 0xea, 0x70, 0xff];
const ORANGE: [u8; 4] = [0xf8, 0xdb, 0x81, 0xff];

const COLORS: [[u8; 4]; 7] = [LIGHT_PINK, PINK, RED, BLUE, YELLOW, DARK_YELLOW, ORANGE];

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new(); // Why is this mutable?

    let window = {
        let size = LogicalSize::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);

        WindowBuilder::new()
            .with_title("Powder Toy in Rust")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    //pixels.resize_buffer(WIDTH / 2, HEIGHT / 2);

    let mut world = World::new();
    let mut paused = false;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // Clear the pixel buffer
            let frame = pixels.get_frame();
            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 0x29; // R
                pixel[1] = 0x24; // G
                pixel[2] = 0x2b; // B
                pixel[3] = 0xff; // A
            }

            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.mouse_held(1) {
                let (mouse_x, mouse_y) = input.mouse().expect("Couldn't get mouse position!");

                //println!("Mouse X: {}, Mouse Y: {}", mouse_x, mouse_y);

                world.add_particle(mouse_x, mouse_y);
            }

            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                paused = !paused;
            }
        }
        world.update();
        window.request_redraw();
    });
}
