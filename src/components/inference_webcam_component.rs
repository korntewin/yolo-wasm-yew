use crate::agents::InferenceAgent;
use crate::agents::InferenceAgentMessage;
use crate::contexts::StreamImgContext;
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
                web_sys::console::log_1(&format!("agent response: {:?}", &img).into());
                annotated_img.set(img);
            }
            InferenceAgentMessage::InitModel { model_size } => {
                web_sys::console::log_1(&format!("agent response: {:?}", &model_size).into());
                is_loaded.set(true);
            }
        })
    };
    agent_bridge.send(InferenceAgentMessage::StreamImg(stream_img.img.to_owned()));

    use_effect({
        let agent_bridge = agent_bridge.clone();
        // agent_bridge.send(InferenceAgentMessage::InitModel {
        //     model_size: "n".to_string(),
        // });

        move || {
            drop(agent_bridge);
        }
    });

    html! {
        <div>
            <h1> {"Annotated Image"} </h1>
            <img src={ (*annotated_img).clone() } />
        </div>
    }
}
