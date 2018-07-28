use std;
use std::collections::HashMap;
use std::hash::Hash;

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

impl std::ops::Add<Orientation> for Orientation {
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
impl std::ops::Sub<Orientation> for Orientation {
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

impl std::ops::AddAssign<Orientation> for Orientation {
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
impl Center {
    pub fn centers_for_corner(c: &Corner) -> [Center; 3] {
        [
            if c.2 == 0 { Center::B } else { Center::F },
            if c.1 == 0 { Center::L } else { Center::R },
            if c.0 == 0 { Center::U } else { Center::D },
        ]
    }
}

#[derive(Debug)]
pub struct Skewb {
    corner_pieces: [usize; 8],
    corner_orientations: [Orientation; 8],
    center_pieces: HashMap<Center, Color>,
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),+ $(,)*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )+
         map
    }}
}

impl Skewb {
    pub fn new() -> Skewb {
        Skewb {
            corner_pieces: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_orientations: [Orientation::UD; 8],
            center_pieces: hashmap![
                Center::U => Color::Y,
                Center::F => Color::B,
                Center::R => Color::R,
                Center::B => Color::G,
                Center::L => Color::O,
                Center::D => Color::W,
            ],
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
    fn i_to_corner_piece(i: usize) -> CornerPiece {
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
    }

    pub fn get_corner_piece(&self, c: &Corner) -> CornerPiece {
        Self::i_to_corner_piece(self.corner_pieces[Self::corner_to_i(c)])
    }
    pub fn get_corner_orientation(&self, c: &Corner) -> Orientation {
        self.corner_orientations[Self::corner_to_i(c)]
    }
    pub fn get_center_piece(&self, c: &Center) -> Color { self.center_pieces[c] }

    fn rotate_elements<V>(array: &mut [V], keys: &Vec<usize>) {
        if keys.len() <= 1 {
            return;
        }

        for i in 0..keys.len() - 1 {
            array.swap(keys[i], keys[i + 1]);
        }
    }

    fn rotate_values<K, V>(h: &mut HashMap<K, V>, keys: &[K])
    where K: Eq + Hash + Copy {
        if keys.len() <= 1 {
            return;
        }

        let temp = h.remove(keys.first().unwrap()).unwrap();
        for i in 0..keys.len() - 1 {
            let x = h.remove(&keys[i + 1]).unwrap();
            h.insert(keys[i], x);
        }
        h.insert(*keys.last().unwrap(), temp);
    }

    pub fn turn_lr(&mut self, c: &Corner) {
        let corners = [
            (c.0, c.1, 1 - c.2),
            (c.0, 1 - c.1, c.2),
            (1 - c.0, c.1, c.2),
        ].into_iter()
            .map(|x| Self::corner_to_i(&x))
            .collect();

        *self
            .corner_orientations
            .get_mut(Self::corner_to_i(&c))
            .unwrap() += Orientation::LR;

        Self::rotate_values(&mut self.center_pieces, &Center::centers_for_corner(&c));

        Self::rotate_elements(&mut self.corner_pieces, &corners);
        Self::rotate_elements(&mut self.corner_orientations, &corners);
        for &c in corners.iter() {
            *self.corner_orientations.get_mut(c).unwrap() += Orientation::LR;
        }
    }
    pub fn turn_fb(&mut self, c: &Corner) {
        self.turn_lr(c);
        self.turn_lr(c);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Turn {
    corners: [Corner; 3],
    centers: [Center; 3],
    twisty: Corner,
}
