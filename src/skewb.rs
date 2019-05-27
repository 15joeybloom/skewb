use std::ops::{Add, AddAssign, Sub};
use std::collections::HashSet;

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
    pub fn sticker(self, o: Orientation) -> Color {
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

fn rotate_elements<V>(array: &mut [V], keys: &[usize]) {
    if keys.len() <= 1 {
        return;
    }

    for i in 0..keys.len() - 1 {
        array.swap(keys[i], keys[i + 1]);
    }
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
            // TODO: this can be computed in a method; we don't need to store this.
            even_rotation: true,
        }
    }

    fn corner_to_i(c: Corner) -> usize {
        match c {
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
    fn center_to_i(c: Center) -> usize {
        match c {
            Center::U => 0,
            Center::F => 1,
            Center::R => 2,
            Center::B => 3,
            Center::L => 4,
            Center::D => 5,
        }
    }

    pub fn get_corner_piece(&self, c: Corner) -> CornerPiece {
        self.i_to_corner_piece(self.corner_pieces[Self::corner_to_i(c)])
    }
    pub fn get_corner_orientation(&self, c: Corner) -> Orientation {
        self.corner_orientations[Self::corner_to_i(c)]
    }
    pub fn get_center_piece(&self, c: Center) -> Color { self.center_pieces[Self::center_to_i(c)] }

    pub fn turn_lr(&mut self, c: Corner) {
        let corners : Vec<usize> = [
            (c.0, c.1, 1 - c.2),
            (c.0, 1 - c.1, c.2),
            (1 - c.0, c.1, c.2),
        ].iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        *self
            .corner_orientations
            .get_mut(Self::corner_to_i(c))
            .unwrap() += Orientation::LR;

        for &c in corners.iter() {
            *self.corner_orientations.get_mut(c).unwrap() += Orientation::LR;
        }

        let centers : Vec<usize> = [
            if c.2 == 0 { Center::B } else { Center::F },
            if c.1 == 0 { Center::L } else { Center::R },
            if c.0 == 0 { Center::U } else { Center::D },
        ].iter()
            .map(|x| Self::center_to_i(*x))
            .collect();
        rotate_elements(&mut self.center_pieces, &centers);
    }
    pub fn turn_fb(&mut self, c: Corner) {
        self.turn_lr(c);
        self.turn_lr(c);
    }

    pub fn rotate_ud(&mut self) {
        let corners : Vec<usize> = [(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let corners : Vec<usize> = [(1, 0, 0), (1, 0, 1), (1, 1, 1), (1, 1, 0)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let centers : Vec<usize> = [Center::B, Center::L, Center::F, Center::R]
            .iter()
            .map(|x| Self::center_to_i(*x))
            .collect();
        rotate_elements(&mut self.center_pieces, &centers);

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
        let corners : Vec<usize> = [(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let corners : Vec<usize> = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let centers : Vec<usize> = [Center::U, Center::L, Center::D, Center::R]
            .iter()
            .map(|x| Self::center_to_i(*x))
            .collect();
        rotate_elements(&mut self.center_pieces, &centers);

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
        let corners : Vec<usize> = [(0, 0, 1), (1, 0, 1), (1, 0, 0), (0, 0, 0)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let corners : Vec<usize> = [(0, 1, 1), (1, 1, 1), (1, 1, 0), (0, 1, 0)]
            .iter()
            .map(|x| Self::corner_to_i(*x))
            .collect();

        rotate_elements(&mut self.corner_pieces, &corners);
        rotate_elements(&mut self.corner_orientations, &corners);

        let centers : Vec<usize> = [Center::U, Center::F, Center::D, Center::B]
            .iter()
            .map(|x| Self::center_to_i(*x))
            .collect();
        rotate_elements(&mut self.center_pieces, &centers);

        self.even_rotation = !self.even_rotation;
        for i in 0..8 {
            self.corner_orientations[i] = match self.corner_orientations[i] {
                Orientation::UD => Orientation::FB,
                Orientation::LR => Orientation::LR,
                Orientation::FB => Orientation::UD,
            };
        }
    }

    fn i_to_moving_i(i: usize) -> usize {
        match i {
            1 => 0,
            3 => 1,
            4 => 2,
            6 => 3,
            _ => panic!("Not a moving corner index!"),
        }
    }

    pub fn normalize(self) -> NormalizedSkewb {
        // TODO: Automatically rotate self so the fixed corners are permuted correctly
        if self.corner_pieces[0] != 0 && self.corner_pieces[2] != 2 {
            panic!("Cannot normalize skewb. Please rotate it for me.")
        }
        let fixed_orientations = [
            self.corner_orientations[0],
            self.corner_orientations[2],
            self.corner_orientations[5],
            self.corner_orientations[7],
        ];
        let moving_orientations = [
            self.corner_orientations[1],
            self.corner_orientations[3],
            self.corner_orientations[4],
            self.corner_orientations[6],
        ];
        let moving_pieces = [
            Self::i_to_moving_i(self.corner_pieces[1]),
            Self::i_to_moving_i(self.corner_pieces[3]),
            Self::i_to_moving_i(self.corner_pieces[4]),
            Self::i_to_moving_i(self.corner_pieces[6]),
        ];
        NormalizedSkewb {
            center_pieces: self.center_pieces,
            fixed_orientations,
            moving_orientations,
            moving_pieces,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NormalizedSkewb {
    pub fixed_orientations: [Orientation; 4],
    pub moving_pieces: [usize; 4],
    pub moving_orientations: [Orientation; 4],
    pub center_pieces: [Color; 6],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum FixedOrMoving {
    Fixed,
    Moving,
}

impl NormalizedSkewb {
    pub fn new() -> NormalizedSkewb {
        NormalizedSkewb {
            fixed_orientations: [Orientation::UD; 4],
            moving_pieces: [0, 1, 2, 3],
            moving_orientations: [Orientation::UD; 4],
            center_pieces: [Color::Y, Color::B, Color::R, Color::G, Color::O, Color::W],
        }
    }

    fn fixed_or_moving(c: Corner) -> (FixedOrMoving, usize) {
        match c {
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
    fn center_to_i(c: Center) -> usize {
        match c {
            Center::U => 0,
            Center::F => 1,
            Center::R => 2,
            Center::B => 3,
            Center::L => 4,
            Center::D => 5,
        }
    }

    pub fn turn_lr(&mut self, c: Corner) {
        let i = if let (FixedOrMoving::Fixed, i) = Self::fixed_or_moving(c) {
            i
        } else {
            panic!("Can't turn a moving corner");
        };

        let corners : Vec<usize> = [
            (c.0, c.1, 1 - c.2),
            (c.0, 1 - c.1, c.2),
            (1 - c.0, c.1, c.2),
        ].iter()
            .map(|x| Self::fixed_or_moving(*x).1)
            .collect();

        rotate_elements(&mut self.moving_pieces, &corners);
        rotate_elements(&mut self.moving_orientations, &corners);

        *self.fixed_orientations.get_mut(i).unwrap() += Orientation::LR;

        for &c in corners.iter() {
            *self.moving_orientations.get_mut(c).unwrap() += Orientation::LR;
        }

        let centers : Vec<usize> = [
            if c.2 == 0 { Center::B } else { Center::F },
            if c.1 == 0 { Center::L } else { Center::R },
            if c.0 == 0 { Center::U } else { Center::D },
        ].iter()
            .map(|x| Self::center_to_i(*x))
            .collect();
        rotate_elements(&mut self.center_pieces, &centers);
    }
    pub fn turn_fb(&mut self, c: Corner) {
        self.turn_lr(c);
        self.turn_lr(c);
    }

    fn moving_i_to_i(i: usize) -> usize {
        match i {
            0 => 1,
            1 => 3,
            2 => 4,
            3 => 6,
            _ => panic!("Not a moving corner index!"),
        }
    }

    pub fn denormalize(self) -> Skewb {
        let corner_pieces = [
            0,
            Self::moving_i_to_i(self.moving_pieces[0]),
            2,
            Self::moving_i_to_i(self.moving_pieces[1]),
            Self::moving_i_to_i(self.moving_pieces[2]),
            5,
            Self::moving_i_to_i(self.moving_pieces[3]),
            7,
        ];
        let corner_orientations = [
            self.fixed_orientations[0],
            self.moving_orientations[0],
            self.fixed_orientations[1],
            self.moving_orientations[1],
            self.moving_orientations[2],
            self.fixed_orientations[2],
            self.moving_orientations[3],
            self.fixed_orientations[3],
        ];
        Skewb {
            center_pieces: self.center_pieces,
            corner_orientations,
            corner_pieces,
            even_rotation: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    LR,
    FB,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    direction: Direction,
    corner: Corner,
}

impl NormalizedSkewb {
    pub fn do_move(&mut self, move_: &Move) {
        match move_.direction {
            Direction::FB => self.turn_fb(move_.corner),
            Direction::LR => self.turn_lr(move_.corner),
        }
    }
    pub fn undo_move(&mut self, move_: &Move){
        match move_.direction {
            Direction::LR => self.turn_fb(move_.corner),
            Direction::FB => self.turn_lr(move_.corner),
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == NormalizedSkewb::new()
    }

    fn _solution(
        &mut self,
        move_stack: &mut Vec<Move>,
        discovered: &mut HashSet<NormalizedSkewb>,
        max_length: usize,
    ) -> bool {
        if self.is_solved() {
            return true;
        } else if move_stack.len() >= max_length || discovered.contains(self) {
            return false;
        }
        discovered.insert(self.clone());

        for corner in vec![(0, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)].into_iter() {
            if let Some(last_move) = move_stack.last() {
                if last_move.corner == corner {
                    continue;
                }
            }

            for direction in vec![Direction::FB, Direction::LR].into_iter() {
                let move_ = Move { direction, corner};
                self.do_move(&move_);
                move_stack.push(move_.clone());
                let has_solution = self._solution(move_stack, discovered, max_length);
                self.undo_move(&move_);
                if has_solution {
                    return true;
                } else {
                    move_stack.pop();
                }
            }
        }
        discovered.remove(self);
        return false;
    }
    pub fn solution(&mut self) -> Option<Vec<Move>> {
        // Iterative DFS

        for solution_length in 0..20 {
            let mut discovered = HashSet::new();
            let mut move_stack = vec![];
            if self._solution(&mut move_stack, &mut discovered, solution_length) {
                return Some(move_stack);
            }
        }
        None
    }
}

#[test]
fn already_solved() {
    let mut sut = NormalizedSkewb::new();
    let solution = sut.solution().unwrap();
    assert!(solution.is_empty());
}

#[test]
fn one_move_solution() {
    let mut sut = NormalizedSkewb::new();
    sut.turn_lr((0, 0, 0));
    let solution = sut.solution().unwrap();
    assert_eq!(1, solution.len());
    assert_eq!(Direction::FB, solution[0].direction);
    assert_eq!((0, 0, 0), solution[0].corner);
}

#[test]
fn two_lefts_make_a_right() {
    let mut sut = NormalizedSkewb::new();
    sut.turn_lr((0, 0, 0));
    sut.turn_lr((0, 0, 0));
    let solution = sut.solution().unwrap();
    assert_eq!(1, solution.len());
    assert_eq!(Direction::LR, solution[0].direction);
    assert_eq!((0, 0, 0), solution[0].corner);
}

#[test]
fn two_move_solution() {
    let mut sut = NormalizedSkewb::new();
    sut.turn_lr((0, 0, 0));
    sut.turn_lr((1, 0, 1));
    let solution = sut.solution().unwrap();
    assert_eq!(2, solution.len());
    assert_eq!(Direction::FB, solution[0].direction);
    assert_eq!((1, 0, 1), solution[0].corner);
    assert_eq!(Direction::FB, solution[1].direction);
    assert_eq!((0, 0, 0), solution[1].corner);
}

#[test]
fn four_move_solution() {
    let mut sut = NormalizedSkewb::new();
    sut.turn_lr((0, 0, 0));
    sut.turn_lr((1, 0, 1));
    sut.turn_fb((0, 0, 0));
    sut.turn_fb((1, 0, 1));
    let solution = sut.solution().unwrap();
    assert_eq!(4, solution.len());
    assert_eq!(Direction::LR, solution[0].direction);
    assert_eq!((1, 0, 1), solution[0].corner);
    assert_eq!(Direction::LR, solution[1].direction);
    assert_eq!((0, 0, 0), solution[1].corner);
    assert_eq!(Direction::FB, solution[2].direction);
    assert_eq!((1, 0, 1), solution[2].corner);
    assert_eq!(Direction::FB, solution[3].direction);
    assert_eq!((0, 0, 0), solution[3].corner);
}
