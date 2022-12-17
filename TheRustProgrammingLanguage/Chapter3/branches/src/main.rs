fn main() {
    let number = 7;

    if number < 5 {
        println!("條件為真");
    } else {
        println!("條件為否");
    }

    // let number = 3;

    // mismatched types
    // not Javascript
    // if number {
    //     println!("數字為三");
    // }

    let number = 3;

    if number != 0 {
        println!("數字不為零");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("數字可以被 4 整除");
    } else if number % 3 == 0 {
        println!("數字可以被 3 整除");
    } else if number % 2 == 0 {
        println!("數字可以被 2 整除");
    } else {
        println!("數字無法被 4、3、2 整除");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };
    // error[E0308]: `if` and `else` have incompatible types
    // let number = if condition { 5 } else { "六" };

    println!("數字結果為：{}", number);
}
