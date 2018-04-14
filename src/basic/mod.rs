pub mod collections;
pub mod my_mod;
mod t_tuple;


macro_rules! foo {
    (x=> $e:expr) => (println!("mode X: {}", $e));
    (y=> $e:expr) =>(println!("mode Y: {}", $e));
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );+
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}


pub fn demo() {
    collections::demo();
    my_mod::demo();
    t_tuple::demo();

    foo!(x=> 123);
    println!("{:?}", o_O!(10;[1,3,5];2;[1,2,3]));
}