use std::io;

fn get_number() -> f64 {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number of v2 items!");
    match input.trim().parse::<f64>() {
        Ok(value) => {
            return value;
        }
        Err(..) => println!("Failed to read float."),
    }
    0.0
}

enum VecVariant {
    Int(i64),
    Float(f64),
    Text(String)
}

impl VecVariant {
    fn value(&self) -> String {
        return match self {
            VecVariant::Int(v) => format!("Integer {}", v),
            VecVariant::Float(v) => format!("Float {}", v),
            VecVariant::Text(v) => format!("Text {}", v)
        }
    }
}

fn main() {
    let v1: Vec<i32> = vec![10, 20, 30];

    for item in &v1 {
        println!("{}", item)
    }

    let mut v2: Vec<f64> = Vec::new();

    let length = get_number() as i64;

    for _i in 0..length {
        v2.push(get_number());
    }

    for (i, &item) in v2.iter().enumerate() {
        println!("v2#{}: {}", i, item)
    }

    for item in &mut v2 {
        *item *= 2.0;
        println!("{}", item);
    }

    let mut vx: Vec<VecVariant> = Vec::new();

    vx.push(VecVariant::Int(10));
    vx.push(VecVariant::Text(String::from("This is a test text")));
    vx.push(VecVariant::Float(10.2));

    for item in &mut vx {
        println!("{}", item.value())
    }
    
}
