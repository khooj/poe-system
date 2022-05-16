mod home;
mod buildpage;
mod make_request;
mod pob;
mod pob_retriever;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use buildpage::BuildPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Build { id: String }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Build { id } => html! { <BuildPage id={id.clone()} /> }
    }
}

#[function_component(Main)]
fn main_func() -> Html {
    html!{
        <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    } 
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}