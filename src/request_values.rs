use crate::response_values::Source;

#[derive(Debug, serde::Serialize)]
pub struct Seek {
    pub offset: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct Skip {
    pub offset: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct SetAutoplay {
    pub autoplay: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct Unmark {
    pub address: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoadSource {
    pub query: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SetVolume {
    pub volume: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct SetCrossfade {
    pub crossfade: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct SetGapless {
    pub gapless: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct PutSoucre {
    pub source: Source,
}

#[derive(Debug, serde::Serialize)]
pub struct Nothing;
