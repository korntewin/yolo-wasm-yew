use std::rc::Rc;
use yew::prelude::*;

pub enum SetStreamImgAction {
    SetNewImg(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StreamImgState {
    pub img: String,
}

impl Default for StreamImgState {
    fn default() -> Self {
        Self {
            img: "<empty>".to_string(),
        }
    }
}

impl Reducible for StreamImgState {
    type Action = SetStreamImgAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SetStreamImgAction::SetNewImg(img) => Self { img }.into(),
        }
    }
}
