#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    #[serde(rename(deserialize = "UsedMemory"))]
    used_memory: f64,
    #[serde(rename(deserialize = "TotalMemory"))]
    total_memory: f64,
    #[serde(rename(deserialize = "ProcessLoad"))]
    process_load: f64,
    #[serde(rename(deserialize = "TotalLoad"))]
    total_load: f64,
    #[serde(rename(deserialize = "Cores"))]
    cores: f64,
    #[serde(rename(deserialize = "Threads"))]
    threads: f64,
    #[serde(rename(deserialize = "NetworkInbound"))]
    network_inbound: f64,
    #[serde(rename(deserialize = "NetworkOutbound"))]
    network_outbound: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceResponse {
    pub source: Source,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Source {
    _type: String,
    tag: String,
    pub title: Option<String>,
    pub webpage_url: Option<String>,
    thumbnail: Option<String>,
    duration: f64,
    is_live: bool,
    pub uploader: String,
    description: Option<String>,
    subtitles: serde_json::Value, //idk type
    chapters: serde_json::Value,  //idk type
    related: bool,
}
