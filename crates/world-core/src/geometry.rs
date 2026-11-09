#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TilePos {
    #[must_use]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// La casilla vecina en esa direccion; `None` si sale del rango de  `u16`.
    #[must_use]
    pub fn step(self, dir: Direction) -> Option<TilePos> {
        let x = self.x;
        let y = self.y;
        if (y == u16::MAX && dir == Direction::Down) || (x == u16::MAX && dir == Direction::Right) {
            return None;
        } else if (x == 0 && dir == Direction::Left) || (y == 0 && dir == Direction::Up) {
            return None;
        }
        let (x, y) = match dir {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Up => (x, y - 1),
        };
        return Some(TilePos { x, y });
    }
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    #[must_use]
    pub fn opposite(self) -> Direction {
        match self {
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_guarda_las_coordenadas() {
        let p = TilePos::new(3, 7);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 7);
    }

    #[test]
    fn step_avanza_una_casilla() {
        let p = TilePos::new(10, 10);
        let up = p.step(Direction::Up);
        assert_eq!(10, up.unwrap().x);
        assert_eq!(9, up.unwrap().y);

        let down = p.step(Direction::Down);
        assert_eq!(10, down.unwrap().x);
        assert_eq!(11, down.unwrap().y);

        let left = p.step(Direction::Left);
        assert_eq!(9, left.unwrap().x);
        assert_eq!(10, left.unwrap().y);

        let right = p.step(Direction::Right);
        assert_eq!(11, right.unwrap().x);
        assert_eq!(10, right.unwrap().y);
    }

    #[test]
    fn step_en_los_bordes_devuelve_none() {
        let p = TilePos::new(0, 5);
        let l = p.step(Direction::Left);
        assert_eq!(None, l);

        let p = TilePos::new(5, 0);
        let up = p.step(Direction::Up);
        assert_eq!(None, up);

        let p = TilePos::new(u16::MAX, 5);
        let r = p.step(Direction::Right);
        assert_eq!(None, r);

        let p = TilePos::new(5, u16::MAX);
        let d = p.step(Direction::Down);
        assert_eq!(None, d);
    }

    #[test]
    fn opposite_es_involutiva() {
        let d = Direction::ALL[0];
        assert_eq!(d, d.opposite().opposite());
        assert_ne!(d, d.opposite());

        let d = Direction::ALL[1];
        assert_eq!(d, d.opposite().opposite());
        assert_ne!(d, d.opposite());

        let d = Direction::ALL[2];
        assert_eq!(d, d.opposite().opposite());
        assert_ne!(d, d.opposite());

        let d = Direction::ALL[3];
        assert_eq!(d, d.opposite().opposite());
        assert_ne!(d, d.opposite());
    }

    #[test]
    fn ida_y_vuelta_vuelve_al_origen() {
        let p = TilePos::new(10, 10);
        let d: Direction = Direction::ALL[0];
        assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));

        let d = Direction::ALL[1];
        assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));

        let d = Direction::ALL[2];
        assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));

        let d = Direction::ALL[3];
        assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));
    }
}
