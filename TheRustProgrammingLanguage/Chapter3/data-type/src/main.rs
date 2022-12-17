fn main() {
    let guess: u32 = "42".parse().expect("這不是數字");
    
    // error[E0282]: type annotations needed
    // let guess = "42".parse().expect("這不是數字");

    let x = 2.0; // f63

    let y: f32 = 3.0; // f32

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("y 的數值為：{}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    
    let six_point_four = x.1;

    let one = x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let months = ["一月", "二月", "三月", "四月", "五月", "六月", "七月",
    "八月", "九月", "十月", "十一月", "十二月"];

    let b = [3; 5];
    // let b = [3, 3, 3, 3, 3]
}
