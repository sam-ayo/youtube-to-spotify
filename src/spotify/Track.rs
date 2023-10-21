use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackObject {
    pub uri: String,
    pub name: String,
    pub artists: Vec<Artist>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub items: Vec<TrackObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTrackSearchResponse {
    pub tracks: Tracks,
}
