mod results;
mod search;
mod video;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Search,
    #[at("/results")]
    Results,
    #[at("/video")]
    Video,
}
pub fn switch(route: &Route) -> Html {
    match route {
        Route::Search => return html! {<search::Search/>},
        Route::Results => return html! {<results::Results/>},
        Route::Video => return html! {<video::Video/>},
    }
}
