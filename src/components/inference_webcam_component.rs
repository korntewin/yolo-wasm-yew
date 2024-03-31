use crate::agents::InferenceAgent;
use crate::agents::InferenceAgentMessage;
use crate::contexts::StreamImgContext;
use yew::prelude::*;
use yew_agent::worker::{use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle};

#[function_component(InferenceWebcam)]
pub fn inference_webcam() -> Html {
    let stream_img = use_context::<StreamImgContext>().unwrap();
    let agent_sub = use_worker_subscription::<InferenceAgent>();
    let agent_bridge: UseWorkerBridgeHandle<InferenceAgent> = use_worker_bridge(move |response| {
        web_sys::console::log_1(&format!("agent response: {:?}", &response).into());
    });
    agent_bridge.send(InferenceAgentMessage::StreamImg(stream_img.img.to_owned()));
    // web_sys::console::log_1(&format!("received stream img: {:?}", &stream_img.img).into());
    web_sys::console::log_1(&format!("agent: {:?}", &agent_sub).into());

    html! {
        <div>
            <h1> {"Annotated Image"} </h1>
            <img src={ stream_img.img.to_owned() } />
        </div>
    }
}
