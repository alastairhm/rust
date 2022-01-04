fn cap_values_borrow(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
}

fn main() {
    let mut values = vec![1,2,3,10000,5];
    cap_values_borrow(10, &mut values);
    for v in values {
        println!("{}", v);
    }
}