pub mod agents;
mod coco_classes;
pub mod components;
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

// use candle_core::Module;
// pub use io::download_binary;
// use log::log;
// pub use yolov8_model::YoloV8;

// use crate::io::LAZY_MODEL;

// #[wasm_bindgen]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[wasm_bindgen]
// pub fn sum_vec(img: Vec<u8>) -> i32 {
//     img.iter().map(|&x| x as i32).sum()
// }

// #[wasm_bindgen]
// pub fn test_gen_img(img: String, shrink_width: f32, shrink_height: f32) {
//     let _img = transform_image(img, shrink_width, shrink_height).unwrap();
//     log("Finished transform image");
// }

// #[wasm_bindgen]
// pub fn test_lazy_model(img: String, shrink_width: f32, shrink_height: f32) {
//     // log(&format!("Before tranforming image: {:?}", img));
//     let img = transform_image(img, shrink_width, shrink_height).unwrap();
//     // log(&format!("Finished tranformed image: {:?}", img));
//     let maybe_model = LAZY_MODEL.lock().unwrap();
//     log(&format!("Finished locking the model"));

//     if let Some(ref model) = *maybe_model {
//         log(&format!("Before model forwarding"));
//         let tensor = model.forward(&img).unwrap().squeeze(0);
//         log(&format!("After model forwarding"));
//         log(&format!("Tensor: {:?}", tensor));
//     } else {
//         log("Model not loaded");
//     }
// }

// #[wasm_bindgen]
// pub fn test_identify_bboxes(
//     img: String,
//     shrink_width: f32,
//     shrink_height: f32,
//     conf_threshold: f32,
//     iou_threshold: f32,
// ) {
//     let img = transform_image(img, shrink_width, shrink_height).unwrap();
//     let maybe_model = LAZY_MODEL.lock().unwrap();
//     log(&format!("Finished locking the model"));

//     if let Some(ref model) = *maybe_model {
//         log(&format!("Before model forwarding"));
//         let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
//         let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
//         log(&format!("After model forwarding"));
//         log(&format!("Tensor: {:?}", bboxes));
//     } else {
//         log("Model not loaded");
//     }
// }

// #[wasm_bindgen]
// pub fn test_annotate_images(
//     img: String,
//     shrink_width: f32,
//     shrink_height: f32,
//     conf_threshold: f32,
//     iou_threshold: f32,
// ) {
//     let orig_img = get_dyn_image(&img).unwrap();
//     let img = transform_image(img, shrink_width, shrink_height).unwrap();
//     let maybe_model = LAZY_MODEL.lock().unwrap();
//     log(&format!("Finished locking the model"));

//     if let Some(ref model) = *maybe_model {
//         log(&format!("Before model forwarding"));
//         let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
//         let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
//         let _annotated_img =
//             annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap());
//         log(&format!("After annotate image"));
//     } else {
//         log("Model not loaded");
//     }
// }

pub fn js_annotate_images(
    img: String,
    shrink_width: f32,
    shrink_height: f32,
    conf_threshold: f32,
    iou_threshold: f32,
    maybe_model: Option<YoloV8>,
) -> String {
    let orig_img = get_dyn_image(&img).unwrap();
    let img = transform_image(img, shrink_width, shrink_height).unwrap();
    web_sys::console::log_1(&format!("Finished locking the model").into());

    if let Some(ref model) = maybe_model {
        web_sys::console::log_1(&format!("Before model forwarding").into());
        let pred = model.forward(&img).unwrap().squeeze(0).unwrap();
        let bboxes = identify_bboxes(&pred, conf_threshold, iou_threshold);
        let annotated_img =
            annotate_images(orig_img, shrink_width, shrink_height, &bboxes.unwrap()).unwrap();
        web_sys::console::log_1(&format!("After annotate image").into());
        return img_to_base64(annotated_img).unwrap();
    } else {
        web_sys::console::log_1(&"Model not loaded".into());
        return "".to_string();
    }
}
