pub mod auto4;
pub mod traits;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::auto4::Auto4;

    #[test]
    fn it_works() {
        let auto4 = Auto4::new().unwrap();
        assert_eq!("2333", auto4.eval_ret_string(r#"aegisub.gettext("2333")"#).unwrap());
    }
}
