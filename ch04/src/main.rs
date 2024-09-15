
fn first_word(s: &str) -> (&str , usize) {
    let mut i: usize = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b' ' {
            return (&s[0..i], i);
        }
        i += 1;
    }
    (&s[0..s.len()], s.len())
}

fn second_word(s: &str) -> (&str, usize){
    let (_, index) = first_word(s);
    if index >= s.len() - 1 {
        return ("", 0);
    }
    let s = &s[index+1..];

    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return (&s[..i], index + i)
        }
    }
    (&s[..], s.len() + index)
}

fn sort_nums(nums: &mut [f64]) {
    for i in 0..(nums.len()-1) {
        for j in (i+1)..nums.len() {
            if nums[i] > nums[j] {
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            }
        }
    }
}

fn select_larger_than(nums: &[f64], criteria: f64) -> &[f64] {
    for (i, &x) in nums.iter().enumerate() {
        if x > criteria {
            return &nums[i..]
        }
    }
    &nums
}

fn main() {
    let s: (&str, usize) = second_word("Hello,world fuck u!");
    println!("{} {}", s.0, s.1);

    let mut nums: [f64; 6] = [3.2, 4.3, 5.3, 1.2, 2.3, 3.3];

    sort_nums(&mut nums);
    print!("Sorted array: ");
    for x in nums {
        print!("{} ", x);
    }
    let criteria = 3.2;
    let larger_ones = select_larger_than(&nums, criteria);
    print!("\n\nLarger than {}: ", criteria);
    for x in larger_ones {
        print!("{} ", x)
    }
}
