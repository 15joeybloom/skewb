extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

use graphics::Graphics;

mod skewb;
mod drawer;
mod unordered_pair;

use skewb::Skewb;
use skewb::NormalizedSkewb;
use skewb::Color;
use skewb::Orientation;
use drawer::Drawer;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Skewb", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let drawer = Drawer::new();

    let mut scrambled = NormalizedSkewb {
        center_pieces: [Color::Y, Color::G, Color::R, Color::O, Color::B, Color::W],
        fixed_orientations: [Orientation::UD, Orientation::LR, Orientation::FB, Orientation::FB],
        moving_orientations: [Orientation::LR, Orientation::LR, Orientation::UD, Orientation::LR],
        moving_pieces: [1, 3, 2, 0],
    };

    if let Some(solution) = scrambled.solution() {
        for move_ in solution.iter() {
            println!("{:?}", move_);
        }
    }
    /*
    Move { direction: FB, corner: (1, 1, 0) }
    Move { direction: LR, corner: (0, 0, 0) }
    Move { direction: LR, corner: (1, 0, 1) }
    Move { direction: LR, corner: (0, 0, 0) }
    Move { direction: FB, corner: (0, 1, 1) }
    Move { direction: LR, corner: (1, 1, 0) }
    Move { direction: LR, corner: (0, 0, 0) }
    Move { direction: LR, corner: (1, 1, 0) }
     */

    let solved = Skewb::new();
    let mut normalized = solved.normalize();
    normalized.turn_fb((0, 1, 1));
    normalized.turn_lr((1, 0, 1));
    let denormalized = normalized.denormalize();

    let draw_me = scrambled.denormalize();

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
