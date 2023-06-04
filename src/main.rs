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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: YoutubePlaylistItems = reqwest::get("https://www.googleapis.com/youtube/v3/playlistItems?key=AIzaSyCpr_wsoiItmkIW6RFMZeu0HDr4d0mIV3w&playlistId=PLTLJ6yJuHHCDaw_rFj_oXnS1JFpyNFJLK&part=snippet")
        .await?
        .json::<YoutubePlaylistItems>()
        .await?;
    for item in resp.items {
        let title = item.snippet.title;
        let description = item.snippet.description;
        println!("----------------------------------------------------------------");
        println!("name of video: {title}\ndescription of the video: {description}\n");
        println!("----------------------------------------------------------------");
    }
    Ok(())
}
