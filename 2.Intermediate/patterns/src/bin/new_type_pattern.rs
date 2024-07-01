use std::ops::Deref;

struct Inches(u32);


// You can also implement the Deref trait to enable accessing the inner type more conveniently.
impl Deref for Inches {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl Inches {
    fn new(value: u32) -> Self {
        Inches(value)
    }

    fn value(&self) -> u32 {
        self.0
    }
}


fn main() {

    let length = Inches::new(10);
    println!("Length in inches: {}", length.value());
    println!("Length using deref: {}", *length); // Deref allows us to access the inner value directly

}