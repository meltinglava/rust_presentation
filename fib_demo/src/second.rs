mod fib {
    pub struct Fib {
        current: usize,
        former: usize,
        count: usize,
    }

    impl Fib {
        pub fn new(former: usize, current: usize) -> Self {
            Self {
                current,
                former,
                count: 0,
            }
        }
    }

    impl Default for Fib {
        fn default() -> Self {
            Self::new(0, 1)
        }
    }

    impl Iterator for Fib {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            Some(match self.count {
                1 => self.former,
                2 => self.current,
                _ => {
                    std::mem::swap(&mut self.current, &mut self.former);
                    self.current += self.former;
                    self.current
                }
            })
        }
    }
}

fn main() {
    let values = 40;
    let sum: usize = fib::Fib::default().take(values).sum();
    println!("Sum of the frist {} values in fibonacci: {}", values, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_second() {
        let actual: Vec<_> = fib::Fib::default().take(10).collect();
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(actual, expected);
    }
}
