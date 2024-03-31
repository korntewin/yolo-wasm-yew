use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    MediaStreamConstraints,
};
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(Webcam)]
pub fn webcam() -> Html {
    let window = window().expect("no global `window` exists");
    let video_ref = use_node_ref();
    let canvas_ref = use_node_ref();
    let button_ref = use_node_ref();

    {
        let video_ref = video_ref.clone();
        let canvas_ref = canvas_ref.clone();
        let button_ref = button_ref.clone();

        use_effect(move || {
            // Init media device
            let mut media_constraint = MediaStreamConstraints::new();
            media_constraint.video(&JsValue::TRUE);
            let media_devices = window
                .navigator()
                .media_devices()
                .expect("Unable to get media devices");

            // Get user media into Rust future
            let user_media = media_devices
                .get_user_media_with_constraints(&media_constraint)
                .unwrap();
            let user_media_futures = JsFuture::from(user_media);

            let video = video_ref
                .cast::<HtmlVideoElement>()
                .expect("video_ref not attached to video element");
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached to canvas element");
            web_sys::console::debug_1(
                &format!("Finish intializing media device: {:?}", &media_devices).into(),
            );

            // Move streaming to background
            let video_1 = video.clone();
            spawn_local(async move {
                let stream = user_media_futures.await.unwrap();
                let stream = MediaStream::from(stream);
                video_1.set_src_object(Some(&stream));
                web_sys::console::log_1(&stream.into());
            });

            // Move data capturing into background
            let onclick = Closure::<dyn Fn(Event)>::wrap(Box::new(move |_| {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                canvas.set_width(video.video_width());
                canvas.set_height(video.video_height());
                context
                    .draw_image_with_html_video_element_and_dw_and_dh(
                        &video,
                        0.,
                        0.,
                        video.video_width() as f64,
                        video.video_height() as f64,
                    )
                    .unwrap();

                let data = canvas.to_data_url_with_type("image/png").unwrap();
                web_sys::console::log_1(&data.into());
            }));

            let button = button_ref
                .cast::<web_sys::HtmlButtonElement>()
                .expect("button_ref not attached to button element");
            button.set_onclick(Some(onclick.as_ref().unchecked_ref()));

            move || {
                button
                    .remove_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }

    html! {
        <div>
            <h1>{"Original Video Stream"}</h1>
            <video ref={video_ref} autoplay=true />
            <h1> {"Captured video stream"} </h1>
            <canvas ref={canvas_ref} />
            <button ref={button_ref} > {"take a picture"} </button>
        </div>
    }
}
