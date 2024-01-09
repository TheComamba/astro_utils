use crate::Float;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<u32>,
    description: String,
    unit: String,
    ucd: String,
    utype: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GaiaResponse {
    metadata: Vec<GaiaMetadataLine>,
    data: Vec<Vec<Option<Float>>>,
}
