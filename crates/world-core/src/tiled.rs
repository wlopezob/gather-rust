use serde::Deserialize;

pub const COLLISION_MAYER: &str = "colisiones";
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
    #[error("El archivo pesa {bytes} bytes; el maximo es {MAX_TMJ_BYTES}")]
    FileTooLarge { bytes: usize },

    #[error("JSON invalid {0}")]
    Json(#[from] serde_json::Error),
}

fn find_layer<'a>(layers: &'a [RawLayer], name: &str) -> Option<&'a RawLayer> {
    layers.iter().find(|item| item.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_layer_busca_por_nombre() {
        let raw_layers: Vec<RawLayer> = vec![
            RawLayer {
                name: String::from("suelo"),
                kind: String::from(""),
                data: None,
            },
            RawLayer {
                name: String::from("colisiones"),
                kind: String::from(""),
                data: None,
            },
        ];
        assert!(find_layer(&raw_layers, "colisiones").is_some());
        assert!(find_layer(&raw_layers, "muros").is_none());
    }
}
