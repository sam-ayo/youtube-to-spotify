struct TrackObject {
    id: String,
}
struct Tracks {
    items: Vec<TrackObject>,
}

pub struct SpotifyTrackSearchResponse {
    tracks: Tracks,
}

pub trait ConvertPlaylist {
    fn create_playlist(user_id: String) -> bool;
    fn search<T>(song_name: String) -> T;
    fn add_song_to_playlist(song_id: String) -> bool;
}
