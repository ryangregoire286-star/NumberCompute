#[allow(unused)]
pub struct Object {
    name: String,
    length: f32,
}

#[allow(unused)]
pub struct NumberTypes<T> {
    pub _number_1: T,
    pub _number_2: T,
}

#[allow(unused)]
pub trait TypesNum {
    fn get_num_1(self);
    fn get_num_2(self);
}
#[allow(unused)]
impl TypesNum for NumberTypes<f32> {
    fn get_num_1(self) {
        self._number_1;
    }

    fn get_num_2(self) {
        self._number_2;
    }
}
