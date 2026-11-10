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
        let (x, y) = match dir {
            Direction::Left => (x.checked_sub(1)?, y),
            Direction::Right => (x.checked_add(1)?, y),
            Direction::Down => (x, y.checked_add(1)?),
            Direction::Up => (x, y.checked_sub(1)?),
        };
        Some(TilePos { x, y })
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
        assert_eq!(Some(TilePos::new(10, 9)), p.step(Direction::Up));
        assert_eq!(Some(TilePos::new(10, 11)), p.step(Direction::Down));
        assert_eq!(Some(TilePos::new(9, 10)), p.step(Direction::Left));
        assert_eq!(Some(TilePos::new(11, 10)), p.step(Direction::Right));
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
        for d in Direction::ALL {
            assert_eq!(d, d.opposite().opposite(), "fallo con {d:?}");
            assert_ne!(d, d.opposite(), "fallo con {d:?}");
        }
    }

    #[test]
    fn ida_y_vuelta_vuelve_al_origen() {
        let p = TilePos::new(10, 10);
        for d in Direction::ALL {
            assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));
            assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));
            assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));
            assert_eq!(Some(p), p.step(d).and_then(|q| q.step(d.opposite())));
        }
    }
}
