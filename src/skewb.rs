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
    corner_pieces: [usize; 8],
    corner_orientations: [Orientation; 8],
    center_pieces: [Color; 6],
    even_rotation: bool,
}

impl Skewb {
    pub fn new() -> Skewb {
        Skewb {
            corner_pieces: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_orientations: [Orientation::UD; 8],
            center_pieces: [Color::Y, Color::B, Color::R, Color::G, Color::O, Color::W],
            even_rotation: true,
        }
    }

    fn corner_to_i(c: &Corner) -> usize {
        match *c {
            (0, 0, 0) => 0,
            (0, 0, 1) => 1,
            (0, 1, 1) => 2,
            (0, 1, 0) => 3,
            (1, 0, 0) => 4,
            (1, 0, 1) => 5,
            (1, 1, 1) => 6,
            (1, 1, 0) => 7,
            x => panic!(format!("{:?} not a corner", x)),
        }
    }
    fn i_to_corner_piece(&self, i: usize) -> CornerPiece {
        if self.even_rotation {
            match i {
                0 => CornerPiece(Color::Y, Color::O, Color::G),
                1 => CornerPiece(Color::Y, Color::O, Color::B),
                2 => CornerPiece(Color::Y, Color::R, Color::B),
                3 => CornerPiece(Color::Y, Color::R, Color::G),
                4 => CornerPiece(Color::W, Color::O, Color::G),
                5 => CornerPiece(Color::W, Color::O, Color::B),
                6 => CornerPiece(Color::W, Color::R, Color::B),
                7 => CornerPiece(Color::W, Color::R, Color::G),
                x => panic!(format!("{:?} not a corner piece index", x)),
            }
        } else {
            match i {
                0 => CornerPiece(Color::Y, Color::G, Color::O),
                1 => CornerPiece(Color::Y, Color::B, Color::O),
                2 => CornerPiece(Color::Y, Color::B, Color::R),
                3 => CornerPiece(Color::Y, Color::G, Color::R),
                4 => CornerPiece(Color::W, Color::G, Color::O),
                5 => CornerPiece(Color::W, Color::B, Color::O),
                6 => CornerPiece(Color::W, Color::B, Color::R),
                7 => CornerPiece(Color::W, Color::G, Color::R),
                x => panic!(format!("{:?} not a corner piece index", x)),
            }
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
        self.i_to_corner_piece(self.corner_pieces[Self::corner_to_i(c)])
    }
    pub fn get_corner_orientation(&self, c: &Corner) -> Orientation {
        self.corner_orientations[Self::corner_to_i(c)]
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
        let corners = [
            (c.0, c.1, 1 - c.2),
            (c.0, 1 - c.1, c.2),
            (1 - c.0, c.1, c.2),
        ].into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        *self
            .corner_orientations
            .get_mut(Self::corner_to_i(&c))
            .unwrap() += Orientation::LR;

        for &c in corners.iter() {
            *self.corner_orientations.get_mut(c).unwrap() += Orientation::LR;
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

    pub fn rotate_ud(&mut self) {
        let corners = [(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let corners = [(1, 0, 0), (1, 0, 1), (1, 1, 1), (1, 1, 0)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let centers = [Center::B, Center::L, Center::F, Center::R]
            .into_iter()
            .map(|x| Self::center_to_i(&x))
            .collect();
        Self::rotate_elements(&mut self.center_pieces, &centers);

        self.even_rotation = !self.even_rotation;
        for i in 0..8 {
            self.corner_orientations[i] = match self.corner_orientations[i] {
                Orientation::UD => Orientation::UD,
                Orientation::LR => Orientation::FB,
                Orientation::FB => Orientation::LR,
            };
        }
    }

    pub fn rotate_fb(&mut self) {
        let corners = [(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let corners = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let centers = [Center::U, Center::L, Center::D, Center::R]
            .into_iter()
            .map(|x| Self::center_to_i(&x))
            .collect();
        Self::rotate_elements(&mut self.center_pieces, &centers);

        self.even_rotation = !self.even_rotation;
        for i in 0..8 {
            self.corner_orientations[i] = match self.corner_orientations[i] {
                Orientation::UD => Orientation::LR,
                Orientation::LR => Orientation::UD,
                Orientation::FB => Orientation::FB,
            };
        }
    }

    pub fn rotate_lr(&mut self) {
        let corners = [(0, 0, 1), (1, 0, 1), (1, 0, 0), (0, 0, 0)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let corners = [(0, 1, 1), (1, 1, 1), (1, 1, 0), (0, 1, 0)]
            .into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);

        let centers = [Center::U, Center::F, Center::D, Center::B]
            .into_iter()
            .map(|x| Self::center_to_i(&x))
            .collect();
        Self::rotate_elements(&mut self.center_pieces, &centers);

        self.even_rotation = !self.even_rotation;
        for i in 0..8 {
            self.corner_orientations[i] = match self.corner_orientations[i] {
                Orientation::UD => Orientation::FB,
                Orientation::LR => Orientation::LR,
                Orientation::FB => Orientation::UD,
            };
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Turn {
    corners: [Corner; 3],
    centers: [Center; 3],
    twisty: Corner,
}
