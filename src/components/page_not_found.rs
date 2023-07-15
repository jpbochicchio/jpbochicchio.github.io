use yew::prelude::*;

#[function_component(NotFound)]
pub fn page_not_found() -> Html {
    html! {
        <div>{"The file you are looking for was not found"}</div>
    }
}