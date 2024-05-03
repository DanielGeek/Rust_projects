// 24. Borrowing.
/*
    - Borrowing Rules
        - At any time, you can have either one mutable reference or any number of immutable references.
        - References must always be valid.

    - Solve out two problems
        - Data race
        - Dangling references
*/

fn main() {
    let mut vec_1 = vec![4, 5, 6];
    // let ref1 = &mut vec_1;
    // let ref2 = &mut vec_1;
    let ref1 = &vec_1;
    let ref2 = &vec_1;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3 = &mut vec_1;

    let vec_2 = {
        let vec_3 = vec![1, 2, 3];
        &vec_3
    };
}
