use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::{
    home::Home, 
    about_me::AboutMe, 
    blog::Blog, 
    page_not_found::NotFound
};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    AboutMe,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::AboutMe => html! { <AboutMe /> },
        Route::Blog => html! { <Blog /> },
        Route::NotFound => html! { <NotFound /> }
    }
}