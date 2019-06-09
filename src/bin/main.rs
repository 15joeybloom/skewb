extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate skewb;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use graphics::Graphics;

use skewb::drawer::Drawer;
use skewb::skewb::Color;
use skewb::skewb::NormalizedSkewb;
use skewb::skewb::Orientation;

fn main() {
    // A skewb!
    let mut scrambled = NormalizedSkewb {
        center_pieces: [Color::Y, Color::G, Color::R, Color::O, Color::B, Color::W],
        fixed_orientations: [
            Orientation::UD,
            Orientation::LR,
            Orientation::FB,
            Orientation::FB,
        ],
        floating_orientations: [
            Orientation::LR,
            Orientation::LR,
            Orientation::UD,
            Orientation::LR,
        ],
        floating_pieces: [1, 3, 2, 0],
    };

    // Solve it!
    println!("Solving. This might take a minute.");
    if let Some(solution) = scrambled.solution() {
        println!("Found a solution:");
        for move_ in solution.iter() {
            println!("{:?}", move_);
        }
    } else {
        println!("No solution found.");
    }

    // Draw it!
    let draw_me = scrambled.denormalize();

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Skewb", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let drawer = Drawer::new();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |mut c, g| {
                let gray = [0.99, 0.99, 0.99, 0.1];
                g.clear_color(gray);
                c.draw_state.blend = Some(graphics::draw_state::Blend::Multiply);
                drawer.draw(&draw_me, &c, g);
            });
        }
    }
}
