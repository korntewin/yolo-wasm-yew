mod components;

use yew::prelude::*;
use crate::components::webcam_component::Webcam;

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

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <Webcam />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
