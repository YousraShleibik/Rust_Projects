// Problem One
//Function: quadratic
//Description: that takes four unsigned integer parameters: a, b, c, and x. The function should return the value of a + b*x + c*x^2.
//input (u32, u32, u32, u32 )
//output u32

pub fn quadratic(a:u32, b:u32, c:u32, x:u32)->u32{
    a + b*x + c*x*x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic() {
        assert_eq!(quadratic(1, 2, 3, 3), 34); // 1 + 2*3 + 3*9
    }
}