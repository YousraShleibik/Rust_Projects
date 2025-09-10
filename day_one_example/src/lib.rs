// Problem One
//Function: quadratic
//Description: that takes four unsigned integer parameters: a, b, c, and x. The function should return the value of a + b*x + c*x^2.
//input (u32, u32, u32, u32 )
//output u32

pub fn quadratic(a:u32, b:u32, c:u32, x:u32)->u32{
    a + b*x + c*x*x
}

//Problem Two
//Function: scale_vector
//Description: single floating point number along with a 2-tuple which represents a two dimensional vector. 
// The function should return a 2-tuple which represents the vector scaled by the value
//input (f32, (f32, f32))
//output (f32, f32)
pub fn scale_vector(point: f32, vector:(f32,f32))-> (f32,f32){
    (point*vector.0, point*vector.1)
}


//Problem Three
//function: dot_product
//Description: takes two 2-tuples which represent two dimensional vectors and calculates the dot product of the vectors. which
// is calculated as (x1 * x2) + (y1 * y2) where (x1, y1) and (x2, y2) are the two input vectors.
//input ((f32, f32), (f32, f32))
//output f32

pub fn dot_product(vector1:(f32,f32), vector2:(f32,f32))-> f32{
    vector1.0 * vector2.0 + vector1.1 * vector2.1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic() {
        assert_eq!(quadratic(1, 2, 3, 3), 34); // 1 + 2*3 + 3*9
    }
    #[test]
    fn test_scale_vector() {
        assert_eq!(scale_vector(5.0, (3.0, 4.0)), (15.0, 20.0));
    }
    #[test]
    fn test_dot_product(){
        assert_eq!(dot_product((2.0, 5.0), (7.0, 1.0)), 19.0); // 2*7 + 5*1
    }
}