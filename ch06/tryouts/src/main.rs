
#[derive(Debug)]
enum IpVersionsNoType {
    V4,
    V6
}

#[derive(Debug)]
struct IpVersionsSimple {
    version: IpVersionsNoType,
    value: String
}

#[derive(Debug)]
enum IpVersions {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Also there's standard library structs for Ipo versions
fn main() {
    let a = IpVersionsNoType::V4;
    let a_struct = IpVersions {version: IpVersionsNoType::V6(String::from("::1"))};
    let b = IpVersions::V4(100, 100, 0, 1);
    
    println!("a = {:?}\n a_s = {:?}\m b = {:?}", a, a_struct, b);
    
}
