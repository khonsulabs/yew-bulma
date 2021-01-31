use std::borrow::Cow;

pub mod button;
pub mod field;
pub mod label;
pub mod radio;
pub mod storage;
pub mod text_input;

pub mod prelude {
    pub use super::{
        button::Button, field::Field, label::Label, radio::Radio, storage::FormStorage,
        text_input::TextInput, FormField,
    };
    pub use crate::title::Title;
}

pub trait FormField: Copy + std::hash::Hash + Eq + PartialEq + std::fmt::Debug + 'static {
    fn form_id(&self) -> Cow<'static, str>;
}
