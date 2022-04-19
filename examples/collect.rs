fn main () {
    let mut iterator = (1..10).into_iter();
//    iterator.skip(2);
    let taken = iterator.take(4);
    let v: Vec<i32> = taken.collect();
    
    for item in v {
        println!("{}", item);
    }
}
