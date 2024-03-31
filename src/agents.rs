use crate::io::get_model;
use crate::yolov8_model::YoloV8;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use yew::platform::spawn_local;
use yew_agent::prelude::*;
use yew_agent::worker::{HandlerId, Worker};

pub struct InferenceAgent {
    model: Arc<Mutex<Option<YoloV8>>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InferenceAgentMessage {
    StreamImg(String),
    InitModel { model_size: String },
}

impl Worker for InferenceAgent {
    type Message = ();
    type Input = InferenceAgentMessage;
    type Output = InferenceAgentMessage;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        InferenceAgent {
            model: Arc::new(Mutex::new(None)),
        }
    }
    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}
    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        match msg {
            InferenceAgentMessage::StreamImg(img) => {
                web_sys::console::log_1(&format!("agent received stream img: {:?}", &img).into());
                scope.respond(id, InferenceAgentMessage::StreamImg(img));
            }
            InferenceAgentMessage::InitModel { model_size } => {
                let model_future = get_model(model_size.clone());
                let scope = scope.clone();
                let model_clone = self.model.clone();
                spawn_local(async move {
                    let model = model_future.await;
                    *model_clone.lock().unwrap() = Some(model);
                    web_sys::console::log_1(
                        &format!("finish init yolo model: {:?}", &model_size).into(),
                    );
                    scope.respond(id, InferenceAgentMessage::InitModel { model_size });
                });
            }
        }
    }
}
