use world_core::{MAX_TMJ_BYTES, MapError, TileMap, TilePos};
const OFICINA: &str = include_str!("fixtures/oficina.tmj");
const INFINITO: &str = include_str!("fixtures/infinito.tmj");
const DEMASIADO_GRANDE: &str = include_str!("fixtures/demasiado_grande.tmj");
const SIN_COLISIONES: &str = include_str!("fixtures/sin_colisiones.tmj");
const TAMANO_INCORRECTO: &str = include_str!("fixtures/tamano_incorrecto.tmj");
const COLISION_DE_OBJETOS: &str = include_str!("fixtures/colisiones_de_objetos.tmj");

#[test]
fn rechaza_mapa_infinito() {
    let resultado = TileMap::from_tmj(INFINITO);
    assert!(matches!(resultado, Err(MapError::Infinite)));
}

#[test]
fn carga_oficina() {
    let resultado = TileMap::from_tmj(OFICINA).expect("fixture invalido");
    assert_eq!(resultado.width(), 4);
    assert_eq!(resultado.height(), 3);
    assert!(resultado.is_walkable(TilePos::new(1, 1)));
    assert!(!resultado.is_walkable(TilePos::new(0, 0)));
    assert!(!resultado.is_walkable(TilePos::new(1, 2)));
    assert!(resultado.is_walkable(TilePos::new(2, 1)));
    assert!(resultado.is_walkable(TilePos::new(2, 2)));
}

#[test]
fn rechaza_json_invalido() {
    let resultado = TileMap::from_tmj("{");
    assert!(matches!(resultado, Err(MapError::Json(..))));
}

#[test]
fn rechaza_lado_invalido() {
    let resultado = TileMap::from_tmj(DEMASIADO_GRANDE);
    assert!(matches!(
        resultado,
        Err(MapError::InvalidSize {
            width: 5000,
            height: 3
        })
    ));
}

#[test]
fn rechaza_mapa_sin_capa_de_colision() {
    let resultado = TileMap::from_tmj(SIN_COLISIONES);
    assert!(matches!(resultado, Err(MapError::MissingCollisionLayer)));
}

#[test]
fn rechaza_capa_de_tamano_incorrecto() {
    let resultado = TileMap::from_tmj(TAMANO_INCORRECTO);
    assert!(matches!(
        resultado,
        Err(MapError::WrongLayerSize {
            expected: 12,
            actual: 11
        })
    ));
}

#[test]
fn rechaza_archivo_demasiado_grande() {
    let demasiado_grande = " ".repeat(MAX_TMJ_BYTES + 1);
    let resultado = TileMap::from_tmj(&demasiado_grande);
    assert!(matches!(resultado, Err(MapError::FileTooLarge { .. })));
}

#[test]
fn rechaza_capa_de_objetos() {
    let resultado = TileMap::from_tmj(COLISION_DE_OBJETOS);
    assert!(
        matches!(resultado, Err(MapError::NotATileLayer { kind }) if kind == "objectgroup"
        )
    );
}

#[test]
fn el_error_explica_que_falta() {
    let mensaje = TileMap::from_tmj(SIN_COLISIONES).unwrap_err().to_string();
    assert!(mensaje.contains("colisiones"));
}

fn mapa_vacio(ancho: u16, alto: u16) -> String {
    let casillas = usize::from(ancho) * usize::from(alto);
    let data = vec!["0"; casillas].join(",");
    format!(
        r#"{{"infinite": false, "width": {ancho}, "height": {alto},
            "layers": [{{"name": "colisiones", "type": "tilelayer", "data": [{data}]}}]}}"#
    )
}

#[test]
fn carga_mapa_de_256x256() {
    let mapa = TileMap::from_tmj(&mapa_vacio(256, 256)).expect("256x256 es un tamaño valido");
    assert_eq!(mapa.width(), 256);
    assert_eq!(mapa.height(), 256);
}
