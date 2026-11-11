use serde::Deserialize;

pub const COLLISION_LAYER: &str = "colisiones";
pub const MAX_MAP_SIDE: u16 = 1024;
pub const MAX_TMJ_BYTES: usize = 16 * 1024 * 1024; // 16 MiB

#[derive(Deserialize)]
struct RawLayer {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    data: Option<Vec<u32>>,
}

#[derive(Deserialize)]
struct RawMap {
    width: u32,
    height: u32,
    infinite: bool,
    layers: Vec<RawLayer>,
}

#[derive(Debug, thiserror::Error)]
pub enum MapError {
    #[error("el archivo pesa {bytes} bytes; el máximo es {MAX_TMJ_BYTES}")]
    FileTooLarge { bytes: usize },

    #[error("JSON invalid: {0}")]
    Json(#[from] serde_json::Error),

    #[error("los mapas infinitos no estan soportados")]
    Infinite,

    #[error("algun lado vale 0 o pasa de 1024, width: {width}, height: {height}")]
    InvalidSize { width: u32, height: u32 },

    #[error("no hay ninguna capa llama colisiones")]
    MissingCollisionLayer,

    #[error("la capa colisiones existe pero es de otro tipo, por ejemplo {kind}")]
    NoATitleLayer { kind: String },

    #[error("data no tiene width x height numeros")]
    WrongLayerSize { expected: usize, actual: usize },
}

fn find_layer<'a>(layers: &'a [RawLayer], name: &str) -> Option<&'a RawLayer> {
    layers.iter().find(|item| item.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_layer_busca_por_nombre() {
        let raw_layers = vec![
            RawLayer {
                name: String::from("suelo"),
                kind: String::from("tilelayer"),
                data: None,
            },
            RawLayer {
                name: String::from("colisiones"),
                kind: String::from("tilelayer"),
                data: None,
            },
        ];
        let colisiones = find_layer(&raw_layers, "colisiones").expect("La capa existe");
        assert_eq!(colisiones.name, "colisiones");
        assert!(find_layer(&raw_layers, "muros").is_none());
    }
}
