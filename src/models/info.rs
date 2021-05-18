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
        field_raw_str!(fields, "class", "info");
        field_raw_str!(fields, "section", "[Script Info]");

        fields.add_field_method_get("raw", |lua, this| Ok(lua.create_string(&this.entry_data())?));
        field_this_str!(fields, key);
        field_this_str!(fields, value);
    }
}