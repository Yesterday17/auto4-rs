use mlua::{UserData, UserDataFields};
use super::Time;

#[derive(Debug)]
pub struct Event {
    pub raw: String,

    pub comment: bool,
    pub layer: i32,

    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_vertical: i32,

    pub start_time: Time,
    pub end_time: Time,

    pub style: String,
    pub actor: String,
    pub effect: String,

    pub text: String,
}

impl Event {
    pub fn entry_data(&self) -> String {
        // TODO
        String::new()
    }
}

impl UserData for Event {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        field_raw_str!(fields, "class", "dialogue");
        field_raw_str!(fields, "section", "[Events]");

        fields.add_field_method_get("raw", |lua, this| Ok(lua.create_string(&this.entry_data())?));
        field_this!(fields, comment);
        field_this!(fields, layer);

        field_this!(fields, start_time, get);
        field_this!(fields, end_time, get);

        field_this_str!(fields, style);
        field_this_str!(fields, actor);
        field_this_str!(fields, effect);

        field_this!(fields, "margin_l", margin_left);
        field_this!(fields, "margin_r", margin_right);
        field_this!(fields, "margin_t", margin_vertical);
        field_this!(fields, "margin_b", margin_vertical);

        field_this_str!(fields, text);

        // TODO: extra
        // https://github.com/Aegisub/Aegisub/blob/6f546951b4f004da16ce19ba638bf3eedefb9f31/src/auto4_lua_assfile.cpp#L184-L191
        fields.add_field_method_get("extra", |lua, _| lua.create_table());
    }
}