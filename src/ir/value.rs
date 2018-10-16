use super::register::Register;

pub enum Value<'a> {
    Constant(i32),
    Register(&'a Register),
}
