// At a high-level, this is against-the-grain for Rust. You cannot transfer ownership of something
// borrowed because you don't own it. You shouldn't borrow (&Foo) my car and then give it to the
// first person you see on the street! This is still true even if I lend you my car and allow you
// to make changes to it (&mut Foo).
// https://stackoverflow.com/questions/28258548/cannot-move-out-of-borrowed-content-when-trying-to-transfer-ownership?noredirect=1&lq=1

// FIXME: Make me compile! Diff budget: 1 line.
#[derive(Debug, Clone, Copy)]
struct MyType(usize);

// Do not modify this function.
pub fn main() {
    let x = MyType(1);
    let y = &x;
    let z = *y;
    // // OR
    // let z = &*y;
    // println!("x: {:?}  y: {:?}  z: {:?}", x, y, z);
}
