use std::{thread, time::Duration};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    function: T,
    value: Option<u32>,
    arg: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(function: T) -> Cacher<T> {
        Cacher {
            function,
            value: None,
            arg: None,
        }
    }

    fn get(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(x) => {
                if let Some(a) = self.arg {
                    if a == arg {
                        return x;
                    }
                }
                self.update(arg)
            },
            None => self.update(arg)
        }
    }

    fn update(&mut self, new_arg: u32) -> u32 {
        let v = (self.function)(new_arg);
        self.arg = Some(new_arg);
        self.value = Some(v);
        v
    }
}

struct FeedbackFunc<T> where T: Fn(u64) -> u64 {
    function: T,
    offset: u64
}

impl<T> FeedbackFunc<T> where T: Fn(u64) -> u64 {
    fn new(function: T) -> Self {
        FeedbackFunc { function, offset: 0 }
    }

    fn next(&mut self) -> u64 {
        self.offset = (self.function)(self.offset);
        self.offset
    }
}

fn main() {
    let mut cacher = Cacher::new(|x| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(1));
        x
    });
    println!("f(2) = {}", cacher.get(2));
    println!("f(2) = {}", cacher.get(2));
    println!("f(3) = {}", cacher.get(3));
    println!("f(3) = {}", cacher.get(3));

    let mut primer = FeedbackFunc::new(|offset| {
        let mut next1 = if offset > 1 { offset + 1 } else { 2 };
        loop {
            let end = ((next1 as f32).powf(0.5)) as u64;
            let mut is_prime = true;
            for i in 2..end+1 {
                if next1 % i == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                return next1;
            }
            next1 += 1;
        }
    });

    for i in 1..50 {
        print!("{}, ", primer.next());
    }
}
