#[cfg(feature = "parser")]
use std::borrow::Cow;

use crate::ast::{Sheet, SheetRef};
use crate::GlobalStyle;
use crate::Style;

/// A struct that can be turned into a [`Style`] or [`GlobalStyle`], it is usually generated by
/// [`css!`](crate::css) macro.
///
/// You can also get a IntoStyle instance from a string or a [`Sheet`] by calling `.into()`.
///
/// ```rust
/// use stylist::IntoStyle;
/// use yew::prelude::*;
/// use stylist::yew::Global;
///
/// let s: IntoStyle = "color: red;".into();
///
/// let rendered = html! {<div class=s.clone() />};
///
/// let global_rendered = html! {<Global css=s />};
/// ```
#[derive(Debug, Clone)]
pub enum IntoStyle {
    #[cfg_attr(documenting, doc(cfg(feature = "parser")))]
    #[cfg(feature = "parser")]
    String(Cow<'static, str>),
    Sheet(SheetRef),
}

impl IntoStyle {
    pub fn to_style(&self) -> Style {
        Style::new(self.to_sheet()).expect("Failed to create style")
    }

    pub fn to_global_style(&self) -> GlobalStyle {
        GlobalStyle::new(self.to_sheet()).expect("Failed to create style")
    }

    pub fn to_sheet(&self) -> SheetRef {
        match self {
            Self::Sheet(ref m) => m.clone(),
            #[cfg_attr(documenting, doc(cfg(feature = "parser")))]
            #[cfg(feature = "parser")]
            Self::String(ref m) => m.parse::<SheetRef>().expect("Failed to parse style"),
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl From<String> for IntoStyle {
    fn from(other: String) -> IntoStyle {
        IntoStyle::String(other.into())
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl From<&'static str> for IntoStyle {
    fn from(other: &'static str) -> IntoStyle {
        IntoStyle::String(other.into())
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl From<Cow<'static, str>> for IntoStyle {
    fn from(other: Cow<'static, str>) -> IntoStyle {
        IntoStyle::String(other)
    }
}

impl From<Sheet> for IntoStyle {
    fn from(other: Sheet) -> IntoStyle {
        IntoStyle::Sheet(other.into())
    }
}

impl From<&'static Sheet> for IntoStyle {
    fn from(other: &'static Sheet) -> IntoStyle {
        IntoStyle::Sheet(other.clone().into())
    }
}

impl From<SheetRef> for IntoStyle {
    fn from(other: SheetRef) -> IntoStyle {
        IntoStyle::Sheet(other)
    }
}
