<div style="text-align: center;">
<center>
        <h1 style="text-align: center;">NumberComputer is a Library for Number Code</h1>
                
</center>
</div>
------------------------------------------

### This Project Makes Anything with Numbers Much easier to UseM

### I also Added Random Operators for Math As Function so Call Function 
        fn get_op();
-----------------------------
        
    mod objects;
    mod operators;

    #[allow(unused)]
    
        pub fn get_rand_x(n2: f32) -> f32 {
                rand::random_range(1.0..n2)
            }
    
        pub fn get_rand_y(n2: f32) -> f32 {
                
                rand::random_range(1.0..n2)
        }

------------------------------------------
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
--------------------------------

</center>--------------------
