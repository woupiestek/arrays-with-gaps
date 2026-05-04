use arrays_with_gaps::RedBlackTree;

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(10, "ten");
    tree.insert(5, "five");
    tree.insert(15, "fifteen");

    println!("baseline tree len = {}", tree.len());
    println!("contains 10 = {}", tree.contains_key(&10));
    println!("15 -> {}", tree.get(&15).unwrap_or(&"<missing>"));

    tree.remove(&5);
    println!("after remove len = {}", tree.len());
}
