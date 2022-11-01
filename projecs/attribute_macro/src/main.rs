use attribute_macro::*;

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let laptop = Product {
        name: "MacBook Pro".to_owned(),
        price: 2000,
    };

    buy_product(laptop, 20);
}

#[log_call(verbose)]
fn buy_product(product: Product, discount: u32) {}
