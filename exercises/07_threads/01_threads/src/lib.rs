// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // 如果输入的 Vec 为空，直接返回 0
    if v.is_empty() {
        return 0;
    }

    // 获取向量长度的一半，作为切割点
    let half_len = v.len() / 2;

    // 为每个线程分配新的 Vec
    let left_half = v[..half_len].to_vec(); // 左半部分
    let right_half = v[half_len..].to_vec(); // 右半部分

    // 创建第一个线程，计算左半部分的和
    let handle1 = thread::spawn(move || {
        left_half.iter().sum::<i32>()
    });

    // 创建第二个线程，计算右半部分的和
    let handle2 = thread::spawn(move || {
        right_half.iter().sum::<i32>()
    });

    // 等待两个线程完成，获取结果并求和
    let left_sum = handle1.join().unwrap();
    let right_sum = handle2.join().unwrap();

    left_sum + right_sum
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
