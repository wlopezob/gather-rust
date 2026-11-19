use std::collections::{HashMap, hash_map::Entry};

use crate::{Direction, Player, PlayerId, TileMap, TilePos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SpawnError {
    #[error("el jugador {0:?} ya existe")]
    DuplicateId(PlayerId),

    #[error("No se puede trasladar al casillero {0:?}")]
    NotWalkable(TilePos),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Move(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ApplyError {
    #[error("Usuario no existe")]
    UnknownPlayer(PlayerId),

    #[error("El casillero esta bloqueado")]
    Blocked,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    tile_map: TileMap,
    players: HashMap<PlayerId, Player>,
}

impl World {
    fn player_mut(&mut self, id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    #[must_use]
    pub fn new(tile_map: TileMap) -> Self {
        Self {
            tile_map,
            players: HashMap::new(),
        }
    }

    #[must_use]
    pub fn tile_map(&self) -> &TileMap {
        &self.tile_map
    }

    #[must_use]
    pub fn player(&self, id: PlayerId) -> Option<&Player> {
        self.players.get(&id)
    }

    /// Agrega un jugador al mundo.
    ///
    /// # Errors
    /// Devuelve [`SpawnError::DuplicateId`] si ya existe un jugador con ese ID.
    /// Devuelve [`SpawnError::NotWalkable`] si la posición no es transitable.
    pub fn spawn(&mut self, p: Player) -> Result<(), SpawnError> {
        if !self.tile_map.is_walkable(p.pos) {
            return Err(SpawnError::NotWalkable(p.pos));
        }
        match self.players.entry(p.id) {
            Entry::Vacant(items) => {
                items.insert(p);
                Ok(())
            }
            Entry::Occupied(_) => Err(SpawnError::DuplicateId(p.id)),
        }
    }

    /// Mueve un jugador al mundo.
    ///
    /// # Errors
    /// Devuelve [`ApplyError::UnknownPlayer`] usuario desconocido.
    /// Devuelve [`ApplyError::Blocked`] si la posición no es transitable.
    pub fn apply(&mut self, id: PlayerId, input: Input) -> Result<(), ApplyError> {
        let Input::Move(dir) = input;
        // 1. Solo leer: el & termina en esta misma línea, porque .pos copia la posición.
        let actual = self.player(id).ok_or(ApplyError::UnknownPlayer(id))?.pos;
        let destino = actual.step(dir).ok_or(ApplyError::Blocked)?;
        if !self.tile_map.is_walkable(destino) {
            return Err(ApplyError::Blocked);
        }

        // 2. Recién aquí se presta todo self como &mut. Desde acá nadie más usa self.
        let player = self.player_mut(id).ok_or(ApplyError::UnknownPlayer(id))?;
        player.pos = destino;
        player.facing = dir;
        Ok(())
    }
}
