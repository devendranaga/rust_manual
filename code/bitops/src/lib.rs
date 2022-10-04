pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn rotate64(val: u64, pos: u32) -> u64 {
    (val << pos) | (val << (64 - pos))
}

pub fn rotate32(val: u32, pos: u32) -> u32 {
    (val << pos) | (val << (32 - pos))
}

pub fn htons(val: u16) -> u16 {
    (val << 8) | (val >> 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = htons(0x1000);
        println!("{}", result);
    }
}
