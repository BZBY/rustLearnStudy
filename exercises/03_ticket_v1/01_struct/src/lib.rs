// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

#[cfg(test)]
mod tests {
    use std::u32;

    use super::*;

    struct Order {
        pub(super) price: u32,
        pub(super) quantity: u32,
    }
    impl Order {
        fn default(a: u32, b: u32) -> Order {
            Order {
                price: a,
                quantity: b,
            }
        }

        fn is_available(self) -> bool {
            if self.quantity > 0 {
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
