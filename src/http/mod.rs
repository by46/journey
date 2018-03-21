use example::common;

pub mod time;
pub mod around;
pub mod content_type;
pub mod hyper_demo;
pub mod tokio2_demo;
pub mod echo;
pub mod connect;

pub fn demo() {
    common::line();
    //    time::demo();
    //    around::demo();
//    content_type::demo();
//    hyper_demo::demo();
//    tokio2_demo::demo();
    echo::demo();
}