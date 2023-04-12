fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);

    const DEMO_CONST: u32 = 100_000;
    println!("The value of constant is: {}", DEMO_CONST);

    let tup = ("string and a number", 100_000);
    let (test_str, test_num) = tup;
    let also_test_num = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

    my_function(23);

    // control flow
    let num = 5;
    if num < 10 {
        println!("one");
    } else if num < 22 {
        println!("two");
    } else {
        println!("three");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("{}", num);

    let mut count = 0;
    let res = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{}", res);

    let mut num = 3;
    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    let a = [10, 20, 30, 40];

    for el in a.iter() {
        println!("{}", el);
    }

    for n in 1..4 {
        println!("{}", n);
    }
}

fn my_function(x: u32) {
    println!("Another function: {}", x);
}
