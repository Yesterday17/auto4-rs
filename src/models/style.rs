use crate::models::common::Color;
use mlua::{UserData, UserDataFields};

// https://github.com/Aegisub/Aegisub/blob/6f546951b4f004da16ce19ba638bf3eedefb9f31/src/ass_style.h
pub struct Style {
    raw: String,

    name: String,
    font: String,
    font_size: u32,

    primary_color: Color,
    secondary_color: Color,
    outline_color: Color,
    shadow_color: Color,

    bold: bool,
    italic: bool,
    underline: bool,
    strikeout: bool,

    /// Font x scale with 100 = 100%
    scale_x: f32,
    /// Font y scale with 100 = 100%
    scale_y: f32,
    /// Additional spacing between characters in pixels
    spacing: f32,
    /// Counterclockwise z rotation in degrees
    angle: f32,

    /// 1: Normal; 3: Opaque box; others are unused in Aegisub
    border_style: u8,
    /// Outline width in pixels
    outline_w: f32,
    /// Shadow distance in pixels
    shadow_w: f32,
    /// \an-style line alignment
    alignment: u8,

    margin_left: i32,
    margin_right: i32,
    margin_vertical: i32,

    /// ASS font encoding needed for some non-unicode fonts
    encoding: i32,
}

impl UserData for Style {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        field_raw_str!(fields, "class", "style");
        field_raw_str!(fields, "section", "[V4+ Styles]");

        field_this_str!(fields, raw);
        field_this_str!(fields, name);

        field_this_str!(fields, "fontname", font);
        field_this!(fields, "fontsize", font_size);

        fields.add_field_method_get("color1", |lua, this| Ok(lua.create_string(&(this.primary_color.to_ass_style_formatted() + "&"))?));
        fields.add_field_method_get("color2", |lua, this| Ok(lua.create_string(&(this.secondary_color.to_ass_style_formatted() + "&"))?));
        fields.add_field_method_get("color3", |lua, this| Ok(lua.create_string(&(this.outline_color.to_ass_style_formatted() + "&"))?));
        fields.add_field_method_get("color4", |lua, this| Ok(lua.create_string(&(this.shadow_color.to_ass_style_formatted() + "&"))?));

        field_this!(fields, bold);
        field_this!(fields, italic);
        field_this!(fields, underline);
        field_this!(fields, strikeout);

        field_this!(fields, scale_x);
        field_this!(fields, scale_y);

        field_this!(fields, spacing);
        field_this!(fields, angle);

        field_this!(fields, "borderstyle", border_style);
        field_this!(fields, "outline", outline_w);
        field_this!(fields, "shadow", shadow_w);

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
