mod agents;
mod components;
mod contexts;
mod store;

use agents::InferenceAgent;
use components::inference_webcam_component::InferenceWebcam;
use components::webcam_component::Webcam;
use contexts::StreamImgContext;
use store::StreamImgState;
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let stream_img = use_reducer(StreamImgState::default);

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <WorkerProvider<InferenceAgent> path="/worker.js">
                <ContextProvider<StreamImgContext> context={stream_img}>
                    <Webcam />
                    <InferenceWebcam />
                </ContextProvider<StreamImgContext>>
            </WorkerProvider<InferenceAgent>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
