#[macro_export]
macro_rules! field_raw {
    ($field: ident, $key: expr, $val: expr) => {
        $field.add_field_method_get($key, |_, _| Ok($val));
    };
}

#[macro_export]
macro_rules! field_raw_str {
    ($field: ident, $key: expr, $val: expr) => {
        $field.add_field_method_get($key, |lua, _| lua.create_string($val));
    };
}

#[macro_export]
macro_rules! field_this {
    ($field: ident, $key: ident) => {
        $field.add_field_method_get(stringify!($key), |_, this| Ok(this.$key));
    };

    ($field: ident, $key: ident, $call: ident) => {
        $field.add_field_method_get(stringify!($key), |_, this| Ok(this.$key.$call()));
    };

    ($field: ident, $key: expr, $val: ident) => {
        $field.add_field_method_get($key, |_, this| Ok(this.$val));
    };

    ($field: ident, $key: expr, $val: ident, $call: ident) => {
        $field.add_field_method_get($key, |_, this| Ok(this.$val.$call()));
    };
}

#[macro_export]
macro_rules! field_this_str {
    ($field: ident, $key: ident) => {
        $field.add_field_method_get(stringify!($key), |lua, this| lua.create_string(&this.$key));
    };

    ($field: ident, $key: ident, $call: ident) => {
        $field.add_field_method_get(stringify!($key), |lua, this| lua.create_string(&this.$key.$call()));
    };

    ($field: ident, $key: expr, $val: ident) => {
        $field.add_field_method_get($key, |lua, this| lua.create_string(&this.$val));
    };

    ($field: ident, $key: expr, $val: ident, $call: ident) => {
        $field.add_field_method_get($key, |lua, this| lua.create_string(&this.$val.$call()));
    };
}

mod common;
mod info;
mod dialogue;
mod style;

pub use common::*;
pub use info::*;
pub use dialogue::*;
pub use style::*;
