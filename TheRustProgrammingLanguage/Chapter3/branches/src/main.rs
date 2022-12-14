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

    // loop {
    //     println!("再一次！");
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("結果為：{}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("升空！！！");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("數值為：{}", a[index]);

        index += 1;
    }

    for element in a {
        println!("數值為：{}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("升空！！！")
}
