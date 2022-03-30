use std::num::ParseIntError;
use std::str::FromStr;
use mlua::ToLua;
use mlua::prelude::*;
use regex::Regex;

#[derive(Debug, Default)]
pub struct Color(u32);

impl Color {
    pub const DEFAULT_PRIMARY: Color = Color(0xffffff00);
    pub const DEFAULT_SECONDARY: Color = Color(0x00ffff00);
    pub const DEFAULT: Color = Color(0x00000000);

    pub fn new(color: u32) -> Color {
        Color(color)
    }

    pub fn to_ass_style_formatted(&self) -> String {
        format!("&H{:8X}", self.0)
    }
}

impl FromStr for Color {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_ascii_lowercase();
        let mut input = input.trim();

        // skip signs
        loop {
            if input.starts_with("+") || input.starts_with("-") {
                input = &input[1..];
            } else {
                break;
            }
        }

        let base = if s.starts_with("&h") || s.starts_with("0x") {
            input = &input[2..];
            16
        } else {
            10
        };

        u32::from_str_radix(input, base).map(|c| Color::new(c.swap_bytes()))
    }
}

#[derive(Debug, Default)]
pub struct Time(u32);

impl Time {
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

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(?P<h>\d+):(?P<m>\d+):(?P<s>\d+)\.(?P<ms>\d+)$").unwrap();
        let result = regex.captures(s).ok_or(())?;
        let h = result.name("h").unwrap().as_str().parse::<u32>().unwrap();
        let m = result.name("m").unwrap().as_str().parse::<u32>().unwrap();
        let s = result.name("s").unwrap().as_str().parse::<u32>().unwrap();
        let ms = result.name("ms").unwrap().as_str().parse::<u32>().unwrap();
        Ok(Time(((h * 60 + m) * 60 + s) * 1000 + ms * 10))
    }
}

pub struct KeyFrames;

impl<'lua> ToLua<'lua> for KeyFrames {
    fn to_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
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