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

mod normalized_skewb;
mod skewb;
mod drawer;
mod unordered_pair;

use skewb::Skewb;
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
    let mut solved = Skewb::new();
    solved.turn_fb(&(0, 1, 1));
    solved.turn_fb(&(0, 0, 1));
    solved.turn_lr(&(0, 1, 1));
    solved.turn_lr(&(0, 0, 1));
    solved.rotate_ud();
    solved.rotate_ud();
    solved.turn_fb(&(0, 1, 1));
    solved.turn_fb(&(0, 0, 1));
    solved.turn_lr(&(0, 1, 1));
    solved.turn_lr(&(0, 0, 1));

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |mut c, g| {
                let gray = [0.99, 0.99, 0.99, 0.1];
                g.clear_color(gray);
                c.draw_state.blend = Some(graphics::draw_state::Blend::Multiply);
                drawer.draw(&solved, &c, g);
            });
        }
    }
}
