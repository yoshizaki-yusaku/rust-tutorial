fn main() {
    // // 無限ループの書き方
    // loop {
    //     println!("again!")
    // }

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // // 発射!
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50, 60];

    // for element in a.iter() {
    //     println!("the value is:{}", element);
    // }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!")
}
