use gloo::console::log;
use std::cell::Cell;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
pub static mut SEARCH: String = String::new();
const ANIME_URL: &str = "https://gogoanime.gold/search.html?keyword=";
mod request;
mod routers;
enum Msg {
    Submit(String),
    Test,
    Videos(Vec<String>),
    Error(Box<dyn std::error::Error>),
}

struct Model {
    value: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        unsafe {
            log!("created a new model with {} value", SEARCH.clone());
        }
        Self {
            value: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit(o) => {
                self.value = o.clone();
                log!("making request");

                log!("did request");
                true
            }
            Msg::Test => {
                ctx.link().send_future(async move {
                    log!("starting request");
                    log!("done");
                    match request::get_videos("https://quotes.toscrape.com/page/1/").await {
                        Ok(o) => {
                            log!(o);
                            Msg::Videos(vec![String::from("t")])
                        }
                        Err(err) => {
                            log!("something went wrong");
                            Msg::Error(err)
                        }
                    }
                });
                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_input_data = Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input: HtmlInputElement = target
                .expect("error getting input")
                .unchecked_into::<HtmlInputElement>();
            unsafe {
                SEARCH = input.value();
                log!("{}", SEARCH.clone());
            }
        });

        html! {
            <div>
                <div>
                    <p>{format!("{:?}",self.value.clone()).replace(r#"""#,"")}</p>
                </div>
                <form>
                    <div>
                        <input type="text" id="query" onchange={get_input_data}/>
                        <label for="query">{"Search for anime"}</label>
                     </div>
                </form>
                <button  onclick={unsafe {ctx.link().callback(|_| {Msg::Submit(SEARCH.clone())})}}>{"Submit"}</button>
                <div>
                    <button onclick={ctx.link().callback(|_| {Msg::Test})}>{"Test"}</button>
                </div>

                <BrowserRouter>
                    <Switch<routers::Route> render={Switch::render(routers::switch)}/>
                </BrowserRouter>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

fn main() {
    yew::start_app::<Model>();
}
