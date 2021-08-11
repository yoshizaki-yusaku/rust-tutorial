fn main() {
    // another_function(5, 6);
    // block_scope();
    // print_five();
    print_plus_one();
}

fn print_plus_one() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



// fn print_five() {
//     let x = five();
//     println!("The value of x is:{}", x);
// }

// fn five() -> i32 {
//     5
// }

// fn block_scope() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }