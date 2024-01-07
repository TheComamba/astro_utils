use std::collections::HashMap;

use crate::Float;

struct ParsecLine {
    age: Float,
    log_l: Float,
    log_te: Float,
    log_r: Float,
}

pub struct ParsecData {
    mass_to_data: HashMap<Float,Vec<ParsecLine>>,
}