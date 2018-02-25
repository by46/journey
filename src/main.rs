extern crate journey;


fn main() {
    //    diverges();
    journey::example::flow::if_demo();
    journey::example::flow::for_demo();
    journey::example::basic::tuple_demo();
    journey::example::basic::vector_demo();
    journey::example::basic::slice_demo();
    journey::example::basic::struct_demo();
    journey::example::basic::struct_demo2();

    journey::example::basic::mutable_demo();
    journey::example::basic::life_demo();
    journey::example::basic::struct_demo();
    journey::example::basic::mutable_demo2();
    journey::example::basic::tuple_struct_demo();
    journey::example::basic::match_demo();
    journey::example::basic::enum_demo();

    journey::example::method::method_demo();
    journey::example::string::string_demo();
    journey::example::generic::generic_demo();
    journey::example::traits::demo();
    journey::example::closure::demo();
    journey::example::drop::demo();
    journey::example::trait_object::demo();
    journey::example::func::demo();
    journey::example::association::demo();
    journey::example::overload::demo();
    journey::example::pointer::demo();

    journey::advance::heap::demo();
    journey::advance::concurrency::demo();
    journey::advance::error::demo();
}