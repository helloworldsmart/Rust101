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
}
