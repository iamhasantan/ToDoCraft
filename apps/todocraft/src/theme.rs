use eframe::egui::{self, Color32, FontId, Stroke, TextStyle, Visuals};

/// M365-inspired neutral surfaces and restrained blue accent. Original ToDoCraft tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub const fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    pub const fn switch_label(self) -> &'static str {
        match self {
            Self::Light => "Dark mode",
            Self::Dark => "Light mode",
        }
    }
}

/// UI code consumes semantic roles, never a hard-coded palette.
pub struct Tokens {
    pub background: Color32,
    pub sidebar_surface: Color32,
    pub surface: Color32,
    pub accent: Color32,
    pub accent_tint: Color32,
    pub text: Color32,
    pub muted_text: Color32,
    pub border: Color32,
    pub important: Color32,
}

impl Tokens {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self {
                background: Color32::from_rgb(246, 248, 252),
                sidebar_surface: Color32::from_rgb(247, 248, 251),
                surface: Color32::WHITE,
                accent: Color32::from_rgb(20, 100, 178),
                accent_tint: Color32::from_rgb(231, 241, 253),
                text: Color32::from_rgb(36, 41, 47),
                muted_text: Color32::from_rgb(91, 100, 111),
                border: Color32::from_rgb(224, 229, 237),
                important: Color32::from_rgb(148, 96, 13),
            },
            ThemeMode::Dark => Self {
                background: Color32::from_rgb(23, 28, 35),
                sidebar_surface: Color32::from_rgb(28, 33, 41),
                surface: Color32::from_rgb(35, 41, 50),
                accent: Color32::from_rgb(126, 182, 249),
                accent_tint: Color32::from_rgb(38, 59, 84),
                text: Color32::from_rgb(239, 241, 246),
                muted_text: Color32::from_rgb(179, 188, 200),
                border: Color32::from_rgb(67, 75, 87),
                important: Color32::from_rgb(245, 200, 107),
            },
        }
    }
}

pub fn apply(context: &egui::Context, mode: ThemeMode) -> Tokens {
    let tokens = Tokens::for_mode(mode);
    let mut visuals = match mode {
        ThemeMode::Light => Visuals::light(),
        ThemeMode::Dark => Visuals::dark(),
    };
    visuals.panel_fill = tokens.background;
    visuals.window_fill = tokens.surface;
    visuals.extreme_bg_color = tokens.surface;
    visuals.override_text_color = Some(tokens.text);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, tokens.border);
    visuals.widgets.inactive.bg_fill = tokens.surface;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, tokens.border);
    visuals.widgets.hovered.bg_fill = tokens.accent_tint;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, tokens.accent);
    visuals.selection.bg_fill = tokens.accent_tint;
    visuals.selection.stroke = Stroke::new(1.0, tokens.accent);
    context.set_visuals(visuals);

    context.style_mut(|style| {
        // A compact 4-point rhythm, inspired by Fluent's layout and type ramp.
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        style.text_styles.insert(TextStyle::Body, FontId::proportional(14.0));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(14.0));
        style.text_styles.insert(TextStyle::Small, FontId::proportional(12.0));
        style.text_styles.insert(TextStyle::Heading, FontId::proportional(24.0));
    });
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_toggle_is_reversible() {
        assert_eq!(ThemeMode::Light.toggle().toggle(), ThemeMode::Light);
        assert_eq!(ThemeMode::Dark.toggle().toggle(), ThemeMode::Dark);
    }

    #[test]
    fn light_and_dark_palettes_are_distinct() {
        let light = Tokens::for_mode(ThemeMode::Light);
        let dark = Tokens::for_mode(ThemeMode::Dark);
        assert_ne!(light.background, dark.background);
        assert_ne!(light.text, dark.text);
    }
}
