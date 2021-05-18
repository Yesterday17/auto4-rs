use mlua::{UserData, UserDataFields};

pub struct ASSInfo {
    key: String,
    value: String,
}

impl ASSInfo {
    fn entry_data(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

impl UserData for ASSInfo {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("class", |lua, _| Ok(lua.create_string("info")?));
        fields.add_field_method_get("section", |lua, _| Ok(lua.create_string("[Script Info]")?));

        fields.add_field_method_get("raw", |lua, this| Ok(lua.create_string(&this.entry_data())?));
        fields.add_field_method_get("key", |lua, this| Ok(lua.create_string(&this.key)?));
        fields.add_field_method_get("value", |lua, this| Ok(lua.create_string(&this.value)?));
    }
}