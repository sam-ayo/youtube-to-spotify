struct TrackObject {
    id: String,
}
struct Tracks {
    items: Vec<TrackObject>,
}

pub struct SpotifyTrackSearchResponse {
    tracks: Tracks,
}

trait Convert {
    fn create_playlist(user_id: String) -> bool;
    fn search<T>(song_name: String) -> T;
    fn add_song_to_playlist(song_id: String) -> bool;
}
pub struct Test {}

impl Test {
    pub fn new(a: &str) -> &str {
        a
    }
}
