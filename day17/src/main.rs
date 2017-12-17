extern crate linked_list;
use linked_list::LinkedList;

fn main() {
    let increment = 335;
    let rounds = 50000000;
    let mut xs = LinkedList::new();
    {
        let mut cursor = xs.cursor();
        cursor.insert(0);
        for i in 0..rounds {
            for _ in 0..increment {
                if cursor.next().is_none() {
                    cursor.next();
                }
            }
            cursor.insert(i + 1);
            cursor.next();
        }
    }
    let zero_index = xs.iter().position(|&x| x == 0).unwrap();
    println!("Directly after 0: {}", xs.iter().skip(zero_index + 1).next().unwrap());
}