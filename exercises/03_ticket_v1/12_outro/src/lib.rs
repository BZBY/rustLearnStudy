// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub mod outro_023{
    pub struct  Order{
        product_name : String,
        quantity : i32,
        unit_price : u32,
    }

    impl Order {
        pub fn total(&self) -> i64 {
            (self.quantity as u32 * self.unit_price) as i64
        }
    }

    impl Order{
        fn vaild_product_name(&self){
            if self.product_name.is_empty() {
                panic!("1")
            }
            if self.product_name.len() > 300 {
                panic!("2")
            }
        }
        fn vailed_quantity(&self){
            if self.quantity <= 0 {
                panic!("2")
            }
        }
        fn vailed_unit_price(&self){
            if self.unit_price <= 0 {
                panic!("2")
            }
        }
        pub fn new(product_name2 : String, quantity2 : i32, unit_price2 : u32)->Order{

            if product_name2.is_empty() {
                panic!("1")
            }
            if product_name2.len() > 300 {
                panic!("2")
            }
            if quantity2 <= 0 {
                panic!("2")
            }
            if unit_price2 <= 0 {
                panic!("2")
            }
            
            Order{
                product_name:product_name2,
                quantity:quantity2,
                unit_price:unit_price2,
            
            }
            
        }
      
        
        pub fn set_product_name(&mut self,product_name : String){
            self.product_name = product_name;
            self.vaild_product_name()
        } 
        
        pub fn set_unit_price(&mut self,price : u32){
            self.unit_price = price;
            self.vailed_unit_price()
        }
        
        pub fn set_quantity(&mut self,quantity : i32){
            self.quantity = quantity;
            self.vailed_quantity()
        }
        
        pub fn product_name(&self)->&String{
            &self.product_name
        }
        pub fn unit_price(&self)->&u32{
            &self.unit_price
        }
        pub fn quantity(&self)->&i32{
            &self.quantity
        }
    }
}