pub fn add_one(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}


pub fn get_total(v: &Vec<u32>) -> u32 {
    v.iter().copied().sum()
}


pub fn sum_tuple(pairs: Vec<(u32, u32)>) -> Vec<u32> {
    pairs.into_iter().map(|(a, b)| a + b).collect()
}
