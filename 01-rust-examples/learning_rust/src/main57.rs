// 117. as_ref() and partial move in option and result

fn main() {
    // let some_option = Some("Alice".to_owned());
    // match &some_option {
    //     Some(inner_value) => println!("Name is {}", inner_value),
    //     None => println!("No name provided"),
    // }

    // println!("{:?}", some_option);

    let some_option = Some("Alice".to_owned());
    let some_1 = &some_option;
    let some_2 = some_option.as_ref();

    try_me(some_option.as_ref());
    println!("{:?}", some_option);
}

fn try_me(option_name: Option<&String>) {
    match option_name {
        Some(inner_value) => println!("Name is {}", inner_value),
        None => println!("No name provided"),
    }
}
