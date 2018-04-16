extern crate journey;


fn main() {
    let mut names: Vec<fn()> = Vec::new();
//    names.push(journey::basic::demo);
    //    names.push(journey::example::demo);
    //    names.push(journey::advance::demo);
    //    names.push(journey::http::demo);
    //    names.push(journey::tokio_demo::demo);
    //    names.push(journey::newegg::demo);
//    names.push(journey::api::demo);
    names.push(journey::fs_demo::demo);
    for name in &mut names {
        name()
    }
}