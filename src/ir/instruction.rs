use super::register::Register;

enum Value {
    Constant(i32),
    Register(Register),
}
