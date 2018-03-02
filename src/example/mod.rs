pub mod closure;
pub mod flow;
pub mod basic;
pub mod method;
pub mod string;
pub mod common;
pub mod generic;
pub mod traits;
pub mod drop;
pub mod trait_object;
pub mod func;
pub mod association;
pub mod overload;
pub mod pointer;


pub fn demo() {
    flow::if_demo();
    flow::for_demo();
    basic::tuple_demo();
    basic::vector_demo();
    basic::slice_demo();
    basic::struct_demo();
    basic::struct_demo2();

    basic::mutable_demo();
    basic::life_demo();
    basic::struct_demo();
    basic::mutable_demo2();
    basic::tuple_struct_demo();
    basic::match_demo();
    basic::enum_demo();

    method::method_demo();
    string::string_demo();
    generic::generic_demo();
    traits::demo();
    closure::demo();
    drop::demo();
    trait_object::demo();
    func::demo();
    association::demo();
    overload::demo();
    pointer::demo();
}