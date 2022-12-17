fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeld_measurement(5, 'h');

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("y 的數值為：{}", y);

    let a = five();

    println!("a 的數值為：{}", a);

    let b = plus_one(5);

    println!("b 的數值為：{}", b);
}

fn another_function(x: i32) {
    println!("x 的數值為：{}", x);
}

fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("測量值爲：{}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

// if add ;
// error: could not compile `functions` due to previous error
fn plus_one(x: i32) -> i32 {
    x + 1
}
