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
use contexts::StreamImgContext;
use store::StreamImgState;
use utils::{annotate_images, get_dyn_image, identify_bboxes, img_to_base64, transform_image};
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yolov8_model::YoloV8;

#[function_component]
pub fn App() -> Html {
    let stream_img = use_reducer(StreamImgState::default);

    html! {
        <div>
            <WorkerProvider<InferenceAgent> path="/worker.js">
                <ContextProvider<StreamImgContext> context={stream_img}>
                    <Webcam />
                    <InferenceWebcam />
                </ContextProvider<StreamImgContext>>
            </WorkerProvider<InferenceAgent>>
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

    web_sys::console::log_1(&format!("Before model forwarding").into());
    let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
    let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
    let annotated_img =
        annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap()).unwrap();
    web_sys::console::log_1(&format!("After annotate image").into());
    return img_to_base64(annotated_img).unwrap();
}
