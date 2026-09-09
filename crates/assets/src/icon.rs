use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Styled, Window, svg};

include!(concat!(env!("OUT_DIR"), "/icon_name.rs"));

/// Whether an icon is drawn with the current text color or with its own
/// intrinsic colors.
///
/// Mono icons (the default) use `currentColor` and are tinted by the
/// surrounding text color. Color icons keep their authored `fill`/`stroke`
/// colors, so brand and product marks render faithfully.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum IconColorMode {
    /// Tint the icon with the ambient text color.
    #[default]
    Mono,
    /// Preserve the icon's intrinsic colors.
    Color,
}

/// A named icon that resolves to a path in an application's asset source.
/// Implement this for custom icon sets accepted by GPUI Component's `Icon`.
pub trait IconNamed {
    fn path(self) -> SharedString;

    /// The default rendering mode inferred from the icon's SVG content.
    fn color_mode(&self) -> IconColorMode {
        IconColorMode::Mono
    }
}

impl IconNamed for IconName {
    fn path(self) -> SharedString {
        IconName::path(self)
    }
}

// Keep direct `.child(IconName::Search)` usable by every presentation layer.
// Explicit themed sizes and transformations belong to the consumer's Icon.
impl RenderOnce for IconName {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let text_style = window.text_style();
        svg()
            .path(self.path())
            .flex_shrink_0()
            .size(text_style.font_size.to_pixels(window.rem_size()))
            .text_color(text_style.color)
    }
}

impl From<IconName> for AnyElement {
    fn from(name: IconName) -> Self {
        name.into_any_element()
    }
}
