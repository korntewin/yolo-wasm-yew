use serde::{Deserialize, Serialize};
use yew_agent::prelude::*;
use yew_agent::worker::{HandlerId, Worker};

pub struct InferenceAgent {
    model: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceAgentMessage {
    pub stream_img: String,
}

impl Worker for InferenceAgent {
    type Message = ();
    type Input = InferenceAgentMessage;
    type Output = InferenceAgentMessage;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        InferenceAgent { model: 0 }
    }
    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}
    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        web_sys::console::log_1(&format!("agent received stream img: {:?}", &msg).into());
        scope.respond(id, msg);
    }
}
