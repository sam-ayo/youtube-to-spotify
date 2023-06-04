use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct YoutubePlaylistItem {
    id: String,
    snippet: Snippet,
}
#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct YoutubePlaylistItems {
    items: Vec<YoutubePlaylistItem>,
}

fn get_youtube_playlist_items(
    url: &str,
) -> Result<YoutubePlaylistItems, Box<dyn std::error::Error>> {
    let response: YoutubePlaylistItems =
        reqwest::blocking::get(url)?.json::<YoutubePlaylistItems>()?;
    Ok(response)
}

fn get_url() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = match std::env::var("URL") {
        Ok(url) => url,
        Err(e) => format!("{e}"),
    };
    Ok(url)
}
fn main() {
    let url = get_url().expect("Check your URL");
    let youtube_items =
        get_youtube_playlist_items(&url).expect("Could not get playlist. Check your url");
    for item in youtube_items.items {
        let title = item.snippet.title;
        let description = item.snippet.description;
        println!("----------------------------------------------------------------");
        println!("name of video: {title}\ndescription of the video: {description}\n");
        println!("----------------------------------------------------------------");
    }
}
