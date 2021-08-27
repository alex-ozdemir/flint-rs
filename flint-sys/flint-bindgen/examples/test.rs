include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn main() {
    unsafe {
        let mut x: fmpz = 1;
        println!("{}", x);
    }
}
