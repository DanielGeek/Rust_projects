
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in arr.iter().enumerate() {

        if item == target {
            return Some(i); // O(n)
        }
    }

    None
}
fn main() {
    let array = [10, 20, 30, 40, 50];

    let target = 50;

    match linear_search(&array, target) {
        Some(index) => println!("Element {} found at index {}.", target, index),

        None => println!("Element {} not found.", target),
    }
}
