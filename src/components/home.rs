use yew::prelude::*;

use crate::components::reusable::header::NavHeader;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <NavHeader />
            <div>{"Home"}</div>
        </>
    }
}