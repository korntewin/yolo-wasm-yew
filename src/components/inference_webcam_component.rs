use crate::agents::InferenceAgent;
use crate::agents::InferenceAgentMessage;
use crate::config;
use crate::contexts::StreamImgContext;
use crate::io::download_binary;
use stylist::yew::styled_component;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::worker::{use_worker_bridge, UseWorkerBridgeHandle};

#[styled_component(InferenceWebcam)]
pub fn inference_webcam() -> Html {
    let is_loaded = use_state(|| false);
    let is_processing = use_state(|| false);
    let annotated_img = use_state_eq(|| ("".to_string(), "".to_string()));

    let stream_img = use_context::<StreamImgContext>().unwrap();
    let agent_bridge: UseWorkerBridgeHandle<InferenceAgent> = {
        let is_loaded = is_loaded.clone();
        let annotated_img = annotated_img.clone();
        let is_processing = is_processing.clone();
        use_worker_bridge(move |response| match response {
            InferenceAgentMessage::StreamImg(mdl_annot_img, source_img) => {
                web_sys::console::debug_1(&format!("Agent finished annotating an image").into());
                annotated_img.set((mdl_annot_img, source_img));
                is_processing.set(false);
            }
            InferenceAgentMessage::FinishLoadingModel => {
                web_sys::console::log_1(&format!("Agent finished loading the model").into());
                is_loaded.set(true);
            }
            _ => {
                //skip as there is no need to do anything for this
            }
        })
    };

    let agent_bridge1 = agent_bridge.clone();
    let is_loaded1 = is_loaded.clone();
    let annotated_img_clone = annotated_img.clone();
    let orig_width = stream_img.width;
    let orig_height = stream_img.height;
    web_sys::console::log_1(&format!("original width: {:?}", orig_width).into());
    web_sys::console::log_1(&format!("original height: {:?}", orig_height).into());

    use_effect(move || {
        if *is_loaded1
            && stream_img.img != "data:,"
            && stream_img.img != annotated_img_clone.1
            && !*is_processing
        {
            // Must not send anything if the model isn't finished loading,
            // if the stream img is empty, or if the model is in processing.
            is_processing.set(true);
            agent_bridge1.send(InferenceAgentMessage::StreamImgWithMetadata {
                img: stream_img.img.to_owned(),
                shrink_width: config::SHRINK_WIDTH,
                shrink_height: config::SHRINK_HEIGHT,
                conf_threshold: config::CONF_THRESHOLD,
                iou_threshold: config::IOU_THRESHOLD,
            });
            web_sys::console::debug_1(&format!("Send an image to the model").into());
        } else if *is_processing {
            web_sys::console::debug_1(
                &format!("Model is processing, skip sending an image").into(),
            );
        };
    });

    use_effect_with((), move |_| {
        let model_future = download_binary(config::MODEL_SIZE.to_string());
        let agent_bridge = agent_bridge.clone();
        spawn_local(async move {
            web_sys::console::log_1(&"Downloading model data".into());
            let model = model_future.await;
            web_sys::console::log_1(&"Finish downloading model data".into());
            agent_bridge.send(InferenceAgentMessage::LoadedModel(model));
        });
    });

    html! {
        <div class={css!("
                         justify-content: center;
                         align-tiems: center;
                         display: flex;
                         flex-direction: column;
                         padding: 1em;
                         ")}>
            {
                if *is_loaded
                    {html!{<h3 class={css!("text-align: center;")}> {"Annotated Image"} </h3>}}
                else
                { html! { <h3 class={css!("text-align: center;")}> {"Loading Yolo model"} </h3> }}
            }
            <img
                src={ (*annotated_img).0.clone() }
                width={orig_width.to_string()}
                height={orig_height.to_string()}
                class={css!("width: auto; max-width: 100%; height: auto;")}
            />
        </div>
    }
}
