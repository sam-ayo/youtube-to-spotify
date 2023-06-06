use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct YoutubePlaylistItem {
    pub snippet: Snippet,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YoutubePlaylistItems {
    pub items: Vec<YoutubePlaylistItem>,
}

impl YoutubePlaylistItems {
    pub fn get_youtube_playlist_items(
        url: &str,
    ) -> Result<YoutubePlaylistItems, Box<dyn std::error::Error>> {
        let response: YoutubePlaylistItems =
            reqwest::blocking::get(url)?.json::<YoutubePlaylistItems>()?;
        Ok(response)
    }
    pub fn get_url() -> Result<String, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let url = match std::env::var("YOUTUBE_URL") {
            Ok(url) => url,
            Err(e) => format!("{e}"),
        };
        Ok(url)
    }
}
