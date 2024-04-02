use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(YewLogo)]
pub fn yew_logo() -> Html {
    html!(
           <img
               class={css!("
                        height: 160px;
                        padding-left: 10px;
                        padding-right: 20px;
                        @media (prefers-reduced-motion: no-preference) {
                            animation: App-logo-spin infinite 20s linear;
                          }

                        @keyframes App-logo-spin {
                          from {
                            transform: rotate(0deg);
                          }
                          to {
                            transform: rotate(360deg);
                          }
                        }"
               )}
               src={"https://yew.rs/img/logo.svg"}
           />
    )
}
