fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn example2() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三個元素是 {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("第三個元素是 {third}"),
        None => println!("第三個元素並不存在。"),
    }
}

fn example3() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

fn example4() {
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exit = &v[100];
    // let does_not_exit = v.get(100)
}
