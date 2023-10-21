use crate::spotify::track::SpotifyTrackSearchResponse;
#[tokio::main]
pub async fn spotify_track_search(bearer: String, track_name: String) {
    let url = format!("https://api.spotify.com/v1/search/?q={track_name}&type=track&limit=1");
    let auth = format!("Bearer {bearer}");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Authorization", auth)
        .send()
        .await
        .unwrap()
        .json::<SpotifyTrackSearchResponse>()
        .await;

    if let Ok(res) = response {
        println!("URI: {}", res.tracks.items[0].uri);
        println!("Name: {}", res.tracks.items[0].name);
        println!("Artists: {:?}", res.tracks.items[0].artists);
    }
}
