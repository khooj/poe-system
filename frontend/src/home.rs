use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| history.push(Route::Build { id: "asd".into() }));

    html! {
        <div class="container_start">
            <form class="upload" action="" method="post" name="upload_build">
                <label for="upload">{ "Ссылка на поб" }</label>
                <input type="text" name="pob" />
                <button class="submit" type="submit" {onclick}>{ "SEND" }</button>
            </form>
        </div>
    }
}