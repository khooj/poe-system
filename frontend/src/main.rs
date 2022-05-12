mod home;
mod buildpage;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use buildpage::BuildPage;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

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
    yew::start_app::<Main>();
}