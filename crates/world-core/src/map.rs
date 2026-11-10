use crate::geometry::TilePos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileMap {
    width: u16,
    height: u16,
    blocked: Vec<bool>, // fila por fila: índice = y * width + x
}

impl TileMap {
    /// `None` si `width` o `height` es 0, o si `blocked.len() != width * height`.
    #[must_use]
    pub fn new(width: u16, height: u16, blocked: Vec<bool>) -> Option<Self> {
        if width == 0 || height == 0 || blocked.len() != usize::from(width) * usize::from(height) {
            return None;
        }
        Some(Self {
            width,
            height,
            blocked,
        })
    }

    #[must_use]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[must_use]
    pub fn contains(&self, pos: TilePos) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// Fuera del mapa cuenta como no caminable
    #[must_use]
    pub fn is_walkable(&self, pos: TilePos) -> bool {
        if !self.contains(pos) {
            return false;
        }
        let indice = usize::from(pos.y) * usize::from(self.width) + usize::from(pos.x);
        !self.blocked[indice]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map_3x2() -> TileMap {
        TileMap::new(3, 2, vec![false, true, false, false, false, true])
            .expect("3 x 2 = 6 casillas")
    }

    #[test]
    fn new_rechaza_dimensiones_invalidas() {
        assert_eq!(TileMap::new(0, 2, vec![]), None);
        assert_eq!(TileMap::new(3, 0, vec![]), None);
        assert_eq!(TileMap::new(3, 2, vec![false; 5]), None);
        assert!(TileMap::new(3, 2, vec![false; 6]).is_some());
    }

    #[test]
    fn new_acepta_mapa_de_256x256() {
        assert!(TileMap::new(256, 256, vec![false; 65_536]).is_some());
    }

    #[test]
    fn contains_en_los_bordes() {
        let map = map_3x2();
        assert!(map.contains(TilePos::new(0, 0)));
        assert!(map.contains(TilePos::new(2, 1)));

        assert!(!map.contains(TilePos::new(3, 0)));
        assert!(!map.contains(TilePos::new(0, 2)));
    }

    #[test]
    fn is_walkable_libre_muro_y_fuera() {
        let map = map_3x2();
        assert!(map.is_walkable(TilePos::new(0, 0)));
        assert!(!map.is_walkable(TilePos::new(1, 0)));
        assert!(!map.is_walkable(TilePos::new(2, 1)));
        assert!(!map.is_walkable(TilePos::new(3, 0)));
        assert!(!map.is_walkable(TilePos::new(u16::MAX, u16::MAX)));
    }
}
