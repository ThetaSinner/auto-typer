use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub version: String,
    pub stages: Vec<Stage>,
}

#[derive(Deserialize)]
pub struct Stage {
    pub order: u32,
    pub input: String,
}

#[derive(Deserialize)]
pub struct Start {
    pub line: u32,
    pub char: u32,
}
