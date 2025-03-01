mod transmute;

#[derive(Debug)]
struct Foo {}

#[derive(Debug)]
struct Bar {}

fn main() {
    let foo = Foo {};
    let bar: Bar = transmute::transmute(foo);
    dbg!(bar);
}
