use makepad_widgets::*;
use makepad_component::a2ui::A2uiThemeColors;

// ============================================================================
// Theme System
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Theme {
    #[default]
    DarkPurple,
    Light,
    Soft,
}

impl Theme {
    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            0 => Theme::DarkPurple,
            1 => Theme::Light,
            2 => Theme::Soft,
            _ => Theme::DarkPurple,
        }
    }

    pub(crate) fn to_index(self) -> usize {
        match self {
            Theme::DarkPurple => 0,
            Theme::Light => 1,
            Theme::Soft => 2,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Theme::DarkPurple => "Dark Purple",
            Theme::Light => "Cloud White",
            Theme::Soft => "Soft Gray",
        }
    }
}

pub(crate) struct ThemeColors {
    pub(crate) bg_primary: Vec4,
    pub(crate) bg_surface: Vec4,
    pub(crate) text_primary: Vec4,
    pub(crate) text_secondary: Vec4,
    pub(crate) accent: Vec4,
    pub(crate) accent_secondary: Vec4,
    pub(crate) status_color: Vec4,
}

impl Theme {
    pub(crate) fn colors(self) -> ThemeColors {
        match self {
            Theme::DarkPurple => ThemeColors {
                bg_primary: vec4(0.102, 0.102, 0.180, 1.0),      // #1a1a2e
                bg_surface: vec4(0.133, 0.133, 0.267, 1.0),      // #222244
                text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
                text_secondary: vec4(0.533, 0.533, 0.533, 1.0),  // #888888
                accent: vec4(0.0, 0.4, 0.8, 1.0),                // #0066CC
                accent_secondary: vec4(0.0, 0.667, 0.4, 1.0),    // #00AA66
                status_color: vec4(0.298, 0.686, 0.314, 1.0),    // #4CAF50
            },
            Theme::Light => ThemeColors {
                bg_primary: vec4(0.961, 0.961, 0.969, 1.0),      // #f5f5f7 (iOS-like)
                bg_surface: vec4(1.0, 1.0, 1.0, 1.0),            // #FFFFFF
                text_primary: vec4(0.11, 0.11, 0.118, 1.0),      // #1c1c1e
                text_secondary: vec4(0.557, 0.557, 0.576, 1.0),  // #8e8e93
                accent: vec4(0.0, 0.478, 1.0, 1.0),              // #007AFF (iOS blue)
                accent_secondary: vec4(0.204, 0.78, 0.349, 1.0), // #34C759 (iOS green)
                status_color: vec4(0.204, 0.78, 0.349, 1.0),     // #34C759
            },
            Theme::Soft => ThemeColors {
                // Soft Gray - mid-tone between dark and light
                bg_primary: vec4(0.435, 0.455, 0.490, 1.0),      // #6f7479 (medium gray-blue)
                bg_surface: vec4(0.533, 0.553, 0.588, 1.0),      // #888d96 (lighter gray)
                text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
                text_secondary: vec4(0.85, 0.85, 0.88, 1.0),     // #d9d9e0 (light gray)
                accent: vec4(0.0, 0.4, 0.8, 1.0),                // #0066CC (same blue as Dark Purple)
                accent_secondary: vec4(0.0, 0.667, 0.4, 1.0),    // #00AA66 (same green as Dark Purple)
                status_color: vec4(0.298, 0.686, 0.314, 1.0),    // #4CAF50 (same green)
            },
        }
    }

    /// Get A2UI surface theme colors for this theme
    pub(crate) fn a2ui_colors(self) -> A2uiThemeColors {
        match self {
            Theme::DarkPurple => A2uiThemeColors::dark_purple(),
            Theme::Light => A2uiThemeColors::light(),
            Theme::Soft => A2uiThemeColors::soft(),
        }
    }
}

