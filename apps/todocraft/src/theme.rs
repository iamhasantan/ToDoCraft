use eframe::egui::{self, Color32, Visuals};

/// Semantic palette: widgets request roles instead of hard-coded RGB values.
pub struct Tokens {
    pub sidebar_surface: Color32,
    pub accent: Color32,
    pub muted_text: Color32,
    pub border: Color32,
    pub important: Color32,
}

pub fn apply(context: &egui::Context) -> Tokens {
    let mut visuals = Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(25, 29, 35);
    visuals.window_fill = Color32::from_rgb(32, 37, 44);
    visuals.extreme_bg_color = Color32::from_rgb(21, 25, 31);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(38, 44, 52);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(54, 66, 80);
    visuals.selection.bg_fill = Color32::from_rgb(55, 104, 160);
    context.set_visuals(visuals);
    Tokens {
        sidebar_surface: Color32::from_rgb(20, 24, 30),
        accent: Color32::from_rgb(112, 178, 246),
        muted_text: Color32::from_rgb(167, 177, 189),
        border: Color32::from_rgb(61, 69, 80),
        important: Color32::from_rgb(249, 197, 85),
    }
}
