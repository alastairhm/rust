use std::collections::LinkedList;

fn main() {
    let mut ll = LinkedList::new();
    
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(4);
    
    for foo in ll {
        println!("{}", foo);
    }
}

