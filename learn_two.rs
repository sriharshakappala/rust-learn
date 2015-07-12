fn main() {
    let x = 5;
    let y = &x;

    foo(y)
}

fn foo(i: &i32) {
    let z = 42;
}
