// use crate::pob::Pob;

use super::make_request::{post_new_build, Error, NewBuild};
// use super::pob_retriever::HttpPobRetriever;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_navigator().unwrap();
    let pob_data_ref = use_node_ref();
    let build_data = use_state(|| "".to_string());

    let build_request = {
        let pob_data_state = build_data.clone();
        let pob_data_ref = pob_data_ref.clone();

        use_async(async move {
            // let pastebin_node = pastebin_link_ref.cast::<HtmlInputElement>();
            // let v = pastebin_node.map(|node| node.value());
            // if v.is_none() {
            //     log::error!("cant get input value");
            //     return Err(Error::CustomError("can't get input value".to_string()));
            // }

            // let v = v.unwrap();

            // let retr = HttpPobRetriever::new();
            // let data = retr.get_pob(&v).await?;

            let pob_data_node = pob_data_ref.cast::<HtmlInputElement>();
            if pob_data_node.is_none() {
                log::error!("can't get pob data ref node");
                return Err(Error::CustomError(
                    "can't get pob data ref node".to_string(),
                ));
            }
            let pob_data_node = pob_data_node.unwrap();
            let data = pob_data_node.value();

            log::info!("pob data2: {}", data);
            pob_data_state.set(data.clone());

            post_new_build(NewBuild {
                pob: data,
                itemset: "".into(),
            })
            .await
        })
    };

    let onclick = {
        let build_request = build_request.clone();

        Callback::from(move |ev: yew::events::MouseEvent| {
            ev.prevent_default();
            build_request.run();
        })
    };

    {
        let build_request1 = build_request.clone();
        let br = build_request.clone();
        let history = history.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(d) = &build_request1.data {
                    history.push(&Route::Build { id: d.clone() });
                }
                || ()
            },
            br.loading,
        );
    }

    let build_id_ref = use_node_ref();
    let onclick_id = {
        let build_id_ref = build_id_ref.clone();
        Callback::from(move |ev: yew::events::MouseEvent| {
            ev.prevent_default();
            let build_id_node = build_id_ref.cast::<HtmlInputElement>();
            if build_id_node.is_none() {
                log::error!("can't get build id ref node");
                return;
            }
            let build_id = build_id_node.unwrap();
            let data = build_id.value();
            history.push(&Route::Build { id: data.clone() });
        })
    };

    html! {
        <div class="container_start">
            <form class="upload" action="" method="post" name="upload_build">
                <label for="pobdata">{ "Данные pob" }</label>
                <input id="pobdata" ref={pob_data_ref} type="text" name="pob" />
                <button class="submit" type="submit" {onclick} disabled={build_request.loading}>{ "SEND" }</button>
                <label for="seebuild">{ "Посмотреть билд" }</label>
                <input id="seebuild" ref={build_id_ref} type="text" name="buildid" />
                <button class="submit" type="submit" onclick={onclick_id}>{ "see" }</button>
            </form>
        </div>
    }
}
