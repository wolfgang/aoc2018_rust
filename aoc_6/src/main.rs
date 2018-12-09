#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_assert() {
        assert_eq!(1 , 1);
    }
}