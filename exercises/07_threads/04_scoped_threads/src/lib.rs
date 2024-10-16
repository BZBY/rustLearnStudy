// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::scope;

pub fn sum(v: Vec<i32>) -> i32 {
    let  half = &v.len()/2;
    let mut all_left = 0;
    let mut all_right = 0;
    scope(|x| {
        x.spawn(|| {
            for i in 0..half {
               all_left += v[i];
            }
        });
        x.spawn( || {
            for i in half..v.len() {
                all_right += v[i];
            }
        });
    });
    all_left + all_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
