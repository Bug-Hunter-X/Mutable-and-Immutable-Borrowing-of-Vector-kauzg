fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    // Error: Cannot borrow `vec` as mutable because it is also borrowed as immutable
    vec.push(3); 
}