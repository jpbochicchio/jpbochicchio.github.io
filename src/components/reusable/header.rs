use yew::prelude::*;
use material_yew::{MatTopAppBar, MatIconButton};

#[function_component(NavHeader)]
pub fn nav_header() -> Html {
    html! {
        <MatTopAppBar center_title=false dense=false>
            <MatIconButton icon="menu" label="Test Label" />
            <div slot="title">{ "Title" }</div>
            <MatIconButton icon="file_download" label="Test Label 2" />
            <MatIconButton icon="print" label="Test Label 3" />
            <MatIconButton icon="favorite" label="Test Label 4" />
        </MatTopAppBar>
    }
}