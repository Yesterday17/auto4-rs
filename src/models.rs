use mlua::{FromLua, ToLua};
use mlua::prelude::*;

pub struct Style;

impl<'lua> FromLua<'lua> for Style {
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