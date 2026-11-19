use proptest::prelude::*;
use proptest::sample::select;
use world_core::{Direction, Input, Player, PlayerId, TileMap, TilePos, World};

/// Un mapa al azar de hasta 8x8 y una casilla de inicio que siempre es piso.
fn mapa_con_inicio() -> impl Strategy<Value = (TileMap, TilePos)> {
    (1u16..=8, 1u16..=8).prop_flat_map(|(ancho, alto)| {
        let casillas = usize::from(ancho) * usize::from(alto);
        (
            prop::collection::vec(any::<bool>(), casillas),
            0..ancho,
            0..alto,
        )
            .prop_map(move |(mut bloqueadas, x, y)| {
                let indice = usize::from(y) * usize::from(ancho) + usize::from(x);
                bloqueadas[indice] = false;
                let mapa = TileMap::new(ancho, alto, bloqueadas).expect("ancho x alto casillas");
                (mapa, TilePos::new(x, y))
            })
    })
}

proptest! {
    #[test]
    fn nadie_termina_dentro_de_una_pared(
        (mapa, inicio) in mapa_con_inicio(),
        movimientos in prop::collection::vec(select(&Direction::ALL[..]), 0..50),
    ) {
        let mut world = World::new(mapa);
        world
            .spawn(Player::new(PlayerId(1), "ana", inicio))
            .expect("el inicio siempre es piso");

        for dir in movimientos {
            let _ = world.apply(PlayerId(1), Input::Move(dir));
            let pos = world.player(PlayerId(1)).expect("ana sigue en el mundo").pos;
            prop_assert!(world.tile_map().is_walkable(pos), "ana quedó en {:?}", pos);
        }
    }
}
