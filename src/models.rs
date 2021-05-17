use mlua::{FromLua, Value, ToLua};
use mlua::prelude::*;

pub struct Style;

impl<'lua> FromLua<'lua> for Style {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        todo!()
    }
}

pub struct KeyFrames;

impl<'lua> ToLua<'lua> for KeyFrames {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        todo!()
    }
}

pub struct ProjectProperties;

impl<'lua> ToLua<'lua> for ProjectProperties {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        todo!()
    }
}