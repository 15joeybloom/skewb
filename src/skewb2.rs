use std::ops::{Add, AddAssign, Sub};

pub type Corner = (u8, u8, u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    B,
    G,
    R,
    O,
    Y,
    W,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    UD,
    LR,
    FB,
}

impl Add<Orientation> for Orientation {
    type Output = Orientation;
    fn add(self, other: Orientation) -> Orientation {
        match (self, other) {
            (Orientation::UD, x) => x,
            (x, Orientation::UD) => x,
            (Orientation::LR, Orientation::LR) => Orientation::FB,
            (Orientation::FB, Orientation::LR) => Orientation::UD,
            (Orientation::LR, Orientation::FB) => Orientation::UD,
            (Orientation::FB, Orientation::FB) => Orientation::LR,
        }
    }
}
impl Sub<Orientation> for Orientation {
    type Output = Orientation;
    fn sub(self, other: Orientation) -> Orientation {
        match (self, other) {
            (x, Orientation::UD) => x,
            (Orientation::LR, Orientation::LR) => Orientation::UD,
            (Orientation::FB, Orientation::FB) => Orientation::UD,
            (Orientation::UD, Orientation::LR) => Orientation::FB,
            (Orientation::UD, Orientation::FB) => Orientation::LR,
            (Orientation::FB, Orientation::LR) => Orientation::LR,
            (Orientation::LR, Orientation::FB) => Orientation::FB,
        }
    }
}

impl AddAssign<Orientation> for Orientation {
    fn add_assign(&mut self, rhs: Orientation) { *self = *self + rhs; }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CornerPiece(pub Color, pub Color, pub Color);
impl CornerPiece {
    pub fn sticker(&self, o: Orientation) -> Color {
        match o {
            Orientation::UD => self.0,
            Orientation::LR => self.1,
            Orientation::FB => self.2,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Center {
    U,
    D,
    L,
    R,
    F,
    B,
}

#[derive(Debug)]
pub struct Skewb {
    fixed_orientations: [Orientation; 4],
    moving_pieces: [usize; 4],
    moving_orientations: [Orientation; 4],
    center_pieces: [Color; 6],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum FixedOrMoving {
    Fixed,
    Moving,
}

impl Skewb {
    pub fn new() -> Skewb {
        Skewb {
            fixed_orientations: [Orientation::UD; 4],
            moving_pieces: [0, 1, 2, 3],
            moving_orientations: [Orientation::UD; 4],
            center_pieces: [Color::Y, Color::B, Color::R, Color::G, Color::O, Color::W],
        }
    }

    fn fixed_or_moving(c: &Corner) -> (FixedOrMoving, usize) {
        match *c {
            (0, 0, 0) => (FixedOrMoving::Fixed, 0),
            (0, 0, 1) => (FixedOrMoving::Moving, 0),
            (0, 1, 1) => (FixedOrMoving::Fixed, 1),
            (0, 1, 0) => (FixedOrMoving::Moving, 1),
            (1, 0, 0) => (FixedOrMoving::Moving, 2),
            (1, 0, 1) => (FixedOrMoving::Fixed, 2),
            (1, 1, 1) => (FixedOrMoving::Moving, 3),
            (1, 1, 0) => (FixedOrMoving::Fixed, 3),
            x => panic!(format!("{:?} not a corner", x)),
        }
    }
    fn i_to_fixed_corner(i: usize) -> CornerPiece {
        match i {
            0 => CornerPiece(Color::Y, Color::O, Color::G),
            1 => CornerPiece(Color::Y, Color::R, Color::B),
            2 => CornerPiece(Color::W, Color::O, Color::B),
            3 => CornerPiece(Color::W, Color::R, Color::G),
            x => panic!(format!("{:?} not a fixed corner piece index", x)),
        }
    }
    fn i_to_moving_corner(i: usize) -> CornerPiece {
        match i {
            0 => CornerPiece(Color::Y, Color::O, Color::B),
            1 => CornerPiece(Color::Y, Color::R, Color::G),
            2 => CornerPiece(Color::W, Color::O, Color::G),
            3 => CornerPiece(Color::W, Color::R, Color::B),
            x => panic!(format!("{:?} not a moving corner piece index", x)),
        }
    }
    fn center_to_i(c: &Center) -> usize {
        match c {
            Center::U => 0,
            Center::F => 1,
            Center::R => 2,
            Center::B => 3,
            Center::L => 4,
            Center::D => 5,
        }
    }

    pub fn get_corner_piece(&self, c: &Corner) -> CornerPiece {
        match Self::fixed_or_moving(c) {
            (FixedOrMoving::Fixed, i) => Self::i_to_fixed_corner(i),
            (FixedOrMoving::Moving, i) => Self::i_to_moving_corner(self.moving_pieces[i]),
        }
    }
    pub fn get_corner_orientation(&self, c: &Corner) -> Orientation {
        match Self::fixed_or_moving(c) {
            (FixedOrMoving::Fixed, i) => self.fixed_orientations[i],
            (FixedOrMoving::Moving, i) => self.moving_orientations[i],
        }
    }
    pub fn get_center_piece(&self, c: &Center) -> Color { self.center_pieces[Self::center_to_i(c)] }

    fn rotate_elements<V>(array: &mut [V], keys: &Vec<usize>) {
        if keys.len() <= 1 {
            return;
        }

        for i in 0..keys.len() - 1 {
            array.swap(keys[i], keys[i + 1]);
        }
    }

    pub fn turn_lr(&mut self, c: &Corner) {
        let i = if let (FixedOrMoving::Fixed, i) = Self::fixed_or_moving(c) {
            i
        } else {
            panic!("Can't turn a moving corner");
        };

        let corners = [
            (c.0, c.1, 1 - c.2),
            (c.0, 1 - c.1, c.2),
            (1 - c.0, c.1, c.2),
        ].into_iter()
            .map(|x| Self::fixed_or_moving(&x).1)
            .collect();

        Self::rotate_elements(&mut self.moving_pieces, &corners);
        Self::rotate_elements(&mut self.moving_orientations, &corners);

        *self.fixed_orientations.get_mut(i).unwrap() += Orientation::LR;

        for &c in corners.iter() {
            *self.moving_orientations.get_mut(c).unwrap() += Orientation::LR;
        }

        let centers = [
            if c.2 == 0 { Center::B } else { Center::F },
            if c.1 == 0 { Center::L } else { Center::R },
            if c.0 == 0 { Center::U } else { Center::D },
        ].into_iter()
            .map(|x| Self::center_to_i(&x))
            .collect();
        Self::rotate_elements(&mut self.center_pieces, &centers);
    }
    pub fn turn_fb(&mut self, c: &Corner) {
        self.turn_lr(c);
        self.turn_lr(c);
    }

    /*
    pub fn rotate_ud(&mut self) {
        let corners = [(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)]
            .into_iter()
            .map(|x| Self::fixed_or_moving(&x).1)
            .collect();

        Self::rotate_elements(&mut self.moving_pieces, &corners);
        Self::rotate_elements(&mut self.moving_orientations, &corners);

        let corners = [(1, 0, 0), (1, 0, 1), (1, 1, 1), (1, 1, 0)]
            .into_iter()
            .map(|x| Self::fixed_or_moving(&x).1)
            .collect();

        Self::rotate_elements(&mut self.moving_pieces, &corners);
        Self::rotate_elements(&mut self.moving_orientations, &corners);

        let centers = [Center::B, Center::L, Center::F, Center::R]
            .into_iter()
            .map(|x| Self::center_to_i(&x))
            .collect();
        Self::rotate_elements(&mut self.center_pieces, &centers);
    }
    */
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Turn {
    corners: [Corner; 3],
    centers: [Center; 3],
    twisty: Corner,
}
