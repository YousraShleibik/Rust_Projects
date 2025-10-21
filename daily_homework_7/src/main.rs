use daily_homework_7::Direction;

fn main() {
    let a = Direction::new(2, 3);
    let b = Direction::new(5, 1);

    // Uses Display
    println!("a = {}", a);
    println!("b = {}", b);

    // Method add
    let c = a.add(&b);
    println!("a.add(&b) = {}", c);

    // Operator +
    let sum = a + b;
    println!("a + b = {}", sum);

    // Operator -
    let p = Direction::new(10.0, 4.5);
    let q = Direction::new(3.0, 1.5);
    let diff = p - q;
    println!("p - q = {}", diff);
}
