use crate::contexts::StreamImgContext;
use yew::prelude::*;

#[function_component(InferenceWebcam)]
pub fn inference_webcam() -> Html {
    let stream_img = use_context::<StreamImgContext>().unwrap();
    web_sys::console::log_1(&format!("received stream img: {:?}", &stream_img.img).into());

    html! {
        <div>
            <h1> {"Annotated Image"} </h1>
            <img src={ stream_img.img.to_owned() } />
        </div>
    }
}
