fn main() {
    let guess: u32 = "42".parse().expect("這不是數字");
    
    // error[E0282]: type annotations needed
    // let guess = "42".parse().expect("這不是數字");

    let x = 2.0; // f63

    let y: f32 = 3.0; // f32
}
