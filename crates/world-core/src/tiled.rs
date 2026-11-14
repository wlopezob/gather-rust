use serde::Deserialize;

use crate::TileMap;

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

    #[error("el mapa mide {width}x{height}; cada lado debe estar entre 1 y {MAX_MAP_SIDE}")]
    InvalidSize { width: u32, height: u32 },

    #[error("no hay ninguna capa llama {COLLISION_LAYER}")]
    MissingCollisionLayer,

    #[error("la capa \"{COLLISION_LAYER}\" debe ser tiles, pero es de otro tipo {kind}")]
    NotATileLayer { kind: String },

    #[error("la capa {COLLISION_LAYER} tiene {actual} casillas; se esperaban {expected}")]
    WrongLayerSize { expected: usize, actual: usize },
}

fn find_layer<'a>(layers: &'a [RawLayer], name: &str) -> Option<&'a RawLayer> {
    layers.iter().find(|item| item.name == name)
}

impl TileMap {
    /// Construye el mapa de colisión a partir del contenido de un archivo `.tmj` de Tiled.
    ///
    /// # Errors
    ///
    /// Devuelve un [`MapError`] si el archivo es demasiado grande, no es JSON válido,
    /// el mapa es infinito, sus medidas no son válidas o la capa de colisión falta,
    /// no es de tiles o no tiene `width × height` casillas.
    pub fn from_tmj(json: &str) -> Result<TileMap, MapError> {
        if json.len() > MAX_TMJ_BYTES {
            return Err(MapError::FileTooLarge { bytes: json.len() });
        }
        let raw: RawMap = serde_json::from_str(json)?;
        if raw.infinite {
            return Err(MapError::Infinite);
        }
        if raw.height == 0
            || raw.width == 0
            || raw.width > u32::from(MAX_MAP_SIDE)
            || raw.height > u32::from(MAX_MAP_SIDE)
        {
            return Err(MapError::InvalidSize {
                width: raw.width,
                height: raw.height,
            });
        }
        let width = u16::try_from(raw.width).map_err(|_| MapError::InvalidSize {
            width: raw.width,
            height: raw.height,
        })?;

        let height = u16::try_from(raw.height).map_err(|_| MapError::InvalidSize {
            width: raw.width,
            height: raw.height,
        })?;

        let layer =
            find_layer(&raw.layers, COLLISION_LAYER).ok_or(MapError::MissingCollisionLayer)?;

        let Some(data) = &layer.data else {
            return Err(MapError::NotATileLayer {
                kind: layer.kind.clone(),
            });
        };

        if data.len() != usize::from(width) * usize::from(height) {
            return Err(MapError::WrongLayerSize {
                expected: usize::from(width) * usize::from(height),
                actual: data.len(),
            });
        }
        let blocked = data.iter().map(|&gid| gid != 0).collect();
        TileMap::new(width, height, blocked).ok_or(MapError::InvalidSize {
            width: u32::from(width),
            height: u32::from(height),
        })
    }
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
                name: String::from(COLLISION_LAYER),
                kind: String::from("tilelayer"),
                data: None,
            },
        ];
        let colisiones = find_layer(&raw_layers, COLLISION_LAYER).expect("La capa existe");
        assert_eq!(colisiones.name, COLLISION_LAYER);
        assert!(find_layer(&raw_layers, "muros").is_none());
    }
}
