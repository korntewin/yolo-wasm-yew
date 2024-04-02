use std::rc::Rc;
use yew::prelude::*;

pub enum SetStreamImgAction {
    SetNewImg(String, u32, u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StreamImgState {
    pub img: String,
    pub width: u32,
    pub height: u32,
}

impl Default for StreamImgState {
    fn default() -> Self {
        Self {
            img: "data:,".to_string(),
            width: 0,
            height: 0,
        }
    }
}

impl Reducible for StreamImgState {
    type Action = SetStreamImgAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SetStreamImgAction::SetNewImg(img, width, height) => Self { img, width, height }.into(),
        }
    }
}
