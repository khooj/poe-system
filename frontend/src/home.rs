use super::make_request::{post_new_build, NewBuild};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use super::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let noderef = use_node_ref();
    let build_id = use_state(|| "".to_string());
    let build_request = {
        let id = build_id.clone();
        use_async(async move {
            post_new_build(NewBuild {
                url: (*id).clone(),
                itemset: "".into(),
            })
            .await
        })
    };

    let onclick = {
        let noderef = noderef.clone();
        let build_id = build_id.clone();
        let build_request = build_request.clone();

        Callback::from(move |ev: yew::events::MouseEvent| {
            ev.prevent_default();
            let node = noderef.cast::<HtmlInputElement>();
            let v = node.map(|node| node.value());
            if v.is_none() {
                log::error!("cant get input value");
                return;
            }

            build_id.set(v.unwrap());
            build_request.run();
        })
    };

    {
        let build_request1 = build_request.clone();
        let br = build_request.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(d) = &build_request1.data {
                    history.push(Route::Build { id: d.clone() });
                }
                || ()
            },
            br.loading,
        );
    }

    html! {
        <div class="container_start">
            <form class="upload" action="" method="post" name="upload_build">
                <label for="upload">{ "Ссылка на поб" }</label>
                <input ref={noderef} type="text" name="pob" />
                <button class="submit" type="submit" {onclick} disabled={build_request.loading}>{ "SEND" }</button>
            </form>
        </div>
    }
}
