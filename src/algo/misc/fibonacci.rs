use std::mem;

pub fn r_fibonacci(num: isize) -> isize {
    match num {
        1 ... 2 => 1,
        _ => r_fibonacci(num - 1) + r_fibonacci(num - 2)
    }
}

pub fn fibonacci(num: isize) -> isize {
    let (mut prev, mut curr) = (1, 1);
    for _ in 2 .. num {
        mem::swap(&mut prev, &mut curr);
        curr = curr + prev;
    }
    curr
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        for fibo in [fibonacci, r_fibonacci].iter() {
            let result = fibo(6);
            assert_eq!(result, 8);
        }
    }
}
