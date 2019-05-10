extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::line::Line;
use graphics::math::Vec2d;
use graphics::polygon::Polygon;
use graphics::{Context, Graphics};

use std::collections::HashMap;

use skewb;
use skewb::Center;
use skewb::Corner;
use skewb::Orientation;
use skewb::Skewb;

use unordered_pair;

impl skewb::Color {
    pub fn rgba(self) -> graphics::types::Color {
        match self {
            skewb::Color::B => [0.0, 0.0, 1.0, 1.0],
            skewb::Color::G => [0.0, 0.8, 0.0, 1.0],
            skewb::Color::R => [1.0, 0.1, 0.1, 1.0],
            skewb::Color::O => [1.0, 0.8, 0.0, 1.0],
            skewb::Color::Y => [1.0, 1.0, 0.0, 1.0],
            skewb::Color::W => [1.0, 1.0, 1.0, 1.0],
        }
    }
}

type Edge = unordered_pair::UnorderedPair<Corner>;

pub struct Drawer {
    corner_points: HashMap<Corner, Vec2d<f64>>,
    edge_points: HashMap<Edge, Vec2d<f64>>,
    corner_stickers: HashMap<(Corner, Orientation), (Corner, Corner)>,
}
impl Drawer {
    pub fn new() -> Drawer {
        let mut corner_points = HashMap::<Corner, Vec2d<f64>>::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    corner_points.insert(
                        (i, j, k),
                        [
                            250.0 + (2.0 * f64::from(j) - 1.0) * (75.0 + f64::from(i) * 75.0),
                            250.0 + (2.0 * f64::from(k) - 1.0) * (75.0 + f64::from(i) * 75.0),
                        ],
                    );
                }
            }
        }

        let mut edges = vec![];
        for i in 0..2 {
            for j in 0..2 {
                edges.push(Edge::new((i, j, 0), (i, j, 1)));
                edges.push(Edge::new((i, 0, j), (i, 1, j)));
                edges.push(Edge::new((0, i, j), (1, i, j)));
            }
        }
        let edge_points = edges
            .iter()
            .map(|Edge { one, two }| {
                let [x0, y0] = corner_points[one];
                let [x1, y1] = corner_points[two];
                (Edge::new(*one, *two), [(x0 + x1) / 2.0, (y0 + y1) / 2.0])
            })
            .collect::<HashMap<Edge, Vec2d<f64>>>();

        let mut corner_stickers = HashMap::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let corner = (i, j, k);
                    corner_stickers
                        .insert((corner, Orientation::UD), ((i, 1 - j, k), (i, j, 1 - k)));
                    corner_stickers
                        .insert((corner, Orientation::LR), ((1 - i, j, k), (i, j, 1 - k)));
                    corner_stickers
                        .insert((corner, Orientation::FB), ((1 - i, j, k), (i, 1 - j, k)));
                }
            }
        }

        Drawer { corner_points, edge_points, corner_stickers }
    }
    pub fn draw<G: Graphics>(&self, skewb: &Skewb, c: &Context, g: &mut G) {
        let black = [0.0, 0.0, 0.0, 1.0];

        // Draw the 12 edges of the cube
        for Edge { one, two } in self.edge_points.keys() {
            let [x0, y0] = self.corner_points[one];
            let [x1, y1] = self.corner_points[two];
            Line::new(black, 1.0).draw([x0, y0, x1, y1], &c.draw_state, c.transform, g);
        }

        // Outline the centers on each face by connecting the midpoints of each pair of incident
        // edges.
        for (e0, [x0, y0]) in self.edge_points.iter() {
            for (e1, [x1, y1]) in self.edge_points.iter() {
                if e0 == e1 || e0.disjoint(e1)
                        // don't draw the down face:
                        || (e0.one.0 == 1 && e0.two.0 == 1 && e1.one.0 == 1 && e1.two.0 == 1) {
                    continue;
                } else {
                    Line::new(black, 1.0).draw([*x0, *y0, *x1, *y1], &c.draw_state, c.transform, g);
                }
            }
        }

        // Fill in the corner pieces
        for ((corner, sticker), (left_corner, right_corner)) in self.corner_stickers.iter() {
            // don't draw the down face
            if left_corner.0 == 1 && right_corner.0 == 1 {
                continue;
            }

            let corner_piece = skewb.get_corner_piece(*corner);
            let corner_orientation = skewb.get_corner_orientation(*corner);
            let color = corner_piece.sticker(*sticker - corner_orientation);
            let p = [
                self.corner_points[corner],
                self.edge_points[&Edge::new(*corner, *left_corner)],
                self.edge_points[&Edge::new(*corner, *right_corner)],
            ];
            Polygon::new(color.rgba()).draw(&p, &c.draw_state, c.transform, g);
        }

        // Fill in the centers
        let center_corners = vec![
            (Center::U, [(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)]),
            (Center::B, [(0, 0, 0), (0, 1, 0), (1, 1, 0), (1, 0, 0)]),
            (Center::F, [(0, 0, 1), (0, 1, 1), (1, 1, 1), (1, 0, 1)]),
            (Center::L, [(0, 0, 0), (0, 0, 1), (1, 0, 1), (1, 0, 0)]),
            (Center::R, [(0, 1, 0), (0, 1, 1), (1, 1, 1), (1, 1, 0)]),
        ];
        for (center, corners) in center_corners {
            let p = [
                self.edge_points[&Edge::new(corners[0], corners[1])],
                self.edge_points[&Edge::new(corners[1], corners[2])],
                self.edge_points[&Edge::new(corners[2], corners[3])],
                self.edge_points[&Edge::new(corners[3], corners[0])],
            ];
            Polygon::new(skewb.get_center_piece(center).rgba()).draw(
                &p,
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }
}
