// struct GroceryItem {
//     name: String,
//     stock: i32,
//     price: f32
// }

// impl GroceryItem {
//     pub fn print_details(&self) {
//         println!("PRODUCT: {:?}\nSTOCK: {:?}\nPRICE: {:?}", self.name, self.stock, self.price)
//     }

//     pub fn set_stock(&mut self, a: i32) {
//         self.stock = a;
//     }

//     pub fn set_prices(&mut self, new_price: f32) {
//         self.price = new_price;
//     }
// }
enum Flavor {
    Strawberry,
    Wintermelon,
    CookiesAndCream
}
struct Drink {
    flavor: Flavor,
    ounce: f32
}

impl Drink {
   pub fn print_drink_info(&self) {
    self.print_drink_flavor();
    self.print_oz()
   }

   fn print_drink_flavor(&self) {
    match self.flavor {
        Flavor::CookiesAndCream => println!("Flavor: COOKIES AND CREAM"),
        Flavor::Strawberry => println!("Flavor: STRAWBERRY"),
        Flavor::Wintermelon => println!("Flavor: WINTERMELON")
    }
   }

   fn print_oz(&self) {
    println!("OZ: {:?}", self.ounce)
   }
}

fn main() {
    // let mut item = GroceryItem{
    //     name: "Hello!".to_string(),
    //     price: 222.35,
    //     stock: 5
    // };

    // item.print_details();
    // item.set_prices(3232.22);
    // item.set_stock(123);
    // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // item.print_details()

    let drink = Drink{
        flavor: Flavor::CookiesAndCream,
        ounce: 0.5
    };

    drink.print_drink_info()
}