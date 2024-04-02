pub mod agents;
mod coco_classes;
pub mod components;
mod config;
pub mod contexts;
mod io;
pub mod store;
mod utils;
pub mod yolov8_model;

use agents::InferenceAgent;
use candle_core::Module;
use components::inference_webcam_component::InferenceWebcam;
use components::webcam_component::Webcam;
use components::logos::yewlogo::YewLogo;
use components::logos::linkedin_logo::LinkedInLogo;
use components::logos::github_logo::GithubLogo;
use contexts::StreamImgContext;
use store::StreamImgState;
use stylist::yew::styled_component;
use utils::{annotate_images, get_dyn_image, identify_bboxes, img_to_base64, transform_image};
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yolov8_model::YoloV8;

#[styled_component]
pub fn App() -> Html {
    let stream_img = use_reducer(StreamImgState::default);

    html! {
        <div class={css!("
        background-color: #282c34;
        height: 120%;
        min-height: 120vh;
        color: white;
        margin: 0;
        font-size: calc(10px + 1vmin);
        padding: 0;
        align-items: center;
        text-align: center;
        ")}>
            <h1> {"Real-time object detection with YoloV8"} </h1>
            <h2> {"Built using Yew and WebAssembly in Rust!"} </h2>

            // Diplay logo
            <div class={css!("
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;
                             ")}>
                <img
                    height="120px"
                    src={"https://github.com/carlosbaraza/web-assembly-logo/blob/master/dist/icon/web-assembly-icon-white-512px.png?raw=true"}
                />
                <YewLogo/>
                <img
                    src={"https://assets-global.website-files.com/646dd1f1a3703e451ba81ecc/64994922cf2a6385a4bf4489_UltralyticsYOLO_mark_blue.svg"}
                    height="120px"
                />
            </div>

            // Main webcam components
            <div class={css!("
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;
            flex-wrap: wrap;
            ")}>
                <WorkerProvider<InferenceAgent> path="/worker.js">
                    <ContextProvider<StreamImgContext> context={stream_img}>
                        <Webcam />
                        <InferenceWebcam />
                    </ContextProvider<StreamImgContext>>
                </WorkerProvider<InferenceAgent>>
            </div>
            
            // Footer components
            <div class={css!("
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;
            ")}>
                <div> {"Please follow me on: "} </div>
                <LinkedInLogo />
                <GithubLogo />
            </div>
        </div>
    }
}

pub fn detect_object(
    img: &str,
    shrink_width: f32,
    shrink_height: f32,
    conf_threshold: f32,
    iou_threshold: f32,
    model: &YoloV8,
) -> String {
    let orig_img = get_dyn_image(&img).unwrap();
    let img = transform_image(&img, shrink_width, shrink_height).unwrap();

    web_sys::console::debug_1(&format!("Before model forwarding").into());
    let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
    let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
    let annotated_img =
        annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap()).unwrap();
    web_sys::console::debug_1(&format!("After annotate image").into());
    return img_to_base64(annotated_img).unwrap();
}
