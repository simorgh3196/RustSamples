fn main() {
    // - 変数と可変数

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // - 多重定義

    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x)

    // let spaces = "   ";
    // let spaces: usize = spaces.len();
    // println!("The value of spaces is: {}", spaces)

    // - 浮動小数点

    // let x = 2.0;    // f64
    // let y: f32 = 3.0;  // f32
    // println!("The value of x is: {}, y is: {}", x, y)

    // - 数値演算

    // let sum = 5 + 10;
    // println!("sum: {}", sum);

    // let difference = 95.5 - 4.3;
    // println!("difference: {}", difference);

    // let product = 4 * 30;
    // println!("product: {}", product);

    // let quotient = 56.7 / 32.2;
    // println!("quotient: {}", quotient);

    // let remaider = 43 % 5;
    // println!("remaider: {}", remaider);

    // - 論理値型

    // let t = true;
    // let f: bool = false;
    // println!("The value of t is: {}, f: {}", t, f);

    // - 文字型

    // let c = 'z';  // char型
    // let z = '   ';  // ?
    // let hear_eyed_cat = ' ';  // ?
    // println!("The value of c is: {}, z: {}, hear_eyed_cat: {}", c, z, hear_eyed_cat):
    // println!("The value of c is: {}", c)

    // - タプル型

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of tup is: ({}, {}, {})", x, y, z);

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // println!("The value of five_hundred is: {}, six_point_four: {}, one: {}", 
    //     five_hundred, six_point_four, one);

    // - 配列型

    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //               "August", "September", "October", "November", "December"];
    // println!("{}月: {}", a[0], months[0]);

    // - 関数

    // another_function();

    // another_function2(5, 9);

    // - 文と式

    // let y = 6;  // これは文
    // // let x = (let y = 6); // これはエラー
    // let y = {
    //     let x = 3;
    //     x + 1    // ここにはセミコロンをつけない
    // };  // これは式のためOK
    // println!("The value of y is: {}", y);

    // - 戻り値のある関数

    // let x = five();
    // println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// - 関数

// fn another_function() {
//     println!("Another function.");
// }

// fn another_function2(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// - 戻り値のある関数

// fn five() -> i32 {
//     5
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}
