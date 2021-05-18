use mlua::prelude::*;
use std::rc::Rc;
use crate::models::{KeyFrames, ProjectProperties, Style};
use crate::traits::AegisubAutomation;

type F = String;

pub struct Auto4 {
    lua: Lua,
    properties: ProjectProperties,
}

impl Auto4 {
    pub fn new() -> LuaResult<Rc<Self>> {
        let me = Rc::new(Self { lua: Lua::new(), properties: Default::default() });
        me.clone().create_global()?;
        Ok(me)
    }

    pub fn eval_ret_string(&self, code: &str) -> LuaResult<String> {
        self.lua.load(code).eval::<String>()
    }

    fn create_global(self: Rc<Self>) -> LuaResult<()> {
        let lua = &self.lua;
        let table = lua.create_table()?;
        // auto4_lua
        table.set("lua_automation_version", 4)?;

        let me = self.clone();
        table.set("register_macro", lua.create_function(
            move |_, (name, description, processing_function, validation_function, is_active_function): (String, String, F, Option<F>, Option<F>)|
                Ok(me.register_macro(name, description, processing_function, validation_function, is_active_function)),
        )?)?;
        let me = self.clone();
        table.set("register_filter", lua.create_function(
            move |_, (name, description, priority, processing_function, configuration_panel_provider): (String, String, i32, F, Option<F>)|
                Ok(me.register_filter(name, description, priority, processing_function, configuration_panel_provider))
        )?)?;

        // let me = self.clone();
        // table.set("text_extents", lua.create_function(move |_, (style, text): (Style, String)| Ok(me.text_extents(style, text)))?)?;
        let me = self.clone();
        table.set("gettext", lua.create_function(move |_, untranslated: String| Ok(me.gettext(untranslated)))?)?;
        let me = self.clone();
        table.set("frame_from_ms", lua.create_function(move |_, ms: i32| Ok(me.frame_from_ms(ms)))?)?;
        let me = self.clone();
        table.set("ms_from_frame", lua.create_function(move |_, frame: i32| Ok(me.ms_from_frame(frame)))?)?;
        let me = self.clone();
        table.set("video_size", lua.create_function(move |_, ()| Ok(me.video_size()))?)?;
        let me = self.clone();
        table.set("keyframes", lua.create_function(move |_, ()| Ok(me.keyframes()))?)?;
        let me = self.clone();
        table.set("decode_path", lua.create_function(move |_, encoded_path: String| Ok(me.decode_path(encoded_path)))?)?;
        let me = self.clone();
        table.set("project_properties", lua.create_function(move |_, ()| Ok(me.project_properties()))?)?;

        // table.set("cancel", lua.create_function(todo!())?)?;
        // table.set("__init_clipboard", lua.create_function(todo!())?)?;
        // table.set("file_name", lua.create_function(todo!())?)?;
        // table.set("get_audio_selection", lua.create_function(todo!())?)?;
        // table.set("set_status_text", lua.create_function(todo!())?)?;

        // auto4_lua_assfile
        // table.set("parse_karaoke_data", lua.create_function(todo!())?);
        // table.set("set_undo_point", lua.create_function(todo!())?);

        // auto4_lua_progresssink
        // table.set("progress", todo!());
        // table.set("dialog", todo!());

        // debug
        table.set("_log", lua.create_function(|_, (level, text): (u8, String)| {
            let level = match level {
                0 | 1 => log::Level::Error,
                2 => log::Level::Warn,
                3 => log::Level::Info,
                4 => log::Level::Debug,
                _ => log::Level::Trace,
            };
            log::log!(level, "{}", text);
            Ok(())
        })?)?;

        lua.globals().set("aegisub", table)?;
        lua.load(r#"
        aegisub.out = function(first, second, ...)
            if type(first) ~= "number" then
                aegisub._log(3, string.format(first, second, ...))
            else
                aegisub._log(first, string.format(second, ...))
            end
        end
        aegisub.debug = {}
        aegisub.debug.out = aegisub.out
        "#).exec()?;
        Ok(())
    }
}

impl AegisubAutomation for Auto4 {
    fn register_macro(&self, name: String, description: String, processing_function: F, validation_function: Option<F>, is_active_function: Option<F>) {
        todo!()
    }

    fn register_filter(&self, name: String, description: String, priority: i32, processing_function: F, configuration_panel_provider: Option<F>) {
        todo!()
    }

    fn text_extents(&self, style: Style, text: String) -> (i32, i32, i32, i32) {
        todo!()
    }

    fn frame_from_ms(&self, ms: i32) -> i32 {
        todo!()
    }

    fn ms_from_frame(&self, frame: i32) -> i32 {
        todo!()
    }

    fn video_size(&self) -> (i32, i32, i32, i32) {
        todo!()
    }

    fn keyframes(&self) -> KeyFrames {
        todo!()
    }

    fn decode_path(&self, encoded_path: String) -> String {
        if !encoded_path.starts_with("?") {
            return encoded_path;
        }
        match encoded_path.find("\\") {
            Some(end) => {
                let specifier = &encoded_path[1..end];
                // TODO: paths
                // https://aegi.vmoe.info/docs/3.2/Aegisub_path_specifiers/
                let prefix = match specifier {
                    "data" => "",
                    "user" => "",
                    "temp" => "",
                    "local" => "",
                    "script" => "",
                    "video" => "",
                    "audio" => "",
                    _ => "",
                };
                format!("{}/{}", prefix, &encoded_path[end + 1..])
            }
            None => encoded_path
        }
    }

    fn project_properties(&self) -> ProjectProperties {
        self.properties.clone()
    }
}