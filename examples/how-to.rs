extern crate uts2ts;

fn main() {
    let ts = uts2ts::uts2ts(204158100);
    println!("struct: {:?}", ts);
    println!("string: {}", ts.as_string());
}
