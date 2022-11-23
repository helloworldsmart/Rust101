fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

use num::Complex;

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop { 
        z = z * z + c;
    }
}

// struct Complex<T> {
//     /// 復數的實數部分
//     re: T,

//     /// 復數的虛數部分
//     im: T,
// }

