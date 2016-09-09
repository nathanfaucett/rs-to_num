#![no_std]


pub trait ToNum {
    fn to_num_radix(&self, radix: usize) -> usize;
    fn to_num_radix_signed(&self, radix: usize) -> isize;
    fn to_num(&self) -> usize;
    fn to_num_signed(&self) -> isize;
}

impl ToNum for str {

    fn to_num_radix(&self, radix: usize) -> usize {
        if radix == 0 {
            0
        } else {
            let mut num = 0;

            for c in self.chars() {
                let digit;

                if c >= '0' && c <= '9' {
                    digit = c as usize - '0' as usize
                } else if c >= 'A' && c <= 'Z' {
                    digit = c as usize - 'A' as usize + 10
                } else if c >= 'a' && c <= 'z' {
                    digit = c as usize - 'a' as usize + 10
                } else {
                    break;
                }

                if digit >= radix {
                    break;
                }

                num *= radix;
                num += digit;
            }

            num
        }
    }

    fn to_num_radix_signed(&self, radix: usize) -> isize {
        if self.starts_with('-') {
            -(self[1..].to_num_radix(radix) as isize)
        } else {
            self.to_num_radix(radix) as isize
        }
    }

    fn to_num(&self) -> usize {
        self.to_num_radix(10)
    }

    fn to_num_signed(&self) -> isize {
        self.to_num_radix_signed(10)
    }
}

#[test]
fn test_to_num() {
    assert_eq!("10".to_num(), 10);
    assert_eq!("-10".to_num_signed(), -10);
}
