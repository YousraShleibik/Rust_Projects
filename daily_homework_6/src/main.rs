fn main() {
    let mut demo = vec![1, 2, 3];
    daily_homework_6::add_one(&mut demo);
    println!("After add_one: {:?}", demo);

    let total = daily_homework_6::get_total(&demo);
    println!("Total: {}", total);

    let summed = daily_homework_6::sum_tuple(vec![(1, 9), (10, 5), (7, 7)]);
    println!("sum_tuple: {:?}", summed);
}