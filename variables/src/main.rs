// fn main() {
//     // let は不変のキーワード
//     let x = 5;
//     // let + mutとすることで、可変にすることができる 
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

fn main() {
    // 定数の宣言には、constを使用する
    // 定数の場合、型の定義が必ず必要
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
}
