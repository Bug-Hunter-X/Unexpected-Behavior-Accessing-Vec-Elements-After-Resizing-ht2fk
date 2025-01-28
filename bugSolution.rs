fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // Accessing the second element before modification
    println!("Second element: {}", second_element);
    vec.push(3);
    // Accessing the second element using get() after push; this safely handles the possibility of an index out of bounds.
    match vec.get(1) {
        Some(value) => println!("Second element after push: {}", value),
        None => println!("Index out of bounds")
    }
} 