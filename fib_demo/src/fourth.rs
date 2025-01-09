mod fib {
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
    /*
    impl Default for Fib {
        fn default() -> Self {
            Self::new(0, 1)
        }
    }
     */
    impl<T> Iterator for Fib<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            Some(match self.count {
                1 => self.former,
                2 => self.current,
                _ => {
                    std::mem::swap(&mut self.current, &mut self.former);
                    self.current += &self.former;
                    self.current
                }
            })
        }
    }
}

fn main() {
    let values = 100;
    let sum: usize = fib::Fib::new(0, 1).take(values).sum();
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
