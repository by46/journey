use example::common;

struct HasDrop;

struct Firework {
    strength: i32,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("dropping")
    }
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("Boom times {}", self.strength)
    }
}

pub fn demo() {
    common::line();
    let _d = HasDrop;

    let _firecracker = Firework { strength: 1 };
    let _tnt = Firework { strength: 100 };
}