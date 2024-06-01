use num::{Integer, ToPrimitive};
use num_traits::Signed;
use std::ops::{Add, Mul, Sub};
use std::{fmt, mem};
use Sign::*;

#[derive(Debug, PartialEq)]
enum Sign {
    Positive,
    Negative,
}
#[derive(Debug)]
pub struct BigInteger {
    digits: Vec<u8>,
    sign: Sign,
}

impl BigInteger {
    fn new(digits: Vec<u8>, sign: Sign) -> BigInteger {
        BigInteger {
            digits: digits,
            sign: sign,
        }
    }

    pub fn new_from_int<
        T: Integer + Signed + ToPrimitive + fmt::Display + std::ops::DivAssign<T>,
    >(
        number: T,
    ) -> BigInteger {
        let mut results: Vec<u8> = Vec::new();
        let sign: Sign = if number.is_positive() {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let mut number = number.to_i128().unwrap().abs();

        while number % 10 > 0 {
            results.push((number % 10) as u8);
            number /= 10;

            // println!(
            //     "{}, {}, {}, {:?}",
            //     number,
            //     number % 10,
            //     number / 10,
            //     results
            // );
        }
        // println!("{:?}", results);
        BigInteger {
            digits: results,
            sign: sign,
        }
    }

    pub fn zero() -> BigInteger {
        BigInteger {
            digits: vec![0],
            sign: Sign::Positive,
        }
    }

    pub fn one() -> BigInteger {
        BigInteger {
            digits: vec![1],
            sign: Sign::Positive,
        }
    }

    fn optimize_digits(mut self) -> Self {
        println!("{:?}", self.digits);
        while let Some(&0) = self.digits.last() {
            self.digits.pop();
        }
        println!("{:?}", self.digits);
        self
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sign::Positive => write!(f, ""),
            Sign::Negative => write!(f, "-"),
        }
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //convert digits to string
        let output: String = self
            .digits
            .iter()
            .rev()
            .map(|x| (x + b'0') as char)
            .collect();

        write!(f, "{}{}", self.sign, output)
    }
}

impl PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        let same_sign = self.sign == other.sign;
        let vec1 = self.digits.clone();
        let vec2 = other.digits.clone();
        let same_digits = remove_trailing_zeros(vec1) == remove_trailing_zeros(vec2);
        same_digits && same_sign
    }
}

fn remove_trailing_zeros(mut vec: Vec<u8>) -> Vec<u8> {
    while let Some(&0) = vec.last() {
        vec.pop();
    }
    vec
}

impl Add for BigInteger {
    type Output = BigInteger;

    fn add(self, other: BigInteger) -> BigInteger {
        let signs = (&self.sign, &other.sign);
        match signs {
            (Sign::Positive, Sign::Negative) => return self - other,
            (Sign::Negative, Sign::Positive) => return other - self,
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                let max_length = self.digits.len().max(other.digits.len());
                let mut vec1 = self.digits.clone();
                let mut vec2 = other.digits.clone();
                vec1.resize(max_length, 0 as u8);
                vec2.resize(max_length, 0 as u8);

                let mut carry = 0;
                let mut result: Vec<u8> = vec1
                    .iter()
                    .zip(vec2)
                    .map(|(x, y)| -> u8 {
                        let res = x + y + carry;
                        if res >= 10 {
                            carry = 1;
                            res - 10
                        } else {
                            carry = 0;
                            res
                        }
                    })
                    .collect();
                if carry == 1 {
                    result.push(1)
                };
                return BigInteger {
                    digits: result,
                    sign: Sign::Positive,
                };
            }
        }
    }
}

impl Sub for BigInteger {
    type Output = BigInteger;

    fn sub(self, other: BigInteger) -> BigInteger {
        let mut resulting_sign = Sign::Positive;
        let max_length = self.digits.len().max(other.digits.len());
        let mut vec1 = self.digits.clone();
        let mut vec2 = other.digits.clone();
        vec1.resize(max_length, 0 as u8);
        vec2.resize(max_length, 0 as u8);

        vec1.reverse();
        vec2.reverse();
        if vec1 < vec2 {
            mem::swap(&mut vec1, &mut vec2);
            resulting_sign = Sign::Negative;
        }
        vec1.reverse();
        vec2.reverse();

        let mut carry: i8 = 0;
        let mut result: Vec<u8> = vec1
            .into_iter()
            .zip(vec2)
            .map(|(x, y)| -> u8 {
                let res: i8 = (x as i8) - (y as i8) - carry;
                if res < 0 {
                    carry = 1;
                    (10 + res) as u8
                } else {
                    carry = 0;
                    res as u8
                }
            })
            .collect();
        if carry == 1 {
            result.push(1)
        };
        return BigInteger {
            digits: result,
            sign: resulting_sign,
        };
    }
}

impl Mul for BigInteger {
    type Output = BigInteger;

    fn mul(self, other: BigInteger) -> BigInteger {
        let max_length = self.digits.len().max(other.digits.len());
        let mut vec1 = self.digits.clone();
        let mut vec2 = other.digits.clone();
        vec1.resize(max_length, 0 as u8);
        vec2.resize(max_length, 0 as u8);
        println!("{:?} * {:?}", vec1, vec2);
        let mut carry = 0;
        let mut summands: Vec<BigInteger> = vec2
            .into_iter()
            .enumerate()
            .map(|(index, elem2)| {
                // println!("step {index}, elem2: {elem2}, vec: {:?}", vec1);
                let mut summand: Vec<u8> = vec1
                    .iter()
                    .map(|elem1| {
                        let res = elem1 * elem2 + carry;
                        if res >= 10 {
                            carry = res / 10;
                            res % 10
                        } else {
                            carry = 0;
                            res
                        }
                    })
                    .collect::<Vec<u8>>();
                for _ in 0..index {
                    summand.insert(0, 0);
                }
                // println!("result of step {index}: {:?}", summand);
                BigInteger {
                    digits: summand,
                    sign: Positive,
                }
            })
            .collect();

        let dev = summands
            .into_iter()
            .reduce(|acc, e| {
                println!("acc:{:?}", acc);
                acc + e
            })
            .unwrap();
        println!("res::::::{:?}", dev);
        dev

        // if carry != 0 {
        //     result.push(carry)
        // }
        // println!("{:?}", result);
        // BigInteger {
        //     digits: vec![1, 2, 3],
        //     sign: match (self.sign, other.sign) {
        //         (Positive, Positive) | (Negative, Negative) => Positive,
        //         (_, _) => Negative,
        //     },
        // }
    }
}

// .map(|(x, y)| -> u8 {
//     let res = x * y + carry;
//     if res >= 10 {
//         carry = res / 10;
//         res % 10
//     } else {
//         carry = 0;
//         res
//     }
// })
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize() {
        let num1 = BigInteger::new_from_int(000012342);

        assert_eq!(num1.optimize_digits(), BigInteger::new_from_int(12342));
    }

    #[test]
    fn test_add() {
        let num1 = BigInteger::new_from_int(12342);
        let num2 = BigInteger::new_from_int(434321);

        assert_eq!(num1 + num2, BigInteger::new_from_int(446663));
    }

    #[test]
    fn test_sub() {
        let num1 = BigInteger::new_from_int(12342);
        let num2 = BigInteger::new_from_int(434321);

        assert_eq!(num1 + num2, BigInteger::new_from_int(446663));
    }

    #[test]
    fn test_mul() {
        let num1 = BigInteger::new_from_int(123);
        let num2 = BigInteger::new_from_int(43);

        assert_eq!(num1 * num2, BigInteger::new_from_int(5289));
    }
}
