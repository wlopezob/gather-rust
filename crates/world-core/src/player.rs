use crate::geometry::{Direction, TilePos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlayerId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub pos: TilePos,
    pub facing: Direction,
}

impl Player {
    /// Empieza mirando hacia abajo
    pub fn new(id: PlayerId, name: impl Into<String>, pos: TilePos) -> Self {
        Self {
            id,
            name: name.into(),
            pos,
            facing: Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_acepta_str_y_string_y_mira_abajo() {
        let jugador = Player::new(PlayerId(1), "Ana", TilePos::new(2, 3));
        assert_eq!("Ana", jugador.name);
        assert_eq!(jugador.facing, Direction::Down);
        assert_eq!(jugador.id, PlayerId(1));
    }

    #[test]
    fn clonar_no_comparte_estado() {
        let original = Player::new(PlayerId(1), "original", TilePos::new(2, 2));
        let mut copia = original.clone();
        copia.pos = TilePos::new(9, 9);
        assert_eq!(TilePos::new(2, 2), original.pos);
    }
}
