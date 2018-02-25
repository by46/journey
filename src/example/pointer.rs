use example::common;

pub fn demo() {
    common::line();

    let x = 5;
    let raw = &x as *const i32;

    let mut y = 10;
    let _raw_mut = &mut y as *mut i32;

    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);
}