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

// Example: Strong Typing with Newtype
struct UserId(u32);
struct OrderId(u32);

fn find_user_by_id(id: UserId) {
    // function implementation
}

fn find_order_by_id(id: OrderId) {
    // function implementation
}
fn main() {

    let length = Inches::new(10);
    println!("Length in inches: {}", length.value());
    println!("Length using deref: {}", *length); // Deref allows us to access the inner value directly

    // In this example, UserId and OrderId are both based on u32,
    // but they are distinct types and cannot be interchanged, providing stronger type safety.
    let user_id = UserId(42);
    let order_id = OrderId(99);

    find_user_by_id(user_id);
    find_order_by_id(order_id);
}