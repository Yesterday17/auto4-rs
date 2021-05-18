use mlua::{UserData, UserDataFields};
use crate::models::common::Time;

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
    start_time: Time,
    /// Ending time
    end_time: Time,

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

impl UserData for AssDialogue {
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
        fields.add_field_method_get("extra", |lua, _| lua.create_table());
    }
}