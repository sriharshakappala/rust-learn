use std::thread;

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..50000000) {
                _x += 1
            }
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
}

// Usually when the time of compiling Rust will change the name of the function.
// Since we are using it in other functions we do not want it to happen. So we are using #[no_mangle]
// pub and extern are for saying it as a public one and can be accessible by external world.
// By default Rust compiles into rlib.
