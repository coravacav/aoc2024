use crate::grid::Coord;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum QuadDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl std::fmt::Display for QuadDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadDirection::Up => write!(f, "^"),
            QuadDirection::Down => write!(f, "v"),
            QuadDirection::Left => write!(f, "<"),
            QuadDirection::Right => write!(f, ">"),
            QuadDirection::None => write!(f, "?"),
        }
    }
}

impl From<char> for QuadDirection {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid char {c}"),
        }
    }
}

impl QuadDirection {
    pub fn rotate_right(self) -> QuadDirection {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::None => panic!("Cannot rotate None"),
        }
    }

    pub fn to_coord_offset(self) -> Coord {
        match self {
            Self::Up => Coord::new(-1, 0),
            Self::Down => Coord::new(1, 0),
            Self::Left => Coord::new(0, -1),
            Self::Right => Coord::new(0, 1),
            Self::None => panic!("Cannot get offset of None"),
        }
    }

    pub fn get_non_opposite_directions(self) -> &'static [Self] {
        match self {
            Self::Up => &[Self::Right, Self::Up, Self::Left],
            Self::Down => &[Self::Left, Self::Down, Self::Right],
            Self::Left => &[Self::Down, Self::Left, Self::Up],
            Self::Right => &[Self::Up, Self::Right, Self::Down],
            Self::None => &[Self::Right, Self::Down, Self::Left, Self::Up],
        }
    }

    pub fn get_all_other_directions(self) -> &'static [Self] {
        match self {
            Self::Up => &[Self::Right, Self::Down, Self::Left],
            Self::Down => &[Self::Left, Self::Up, Self::Right],
            Self::Left => &[Self::Up, Self::Right, Self::Down],
            Self::Right => &[Self::Down, Self::Left, Self::Up],
            Self::None => &[Self::Right, Self::Down, Self::Left, Self::Up],
        }
    }

    pub fn get_all_directions() -> &'static [Self] {
        &[Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

impl std::ops::Add<Coord> for QuadDirection {
    type Output = Coord;

    fn add(self, other: Coord) -> Self::Output {
        other + self.to_coord_offset()
    }
}

impl std::ops::Add<QuadDirection> for Coord {
    type Output = Coord;

    fn add(self, other: QuadDirection) -> Self::Output {
        self + other.to_coord_offset()
    }
}

impl std::ops::AddAssign<QuadDirection> for Coord {
    fn add_assign(&mut self, other: QuadDirection) {
        *self += other.to_coord_offset();
    }
}

impl std::ops::Sub<QuadDirection> for Coord {
    type Output = Coord;

    fn sub(self, other: QuadDirection) -> Self::Output {
        self - other.to_coord_offset()
    }
}

impl std::ops::SubAssign<QuadDirection> for Coord {
    fn sub_assign(&mut self, other: QuadDirection) {
        *self -= other.to_coord_offset();
    }
}
