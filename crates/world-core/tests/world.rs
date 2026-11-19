use world_core::{
    ApplyError, Direction, Input, Player, PlayerId, SpawnError, TileMap, TilePos, World,
};

fn mapa_de_ejemplo() -> TileMap {
    TileMap::new(
        5,
        4,
        vec![
            true, true, true, true, true, true, false, false, false, true, true, false, true,
            false, true, true, true, true, true, true,
        ],
    )
    .expect("5 x 4 = 20 casillas")
}

fn mapa_de_ejemplo_3x1() -> TileMap {
    TileMap::new(3, 1, vec![false, false, false]).expect("3 x 1 = 3 casillas")
}

#[test]
fn mover_a_piso() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(1, 1));
    world.spawn(ana).expect("ana entra a un piso");

    let result = world.apply(
        PlayerId(1),
        world_core::Input::Move(world_core::Direction::Right),
    );
    let ana = world.player(PlayerId(1)).expect("ana sigue en el mundo");
    assert!(result.is_ok());
    assert_eq!(ana.pos, TilePos::new(2, 1));
    assert_eq!(ana.facing, Direction::Right);
}

#[test]
fn la_pared_no_cambia_nada() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(1, 1));
    world.spawn(ana).expect("ana entra a un piso");
    let antes = world.clone();
    let result = world.apply(
        PlayerId(1),
        world_core::Input::Move(world_core::Direction::Up),
    );
    assert_eq!(result, Err(ApplyError::Blocked));
    assert_eq!(antes, world);
}

#[test]
fn los_bordes() {
    let mut world = World::new(mapa_de_ejemplo_3x1());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(0, 0));
    world.spawn(ana).expect("ana entra a un piso");

    let result = world.apply(
        PlayerId(1),
        world_core::Input::Move(world_core::Direction::Left),
    );
    assert_eq!(result, Err(ApplyError::Blocked));

    let mut world = World::new(mapa_de_ejemplo_3x1());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(2, 0));
    world.spawn(ana).expect("ana entra a un piso");
    let result = world.apply(
        PlayerId(1),
        world_core::Input::Move(world_core::Direction::Right),
    );
    assert_eq!(result, Err(ApplyError::Blocked));
}

#[test]
fn jugador_desconocido() {
    let mut world = World::new(mapa_de_ejemplo_3x1());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(0, 0));
    world.spawn(ana).expect("ana entra a un piso");

    let result = world.apply(
        PlayerId(9),
        world_core::Input::Move(world_core::Direction::Left),
    );
    assert!(matches!(
        result,
        Err(ApplyError::UnknownPlayer(PlayerId(9)))
    ));
}

#[test]
fn dos_en_la_misma_casilla() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(1, 1));
    let beto: Player = Player::new(world_core::PlayerId(2), "beto", TilePos::new(2, 1));
    world.spawn(ana).expect("ana entra a un piso");
    world.spawn(beto).expect("pedro entra a un piso");

    let result = world.apply(PlayerId(1), Input::Move(world_core::Direction::Right));
    assert!(result.is_ok());
    assert_eq!(
        world
            .player(PlayerId(1))
            .expect("ana sigue en el mundo")
            .pos,
        world
            .player(PlayerId(2))
            .expect("beto sigue en el mundo")
            .pos
    );
}

#[test]
fn spawn_en_pared() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(0, 0));
    let result = world.spawn(ana);
    assert!(matches!(result, Err(SpawnError::NotWalkable(pos)) if pos == TilePos::new(0, 0)));
}

#[test]
fn un_id_repetido_no_reemplaza() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(1, 1));
    let beto: Player = Player::new(world_core::PlayerId(1), "beto", TilePos::new(3, 1));
    world.spawn(ana).expect("ana entra a un piso");
    let resul = world.spawn(beto);
    let ana = world.player(PlayerId(1)).expect("ana sigue en el mundo");

    assert_eq!(resul, Err(SpawnError::DuplicateId(PlayerId(1))));
    assert_eq!(ana.name, "ana");
    assert_eq!(ana.pos, TilePos::new(1, 1));
}

#[test]
fn consultar_un_desconocido() {
    let mut world = World::new(mapa_de_ejemplo());
    let ana: Player = Player::new(world_core::PlayerId(1), "ana", TilePos::new(1, 1));
    world.spawn(ana).expect("ana entra a un piso");
    let player = world.player(PlayerId(9));
    assert!(player.is_none());
}
