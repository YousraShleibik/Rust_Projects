fn compute (a: u32, b: u32) -> u32 {
 a + b * 2
}




#[cfg(test)]
mod tests {
use crate::compute;

#[test]
fn  case(){
assert_eq!(compute(1,2),5);

}
#[test]
fn  case_two(){
assert_eq!(compute(0,0),0);


}
}
