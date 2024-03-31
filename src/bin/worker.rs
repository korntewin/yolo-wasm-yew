use yew_agent::Registrable;
use yolo_wasm_yew::agents::InferenceAgent;

fn main() {
    InferenceAgent::registrar().register();
}
