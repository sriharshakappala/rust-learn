fn main() {
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo(j);
}

fn foo(x: &i32) {
    let y = 10;
    let z = &y;

    baz(z);
    bar(x, z)
}

fn bar(a: &i32, b: &i32) {
    let c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(f: &i32) {
    let g = 100;
}


/*

Address Name Value

2^30 _ 20
.........
.........
// bar()

// baz()
7 g 100
6 f 4
// foo()
5 z 4
4 y 10
3 x 0
// main()
2 j 0
1 i 2^30
0 h 3

*/
