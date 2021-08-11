fn main() {
    // let number = 6;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // // rustでは、ifの評価値は必ずbool型でないといけない
    // // Javascriptのように、勝手にboolへは変換されない
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // if number % 4 == 0 {
    //     // 数値は4で割り切れます
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     // 数値は3で割り切れます
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     // 数値は2で割り切れます
    //     println!("number is divisible by 2");
    // } else {
    //     // 数値は4、3、2で割り切れません
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    let condition = true;

    // ifを変数に代入できるが、異なる型を代入することはできない
    let number = if condition {
        5
    } else {
        // "six"
        6
    };

    println!("The value of number is: {}", number);
}
