// fn main() {
//     let tup = (500, 6.4, 1);
//     let (_x, y, _z) = tup;
//     println!("The value of y is: {}", y)
// }

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of x is: {}, y is: {}, z is {}", five_hundred, six_point_four, one);
}
