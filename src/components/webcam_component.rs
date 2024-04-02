use crate::config;
use crate::contexts::StreamImgContext;
use crate::store::SetStreamImgAction;
use stylist::yew::styled_component;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream,
    MediaStreamConstraints,
};
use yew::platform::spawn_local;
use yew::prelude::*;

#[styled_component(Webcam)]
pub fn webcam() -> Html {
    let window = window().expect("no global `window` exists");
    let video_ref = use_node_ref();
    let canvas_ref = use_node_ref();
    let stream_img = use_context::<StreamImgContext>().unwrap();

    {
        let video_ref = video_ref.clone();
        let canvas_ref = canvas_ref.clone();

        use_effect_with((), move |_| {
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
                web_sys::console::log_1(&format!("Finished setup streaming: {:?}", &stream).into());
            });

            // Move data capturing into background
            let on_interval_handle = Closure::<dyn Fn(Event)>::wrap(Box::new(move |_| {
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
                canvas.set_hidden(true);

                let data = canvas.to_data_url_with_type("image/png").unwrap();
                web_sys::console::debug_1(&format!("finished dispatch an image").into());
                stream_img.dispatch(SetStreamImgAction::SetNewImg(
                    data,
                    video.video_width(),
                    video.video_height(),
                ));
            }));

            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    on_interval_handle.as_ref().unchecked_ref(),
                    config::INTERVAL_MILLISEC,
                )
                .unwrap();

            move || {
                drop(on_interval_handle);
            }
        });
    }

    html! {
        <div class={css!("
                         justify-content: center;
                         align-tiems: center;
                         display: flex;
                         flex-direction: column;
                         padding: 1em;
                         ")}>
            <h3 class={css!("text-align: center;")}>{"Original Video Stream"}</h3>
            <video ref={video_ref} autoplay=true class={css!("width: auto; max-width: 100%; height: auto;")}/>
            <canvas ref={canvas_ref} />
        </div>
    }
}
