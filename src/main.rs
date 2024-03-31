mod components;
mod contexts;
mod store;

use crate::components::inference_webcam_component::InferenceWebcam;
use crate::components::webcam_component::Webcam;
use crate::contexts::StreamImgContext;
use crate::store::StreamImgState;
use yew::prelude::*;

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
            <ContextProvider<StreamImgContext> context={stream_img}>
                <Webcam />
                <InferenceWebcam />
            </ContextProvider<StreamImgContext>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
