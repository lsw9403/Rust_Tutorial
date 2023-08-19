struct Foo {
    x: i32,
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    f.x = 15;
    println!("{}", f.x);
    
    foo.x = 13;
    println!("{}", foo.x);
}
