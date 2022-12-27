use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub(crate) version: String,
    pub(crate) stages: Vec<Stage>,
}

#[derive(Deserialize)]
pub struct Stage {
    pub(crate) order: u32,
    pub(crate) input: String,
}

#[derive(Deserialize)]
pub struct Start {
    pub(crate) line: u32,
    pub(crate) char: u32,
}
