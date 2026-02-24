#[allow(unused)]

struct MathFunc {
    _add: i32,
    _sub: i32,
    _mul: i32,
    _div: i32,
}

impl MathFunc {
    pub fn _add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn _sub(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn _mul(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn _div(a: i32, b: i32) -> i32 {
        a / b
    }

    pub fn _get_pow(a: i32) -> i32 {
        i32::pow(a, 2)
    }
}
