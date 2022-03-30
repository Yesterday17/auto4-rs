use crate::models::common::Color;
use mlua::{UserData, UserDataFields};

// https://github.com/Aegisub/Aegisub/blob/6f546951b4f004da16ce19ba638bf3eedefb9f31/src/ass_style.h
#[derive(Debug)]
pub struct Style {
    pub raw: String,

    pub name: String,
    pub font: String,
    pub font_size: f32,

    pub primary_color: Color,
    pub secondary_color: Color,
    pub outline_color: Color,
    pub shadow_color: Color,

    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,

    /// Font x scale with 100 = 100%
    pub scale_x: f32,
    /// Font y scale with 100 = 100%
    pub scale_y: f32,
    /// Additional spacing between characters in pixels
    pub spacing: f32,
    /// Counterclockwise z rotation in degrees
    pub angle: f32,

    /// 1: Normal; 3: Opaque box; others are unused in Aegisub
    pub border_style: u8,
    /// Outline width in pixels
    pub outline: f32,
    /// Shadow distance in pixels
    pub shadow: f32,
    /// \an-style line alignment
    pub alignment: u8,

    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_vertical: i32,

    /// ASS font encoding needed for some non-unicode fonts
    pub encoding: i32,
}

impl UserData for Style {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        field_raw_str!(fields, "class", "style");
        field_raw_str!(fields, "section", "[V4+ Styles]");

        field_this_str!(fields, raw);
        field_this_str!(fields, name);

        field_this_str!(fields, "fontname", font);
        field_this!(fields, "fontsize", font_size);

        field_this_str!(fields, "color1", primary_color, to_ass_style_formatted);
        field_this_str!(fields, "color2", secondary_color, to_ass_style_formatted);
        field_this_str!(fields, "color3", outline_color, to_ass_style_formatted);
        field_this_str!(fields, "color4", shadow_color, to_ass_style_formatted);

        field_this!(fields, bold);
        field_this!(fields, italic);
        field_this!(fields, underline);
        field_this!(fields, strikeout);

        field_this!(fields, scale_x);
        field_this!(fields, scale_y);

        field_this!(fields, spacing);
        field_this!(fields, angle);

        field_this!(fields, "borderstyle", border_style);
        field_this!(fields, "outline", outline);
        field_this!(fields, "shadow", shadow);

        field_this!(fields, "align", alignment);

        field_this!(fields, "margin_l", margin_left);
        field_this!(fields, "margin_r", margin_right);
        field_this!(fields, "margin_t", margin_vertical);
        field_this!(fields, "margin_b", margin_vertical);
        field_this!(fields, "margin_v", margin_vertical);

        field_this!(fields, encoding);

        // From STS.h: "0: window, 1: video, 2: undefined (~window)"
        field_raw!(fields, "relative_to", 2);
    }
}
