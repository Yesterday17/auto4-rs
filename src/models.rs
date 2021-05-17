use mlua::{FromLua, ToLua};
use mlua::prelude::*;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn to_ass_style_formatted(&self) -> String {
        format!("&H{:2X}{:2X}{:2X}{:2X}", self.a, self.b, self.g, self.r)
    }
}

pub struct ASSTime(u32);

impl ASSTime {
    /// Get millisecond, rounded to centi-second precision
    /// Always round up for 5ms because the range is [start, stop)
    pub fn get(&self) -> u32 {
        (self.0 + 5) - (self.0 + 5) % 10
    }

    pub fn get_ass_formatted(&self, ms_precision: bool) -> String {
        // let mut ret = String::with_capacity(10 + ms_precision);
        todo!()
    }
}

pub struct ASSInfo {
    key: String,
    value: String,
}

impl<'lua> ToLua<'lua> for ASSInfo {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("class", "info")?;
        table.set("section", "[Script Info]")?;

        table.set("raw", format!("{}: {}", self.key, self.value))?;
        table.set("key", self.key)?;
        table.set("value", self.value)?;
        Ok(LuaValue::Table(table))
    }
}

pub struct AssDialogue {
    row: i32,
    /// Is this a comment line?
    comment: bool,
    /// Layer number
    layer: i32,

    margin_left: i32,
    margin_right: i32,
    margin_vertical: i32,

    /// Starting time
    start_time: ASSTime,
    /// Ending time
    end_time: ASSTime,

    /// Style name
    style: String,
    /// Actor name
    actor: String,
    /// Effect name
    effect: String,
    /// IDs of extra data entries for line
    extra_ids: Vec<u32>,
    /// Raw text data
    text: String,
}

impl AssDialogue {
    pub fn entry_data(&self) -> String {
        // TODO
        String::new()
    }
}

impl<'lua> ToLua<'lua> for AssDialogue {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("class", "dialogue")?;
        table.set("section", "[Events]")?;

        table.set("raw", self.entry_data())?;
        table.set("comment", self.comment)?;

        table.set("layer", self.layer)?;

        table.set("start_time", self.start_time.get())?;
        table.set("end_time", self.end_time.get())?;

        table.set("style", self.style)?;
        table.set("actor", self.actor)?;
        table.set("effect", self.effect)?;

        table.set("margin_l", self.margin_left)?;
        table.set("margin_r", self.margin_right)?;
        table.set("margin_t", self.margin_vertical)?;
        table.set("margin_b", self.margin_vertical)?;

        table.set("text", self.text)?;

        // TODO: extra
        let extra = lua.create_table()?;
        table.set("extra", extra)?;

        Ok(LuaValue::Table(table))
    }
}

// https://github.com/Aegisub/Aegisub/blob/6f546951b4f004da16ce19ba638bf3eedefb9f31/src/ass_style.h
pub struct ASSStyle {
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

impl<'lua> ToLua<'lua> for ASSStyle {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("raw", self.raw)?;
        table.set("name", self.name)?;

        table.set("fontname", self.font)?;
        table.set("fontsize", self.font_size)?;

        table.set("color1", self.primary_color.to_ass_style_formatted() + "&")?;
        table.set("color2", self.secondary_color.to_ass_style_formatted() + "&")?;
        table.set("color3", self.outline_color.to_ass_style_formatted() + "&")?;
        table.set("color4", self.shadow_color.to_ass_style_formatted() + "&")?;

        table.set("bold", self.bold)?;
        table.set("italic", self.italic)?;
        table.set("underline", self.underline)?;
        table.set("strikeout", self.strikeout)?;

        table.set("scale_x", self.scale_x)?;
        table.set("scale_y", self.scale_y)?;

        table.set("spacing", self.spacing)?;
        table.set("angle", self.angle)?;

        table.set("borderstyle", self.border_style)?;
        table.set("outline", self.outline_w)?;
        table.set("shadow", self.shadow_w)?;

        table.set("align", self.alignment)?;

        table.set("margin_l", self.margin_left)?;
        table.set("margin_r", self.margin_right)?;
        table.set("margin_t", self.margin_vertical)?;
        table.set("margin_b", self.margin_vertical)?;
        table.set("margin_v", self.margin_vertical)?;

        table.set("encoding", self.encoding)?;

        // From STS.h: "0: window, 1: video, 2: undefined (~window)"
        table.set("relative_to", 2)?;

        table.set("class", "style")?;
        table.set("section", "[V4+ Styles]")?;
        Ok(LuaValue::Table(table))
    }
}

impl<'lua> FromLua<'lua> for ASSStyle {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        todo!()
    }
}

pub struct KeyFrames;

impl<'lua> ToLua<'lua> for KeyFrames {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        todo!()
    }
}

/// https://github.com/Aegisub/Aegisub/blob/master/src/ass_file.h
#[derive(Default, Clone)]
pub struct ProjectProperties {
    automation_scripts: String,
    export_filters: String,
    export_encoding: String,
    style_storage: String,

    audio_file: String,
    video_file: String,
    timecodes_file: String,
    keyframes_file: String,

    video_zoom: f32,
    ar_value: f32,
    scroll_position: i32,
    active_row: i32,
    ar_mode: i32,
    video_position: i32,
}

impl<'lua> ToLua<'lua> for ProjectProperties {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("automation_scripts", self.automation_scripts)?;
        table.set("export_filters", self.export_filters)?;
        table.set("export_encoding", self.export_encoding)?;
        table.set("style_storage", self.style_storage)?;

        // TODO: c->path->MakeAbsolute
        table.set("audio_file", self.audio_file)?;
        table.set("video_file", self.video_file)?;
        table.set("timecodes_file", self.timecodes_file)?;
        table.set("keyframes_file", self.keyframes_file)?;

        table.set("video_zoom", self.video_zoom)?;
        table.set("ar_value", self.ar_value)?;
        table.set("scroll_position", self.scroll_position)?;
        table.set("active_row", self.active_row)?;
        table.set("ar_mode", self.ar_mode)?;
        table.set("video_position", self.video_position)?;
        Ok(LuaValue::Table(table))
    }
}