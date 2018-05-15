
fn main() {
    let hoge = Some(5);

    match hoge {
        Option::Some(v) => println!("{}", v),
        Option::None => println!("None"),
    }

    let piyo = hoge.map(|s| s * 2);
    println!("{}", piyo.unwrap())
}
