#[allow(unused)]

pub struct Operators {
    pub op: Vec<&'static str>,
}

#[allow(unused)]
pub fn get_op() -> &'static str {
    let ran_num = rand::rng();
    let op = Operators {
        op: vec!["+", "-", "*", "/"],
    };

    let els = op.op[rand::random_range(0..=3)];

    els
}
