//! Yew integration module.

use crate::Style;
use yew::html::Classes;

#[cfg_attr(documenting, doc(cfg(feature = "yew")))]
impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}
