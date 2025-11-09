use pocket_blockchain::pocket_chain::blockchain::{Blockchain};
fn main() {
    println!("Hello, world!");
    let bc = Blockchain::<String>::new(5);
    print!("{:?}", bc.chain[0])
}
