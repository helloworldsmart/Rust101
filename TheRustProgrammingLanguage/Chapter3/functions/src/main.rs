fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeld_measurement(5, 'h')
}

fn another_function(x: i32) {
    println!("x 的數值為：{}", x);
}

fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("測量值爲：{}{}", value, unit_label);
}
