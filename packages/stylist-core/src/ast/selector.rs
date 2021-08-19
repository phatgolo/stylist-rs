use std::borrow::Cow;
use std::fmt;

use super::ToStyleStr;

/// A CSS Selector.
///
/// E.g.:
/// ```css
/// div[attr="val"].my-class#some-id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selector {
    pub inner: Cow<'static, str>,
}

impl ToStyleStr for Selector {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        // If contains current selector or root pseudo class, replace them with class name.
        if self.inner.contains('&') || self.inner.contains(":root") {
            let scoped_class = format!(".{}", class_name);

            write!(
                w,
                "{}",
                self.inner
                    .replace("&", scoped_class.as_str())
                    .replace(":root", scoped_class.as_str())
            )?;

        // If selector starts with a pseudo-class, apply it to the root element.
        } else if self.inner.starts_with(':') {
            write!(w, ".{}{}", class_name, self.inner)?;

        // For other selectors, scope it to be the children of the root element.
        } else {
            write!(w, ".{} {}", class_name, self.inner)?;
        }

        Ok(())
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for Selector {
    fn from(s: T) -> Self {
        Self { inner: s.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_gen_simple() {
        let s: Selector = ".abc".into();

        assert_eq!(s.to_style_str("stylist-abcdefgh"), ".stylist-abcdefgh .abc");
    }

    #[test]
    fn test_selector_pseduo() {
        let s: Selector = ":hover".into();

        assert_eq!(
            s.to_style_str("stylist-abcdefgh"),
            ".stylist-abcdefgh:hover"
        );
    }

    #[test]
    fn test_selector_root_pseduo() {
        let s: Selector = ":root.big".into();

        assert_eq!(s.to_style_str("stylist-abcdefgh"), ".stylist-abcdefgh.big");
    }

    #[test]
    fn test_selector_gen_current() {
        let s: Selector = "&.big".into();

        assert_eq!(s.to_style_str("stylist-abcdefgh"), ".stylist-abcdefgh.big");
    }
}
