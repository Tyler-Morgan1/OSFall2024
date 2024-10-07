const FARENHEIT:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let result = (f - FARENHEIT)*5.0/9.0;
    return result;
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let result = c * 9.0/5.0 + 32.0;
    return result;
}


fn main() {

    let mut temp = 104.00;

    let mut x = fahrenheit_to_celsius(temp);
    println!("{}", x);

    for idx in 0..5 {
        temp += 1.0;
        x = fahrenheit_to_celsius(temp);
        println!("{}", x);
    }

    // let mut y = celsius_to_fahrenheit(temp);
    // println!("{}", y);

    // println!("Hello, world!");
}
