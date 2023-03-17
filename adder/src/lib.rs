pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn sub(left: usize, right: usize) -> usize {
    left - right
}
pub fn mul(left: usize, right: usize) -> usize {
    left * right
}
pub fn div(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
}

#[cfg(test)]
mod tests_sub {
    use super::*;
#[test]
    fn it_works() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}

#[cfg(test)]
mod tests_multiplication {
    use super::*;
#[test]
    fn it_works() {
        let result = mul(2, 2);
        assert_eq!(result, 3);
    }
}

#[cfg(test)]
mod tests_division {
    use super::*;
#[test]
    fn it_works() {
        let result = div(2, 2);
        assert_eq!(result, 1);
    }
}