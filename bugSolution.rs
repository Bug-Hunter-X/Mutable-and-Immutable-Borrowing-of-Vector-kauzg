fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    // Consume the iterator, removing immutable borrow
    iter.for_each(|_| {});
    vec.push(3);
    println!("Vector after push: {:?}", vec);
} 