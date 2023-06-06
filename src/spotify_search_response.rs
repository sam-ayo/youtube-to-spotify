struct TrackObject {
    id: String,
}
struct Tracks {
    items: Vec<TrackObject>,
}

pub struct SpotifyTrackSearchResponse {
    tracks: Tracks,
}
