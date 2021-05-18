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
        fields.add_field_method_get("class", |lua, _| Ok(lua.create_string("dialogue")?));
        fields.add_field_method_get("section", |lua, _| Ok(lua.create_string("[Events]")?));

        fields.add_field_method_get("raw", |lua, this| Ok(lua.create_string(&this.entry_data())?));
        fields.add_field_method_get("comment", |_, this| Ok(this.comment));
        fields.add_field_method_get("layer", |_, this| Ok(this.layer));

        fields.add_field_method_get("start_time", |_, this| Ok(this.start_time.get()));
        fields.add_field_method_get("end_time", |_, this| Ok(this.end_time.get()));

        fields.add_field_method_get("style", |lua, this| Ok(lua.create_string(&this.style)?));
        fields.add_field_method_get("actor", |lua, this| Ok(lua.create_string(&this.actor)?));
        fields.add_field_method_get("effect", |lua, this| Ok(lua.create_string(&this.effect)?));

        fields.add_field_method_get("margin_l", |_, this| Ok(this.margin_left));
        fields.add_field_method_get("margin_r", |_, this| Ok(this.margin_right));
        fields.add_field_method_get("margin_t", |_, this| Ok(this.margin_vertical));
        fields.add_field_method_get("margin_b", |_, this| Ok(this.margin_vertical));

        fields.add_field_method_get("text", |lua, this| Ok(lua.create_string(&this.text)?));

        // TODO: extra
        fields.add_field_method_get("extra", |lua, _| lua.create_table());
    }
}