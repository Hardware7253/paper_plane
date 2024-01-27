use bevy::prelude::*;

// Color scheme using https://lospec.com/palette-list/sweetie-16 color palette
pub const BACKGROUND_HEX: &'static str = "1a1c2c";
pub const FOREGROUND_HEX: &'static str = "333c57";
pub const TEXT_HEX: &'static str = "f4f4f4";
pub const TEXT_BOLD_HEX: &'static str = "94b0c2";

pub const BUTTON_DEFAULT_HEX: &'static str = FOREGROUND_HEX;
pub const BUTTON_HOVER_HEX: &'static str = "2a3147";
pub const BUTTON_PRESSED_HEX: &'static str = "202536";

pub struct TextStyle {
    pub size: f32,
    pub color_hex: &'static str,
    pub font: &'static str,
    pub allignment: TextAlignment,
}

pub struct ButtonStyle {
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,

    pub width: Val,
    pub height: Val,

    pub color_hex: &'static str,

    pub text_style: Option<TextStyle>,
}


pub const TITLE_TEXT_STYLE: TextStyle = TextStyle {
    size: 96.0,
    color_hex: TEXT_BOLD_HEX,
    font: "fonts/Wonkies.ttf",
    allignment: TextAlignment::Center,
};

pub const BODY_TEXT_STYLE: TextStyle = TextStyle {
    size: 48.0,
    color_hex: TEXT_HEX,
    font: "fonts/Wonkies.ttf",
    allignment: TextAlignment::Center,
};

// Ui Button style
pub const BUTTON_STYLE: ButtonStyle = ButtonStyle {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,

    width: Val::Percent(30.0),
    height: Val::Percent(8.0),

    color_hex: BUTTON_DEFAULT_HEX,

    text_style: Some(BODY_TEXT_STYLE),
};


