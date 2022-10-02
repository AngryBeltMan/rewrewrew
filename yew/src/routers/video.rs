use yew::prelude::*;
const TESTVIDEO: &str = "https://mplayer.sbs/default.php?id=MTkyNzQ4";
#[function_component(Video)]
pub fn video() -> Html {
    html! {
        <iframe src={TESTVIDEO}/>
    }
}
