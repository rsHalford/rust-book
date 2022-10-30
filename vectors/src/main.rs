#[derive(Debug)]
enum SpreadsheelCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // println!("The third element is {}.", does_not_exist);

    let hundred = v.get(100);
    match hundred {
        Some(hundred) => println!("The hundredth element is {}.", hundred),
        None => println!("There is no hundredth element."),
    }

    let mut n = vec![100, 32, 57];
    for i in &mut n {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheelCell::Int(3),
        SpreadsheelCell::Text(String::from("blue")),
        SpreadsheelCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }
}
