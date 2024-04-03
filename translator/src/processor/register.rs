use crate::define_register;


define_register!(REG0, 0, "reg0");
define_register!(REG1, 1, "reg1", "reg2");

#[derive(Debug)]
pub struct Register<'a> {
    id: u8,
    representations: &'a[&'a str],
}

#[macro_export]
macro_rules! define_register {
    ($name:ident, $id:expr, $($representations:expr), +) => {
        pub const $name: Register = Register {
            id: $id,
            // representations: vec![$($representations), +],
            representations: &[$($representations), +],
        };
    };
}

impl<'a> Register<'a> {
    pub fn has_name(&self, name: &String) -> bool {
        self.representations
            .into_iter()
            .any(|a| a == name)
    }

    pub fn get_id(&self) -> &u8 {
        &self.id
    }
}