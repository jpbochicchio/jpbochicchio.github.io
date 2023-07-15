use yew_router::prelude::*;
use yew::prelude::*;

mod components;
mod route;

use route::*;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}