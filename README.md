# NumberComputer is a Library for Number Code
## This Project Makes Anything with Numbers Much easier to Use


        
    mod objects;
    mod operators;

    #[allow(unused)]
    
    pub fn get_rand_x(n2: f32) -> f32 {
        rand::random_range(1.0..n2)
    }
    
    pub fn get_rand_y(n2: f32) -> f32 {
        rand::random_range(1.0..n2)
    }
