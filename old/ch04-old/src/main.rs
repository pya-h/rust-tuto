
fn prime(which: i64) -> i128 {
        let mut number: i64 = 0;
        let mut i: i128 = 1;
        while number < which {
                i += 1;
                let mut j = 2;
                let end = i / 2;
                while j <= end {
                        if i % j == 0 {
                                break;
                        }
                        j += 1;
                }
                if j > end {
                        number += 1;
                }
        }
        i
}

fn take_ownership(moved_str: String) -> () {
        println!("moved string '{}' is available in take_ownership function, and its not available in main function, anymore...", moved_str)
}

fn give_ownership() -> String {
        let x = String::from("string returned from function to move ownership back...");
        x
}

fn ret_multiple_values(str: String) -> (String, usize) {
	let length = str.len();
	(str, length)
}

fn borrower_len(str: &String) -> usize {
	// now its no need to return str again,
	// to allow main fn to gain access to the sent parameter value.
	str.len()
	// its immutable reference though, this fn cant change str
	// 4ex: str.push_str => error
}

fn extend_str(str: &mut String) -> () {
	str.push_str(str.clone().as_str()); // not cloning `str` causes it to be moved to push_str
	// now str is a mutable reference
	// its borrowed and can be changed at the same time
}
/*
fn dangle_ref() -> &String {
	let s = String::from("dangle ref test string");
	&s
}*/

fn first_word_string(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (index, &element) in bytes.iter().enumerate() {
                if element == b' ' || element == b'\t' {
                        return &s[0..index]; // string slicing just as in python
                        // the difference is, using references, this slices sxactly refer to the same string, not copying it.
                }
        }

        &s[..] // or &str[0..slen] (just like python, but remeber its not copying anything)
}

fn first_word(s: &str) -> &str {
        // this one is more general version
        // it accepts both String and string slices and str literals
        let bytes = s.as_bytes();

        for (idx, &el) in bytes.iter().enumerate() {
                if el == b' ' || el == b'\t' {
                        return &s[..idx];
                }
        }
        &s[..]
}

fn search_after(arr: &[i64], first_item: i64) -> &[i64] {
        for (idx, &el) in arr.iter().enumerate() {
                if el == first_item {
                        return &arr[idx..];
                }
        }
        &arr[..]
}
fn main() {
        let x = prime(100);
        println!("{}\n-‐----------------------------------", x);

        let lst = [10,12,14,20,30];
        for l in lst.iter() {
                println!("{}", l);
        }

        for i in (1..x).rev() {
                println!("{}", i)
        }
        // move test
        let mut s = String::from("test");
        s.push_str("new text");
        println!("{}", s);
        let s2 = s;
//      println!("{} {}", s2, s);
// causes error

        println!("now only s2 is available: {}", s2);

        let s3 = s2.clone();
        println!(".clone prevents moving, s2={}, s3={}", s2, s3);
        let mut sx = s3;
        sx.push_str(" the continue.. ");
        take_ownership(sx);
        // println!("{}", sx); ==> ERROR: value moved error
        let sx2 = give_ownership();
        println!("`{}`", sx2);

        let newstr = String::from("this is a test");
	//let tup = ret_multiple_values(newstr);
	let (ns2, length) = ret_multiple_values(newstr);
	//println!("{}: {}", tup.0, tup.1);

	println!("`{}`: {}", ns2, length);
	// previous function, actually takes and then gives ownership of `newstr`  again
	let blen = borrower_len(&ns2);
	println!("again ns2=`{}`:{} can be accessed, because its borrowed, not moved to the function", ns2, blen);
	let mut mustr = ns2;
	extend_str(&mut mustr);
	println!("ns2 after a mutable borrow: `{}`", mustr);

	// you cannot make immutable reference of a variable that is referenced before
//	let mut sX = String::from("borrow and reference test");
//	let sY = &sX;
//	let sZ = &mut sX;
//	let sR = &mut sX;
//	println!("{} {} {} {}", sX, sY, sZ, sR);
//	let x2 = dangle_ref();
	// reference to freed memory is called dangled rrf and is not allowed

        let word = first_word(&mustr[..]);
        let mut string_from_slice = String::from(word);
        string_from_slice.push_str(word);
        // mustr.clear(); // ERROR: and using referenced slices prevents program from losing the connections; such as connection between firstword and mustr
        println!("first word of mustr is `{}`", word);
        let arr: [i64; 6] = [1,2,3,4,5,6];
        let arr2: &[i64] = search_after(&arr, 4);

        for l in arr2.iter() {
                println!("{}", l)
        }

}
