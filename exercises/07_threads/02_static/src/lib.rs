// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let handle = thread::spawn(move || {
        let mut sum1 = 0;
        for i in 0..slice.len()/2 {
            sum1 += slice[i];
        }
        sum1
    });
    let handle2 = thread::spawn(move || {
        let mut sum2 = 0;
        for i in slice.len()/2..slice.len() {
            sum2 += slice[i];
        }
        sum2
    });
    let a = handle.join().unwrap();
    let b = handle2.join().unwrap();
    a+b
    // let handle1 = thread::spawn(move || slice[..slice.len()/2].iter().sum::<i32>());
    // let handle2 = thread::spawn(move || slice[slice.len()/2..].iter().sum::<i32>());
    // handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
