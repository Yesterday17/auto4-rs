use mlua::{UserData, UserDataMethods, MetaMethod};
use mlua::prelude::*;
use std::fs::read_to_string;
use std::path::Path;

pub struct LuaAssFile {
    can_modify: bool,
    can_set_undo: bool,

    /// How ass file been modified by the script since the last commit
    last_modification_type: i32,
}

impl LuaAssFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        Self::parse(&read_to_string(path).expect("failed to read"))
    }

    pub fn parse(input: &str) -> Self {
        Self {
            can_modify: false,
            can_set_undo: false,
            last_modification_type: 0,
        }
    }
}

impl UserData for LuaAssFile {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: LuaValue| -> LuaResult<LuaValue> {
            match index {
                LuaValue::Integer(index) => {
                    // AssEntryToLua(L, index - 1);
                    // Ok(LuaValue::UserData(lua.create_userdata(this)?))
                    Ok(LuaValue::Integer(0))
                }
                LuaValue::String(field) => {
                    match field.to_str() {
                        Ok("n") => {
                            // return lines.size
                            Ok(LuaValue::Integer(0))
                        }
                        Ok("delete") => todo!(),
                        Ok("deleterange") => todo!(),
                        Ok("insert") => todo!(),
                        Ok("append") => todo!(),
                        Ok("script_resolution") => todo!(),
                        // err Invalid indexing in Subtitle File object
                        Ok(field) => {
                            println!("{}", field);
                            todo!()
                        }
                        Err(_) => todo!()
                    }
                }
                _ => todo!() // err Attempt to index a Subtitle File object with value of type
            }
        });
        methods.add_meta_method(MetaMethod::Len, |lua, this, _: ()| -> LuaResult<LuaInteger> {
            Ok(0)
        });
        // table.set("__newindex", lua.create_function(|_, ()| todo!())?)?;
        // table.set("__gc", lua.create_function(|_, ()| todo!())?)?;
        // table.set("__ipairs", lua.create_function(|_, ()| todo!())?)?;
    }
}

#[cfg(test)]
mod test {
    use mlua::prelude::*;
    use crate::file::LuaAssFile;

    #[test]
    fn test_file() {
        let lua = Lua::new();
        lua.globals().set("file", LuaAssFile::default()).unwrap();
        lua.load(r#"
        print(file.n)
        print(#file)
        "#).exec().unwrap();
    }
}