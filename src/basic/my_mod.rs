use example::common;

fn private_function() {
    println!("called `my_mod:private_function()`")
}

pub fn function() {
    println!("called `my_mod:function")
}

pub fn indirect_access() {
    println!("called `my_mod::indirect_access`");
    private_function();
}

pub mod nested {
    pub fn function() {
        println!("called `my_mod::nested::function()`")
    }

    #[allow(dead_code)]
    fn private_function() {}

    pub(in basic) fn public_function_in_mod() {
        public_function_in_mod();
    }

    pub(self) fn public_function_in_nested() {
        println!("called in my_mod::nested::public_function_in_nested")
    }

    pub(super) fn public_function_in_nested1() {
        public_function_in_nested();
    }
    pub(crate) fn crate_function(){
        nested2::crate_function2();
        self::nested2::crate_function2();
    }
    pub mod nested2{
        pub(super) fn public2(){
            println!("called my_mod::nested::nested2::public2")
        }
        pub(crate) fn crate_function2(){

        }
    }
}

pub fn demo() {
    common::line();
    nested::public_function_in_nested1();
//    nested::nested2::public2();
    nested::nested2::crate_function2();
}