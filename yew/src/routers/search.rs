use crate::routers;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

#[function_component(Search)]
pub fn search() -> Html {
    let history = use_history().unwrap();
    let on_click = Callback::from(move |_| {
        history.push(routers::Route::Results);
    });
    html! {}
}
