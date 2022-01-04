// fn take_ownership_sum(v: Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for value in v {
//         sum += value;
//     }
//     return sum;
// }

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    return sum;
}

fn main() {
    let values = vec![1,2,3,4,5];
    let sum = borrow_sum(&values);
    println!("Sum of {} values: {}", values.len(), sum)
}