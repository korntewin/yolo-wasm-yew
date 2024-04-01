use crate::detect_object;
use crate::io::{load_model_from_data, ModelData};
use crate::yolov8_model::YoloV8;
use serde::{Deserialize, Serialize};
use yew_agent::prelude::*;
use yew_agent::worker::{HandlerId, Worker};

pub struct InferenceAgent {
    model: Option<YoloV8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InferenceAgentMessage {
    StreamImg(String),
    StreamImgWithMetadata {
        img: String,
        shrink_width: f32,
        shrink_height: f32,
        conf_threshold: f32,
        iou_threshold: f32,
    },
    LoadedModel(ModelData),
    FinishLoadingModel,
}

impl Worker for InferenceAgent {
    type Message = ();
    type Input = InferenceAgentMessage;
    type Output = InferenceAgentMessage;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        InferenceAgent { model: None }
    }
    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}
    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        match msg {
            InferenceAgentMessage::StreamImgWithMetadata {
                img,
                shrink_width,
                shrink_height,
                conf_threshold,
                iou_threshold,
            } => {
                web_sys::console::log_1(&format!("agent received stream img: {:?}", &img).into());
                if let Some(mdl) = &self.model {
                    let annotated_img = detect_object(
                        img,
                        shrink_width,
                        shrink_height,
                        conf_threshold,
                        iou_threshold,
                        mdl,
                    );
                    scope.respond(id, InferenceAgentMessage::StreamImg(annotated_img));
                } else {
                    web_sys::console::log_1(&format!("Model is not loaded yet").into());
                }
            }
            InferenceAgentMessage::LoadedModel(model_data) => {
                web_sys::console::log_1(&format!("Agent received model data").into());
                let model = load_model_from_data(model_data);
                self.model.replace(model);
                web_sys::console::log_1(&format!("Agent finish loading model into memory").into());
                scope.respond(id, InferenceAgentMessage::FinishLoadingModel);
            }
            _ => {}
        }
    }
}
