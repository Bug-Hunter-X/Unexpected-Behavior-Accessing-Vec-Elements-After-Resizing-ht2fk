fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // Accessing the second element
    println!("Second element: {}", second_element);
    vec.push(3);
    // Accessing the second element again after pushing a new element
    println!("Second element after push: {}", vec[1]);
}