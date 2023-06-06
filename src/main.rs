mod spotify_search_response;
use spotify_search_response::SpotifyTrackSearchResponse;

mod youtube_playlist_item;
use youtube_playlist_item::YoutubePlaylistItems;

mod url;

fn main() {
    let url = YoutubePlaylistItems::get_url().expect("Check your URL");
    let youtube_items = YoutubePlaylistItems::get_youtube_playlist_items(&url)
        .expect("Could not get playlist. Check your url");
    println!("{:?}", youtube_items);
    for item in youtube_items.items {
        let title = item.snippet.title;
        let description = item.snippet.description;
        println!("----------------------------------------------------------------");
        println!("name of video: {title}\ndescription of the video: {description}\n");
        println!("----------------------------------------------------------------");
    }
}
