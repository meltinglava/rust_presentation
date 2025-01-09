use num::BigUint;

mod fib {
    use num::{One, Zero};
    use std::ops::AddAssign;

    pub struct Fib<T> {
        current: T,
        former: T,
        count: usize,
    }

    impl<T> Fib<T> {
        pub fn new(former: T, current: T) -> Self {
            Self {
                current,
                former,
                count: 0,
            }
        }
    }

    impl<T: Zero + One> Default for Fib<T> {
        fn default() -> Self {
            Self::new(T::zero(), T::one())
        }
    }

    impl<T> Iterator for Fib<T>
    where
        T: for<'a> AddAssign<&'a T> + Clone,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            Some(match self.count {
                1 => self.former.clone(),
                2 => self.current.clone(),
                _ => {
                    std::mem::swap(&mut self.current, &mut self.former);
                    self.current += &self.former;
                    self.current.clone()
                }
            })
        }
    }
}

fn main() {
    let values = 1000;
    let fibs = fib::Fib::<BigUint>::default();
    let sum: BigUint = fibs.take(values).sum();
    println!("Sum of the frist {} values in fibonacci: {}", values, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_fourth() {
        let actual: Vec<usize> = fib::Fib::new(0, 1).take(10).collect();
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(actual, expected);
    }
}
