use crate::agents::InferenceAgent;
use crate::agents::InferenceAgentMessage;
use crate::contexts::StreamImgContext;
use crate::io::download_binary;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::worker::{use_worker_bridge, UseWorkerBridgeHandle};

#[function_component(InferenceWebcam)]
pub fn inference_webcam() -> Html {
    let is_loaded = use_state(|| false);
    let annotated_img = use_state(|| "".to_string());

    let stream_img = use_context::<StreamImgContext>().unwrap();
    let agent_bridge: UseWorkerBridgeHandle<InferenceAgent> = {
        let is_loaded = is_loaded.clone();
        let annotated_img = annotated_img.clone();
        use_worker_bridge(move |response| match response {
            InferenceAgentMessage::StreamImg(img) => {
                web_sys::console::log_1(&format!("Agent finished annotating an image").into());
                annotated_img.set(img);
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

    if *is_loaded && stream_img.img != "data:," {
        // Must not send anything if the model isn't finished loading or
        // if the stream img is empty
        agent_bridge.send(InferenceAgentMessage::StreamImgWithMetadata {
            img: stream_img.img.to_owned(),
            shrink_width: 32. * 6.,
            shrink_height: 32. * 6.,
            conf_threshold: 0.2,
            iou_threshold: 0.4,
        });
        web_sys::console::log_1(&format!("Send an image to the model").into());
    };

    use_effect_with((), move |_| {
        let model_future = download_binary("n".to_string());
        let agent_bridge = agent_bridge.clone();
        spawn_local(async move {
            web_sys::console::log_1(&"Downloading model data".into());
            let model = model_future.await;
            web_sys::console::log_1(&"Finish downloading model data".into());
            agent_bridge.send(InferenceAgentMessage::LoadedModel(model));
        });
    });

    html! {
        <div>
            {if *is_loaded {html!{<h1> {"Annotated Image"} </h1>}} else { html! { <h1> {"Loading Yolo model"} </h1> }} }
            <img src={ (*annotated_img).clone() } />
        </div>
    }
}
