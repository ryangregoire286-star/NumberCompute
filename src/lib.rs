mod m_functions;
mod objects;
mod operators;

#[allow(unused)]

pub fn get_rand_x(n2: f32) -> f32 {
    rand::random_range(1.0..n2)
}

#[allow(unused)]

struct Numbers {
    x: f32,
    y: f32,
}

pub fn get_rand_y(n2: f32) -> f32 {
    let num = Box::new(Numbers { x: 1.0, y: n2 });

    if n2 <= 1.0 as f32 {
        rand::random_range(1.0..num.y)
    } else {
        rand::random_range(1.0..num.x)
    }
}
