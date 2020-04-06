mod data_structures;
use data_structures::trees::Tree;
fn main() {
    let mut tree:Tree<u32> = Tree::new(1);
    tree.add_child(13);
    tree.add_child(15);
    tree.next(0);
    tree.add_child(156);
    tree.reset();
    tree.child_count();
    println!("{}", tree.get().val);
}
