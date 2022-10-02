extern crate reqwest;
use gloo::console::log;
extern crate reqwasm;
pub use scraper::{Html as html, Selector};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
const ANIME_URL: &'static str = "https://gogoanime.gold/search.html?keyword=";
pub async fn get_videos(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    // log!("getting video");
    let res = gloo::net::http::Request::get(query)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
pub async fn get_videos_new(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text().unwrap()).await.unwrap();
    let doc = html::parse_document(&text.as_string().unwrap());
    log!("got data of the url");
    let videos = Selector::parse("div#content .wrapper div div strong div article").unwrap();
    log!("got the videos");
    let d = &doc;
    for video in d.select(&videos) {
        // log!("selected the video");
        let video_data = video.text().collect::<String>();
        log!(video_data);
    }
    let vids: Vec<String> = doc
        .select(&videos)
        .into_iter()
        .map(|s| s.text().collect::<String>())
        .collect();
    log!("done");
    Ok(vids)
}
#[derive(Clone, Debug)]
struct Anime {
    title: String,
    url: String,
}
#[test]
fn get_video_test() {
    tokio_test::block_on(async {
        let v = get_videos(&format!("{}{}", ANIME_URL, "one+piece"))
            .await
            .unwrap();
        panic!("{:?}", v);
    });
    assert!(true)
}
