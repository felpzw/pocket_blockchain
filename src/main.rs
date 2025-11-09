
use pocket_blockchain::pocket_chain::blockchain::{Block, Blockchain};
fn main() {
    let bc = Blockchain::<String>::new();
    print!("{:#?}", bc.chain[0]);
    let mut nonce: u64 = 0;
    let mut test_block = Block::new_actual_block(String::from("Teste"), 1, &nonce);
    let mut found = String::default();

    while true {
        let a_hash = test_block.calculate_hash();
        match validate_hash(&test_block, 2) {
            true => {
                found = a_hash;
                break;
            },
            false => {
                nonce += 1;
                println!("Updating Nonce: {nonce}");
                test_block.update_nonce(&nonce);
            },
        }
    }
    println!("Found Hash: {found}");
}

fn validate_hash(block: &Block<String>, dif: usize) -> bool {
    let u_hash = block.calculate_hash();
    let chars = u_hash.chars().count();
    let char_vec: Vec<char> = u_hash.chars().collect();
    for n in (chars - dif)..(chars) {
        if char_vec[n] != '0' {
            return false
        }
    }
    true
}
