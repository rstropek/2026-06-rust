#![allow(unused_variables)]

fn main() {
    let fourtytwo = Some(42);
    let nothing: Option<i32> = None;

    if fourtytwo.is_some_and(|x| x == 42) {
        println!("fourtytwo is Some and equals 42");
    }

    match fourtytwo {
        Some(x) if x == 42 => println!("fourtytwo is Some and equals 42"),
        _ => println!("fourtytwo is not Some or does not equal 42"),
    }

    if let Some(x) = fourtytwo && x == 42 {
        println!("fourtytwo is Some and equals 42");
    }

    fn sqrt_i32(x: i32) -> Option<i32> {
        if x >= 0 {
            Some((x as f64).sqrt() as i32)
        } else {
            None
        }
    }
    let sqrt_42 = fourtytwo.and_then(sqrt_i32);

    let mut option = Some(String::from("ConnectionString=..."));

    /*
    if let Some(s) = option {
        println!("The connection string is: {}", s);
    }
    */

    option.as_ref().map(|s| println!("The connection string is: {}", s));
    option.as_ref().map(|s| println!("The connection string is: {}", s));

    if let Some(s) = option.as_mut() {
        s.push('!');
    }
    println!("{:?}", option);

    option.as_mut().map(|s| s.push('!'));
    println!("{:?}", option);

    let value_in_option = option.take();
    println!("Taken value: {:?}", value_in_option);
    println!("Option after take: {:?}", option);

    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}