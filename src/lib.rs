pub mod pocket_chain {

    pub mod blockchain{
        use std::fmt::Debug;

        use serde::Serialize;
        use serde_json::json;
        use sha2::{Digest, Sha256};
        use chrono::Utc;


        #[derive(Debug, Clone, Serialize)]
        pub struct Block<T> {
            pub index: u64,
            pub timestamp: String,
            pub data: T,
            pub previous_hash: String,
            pub nonce: u64,

            #[serde(skip_serializing)]
            pub hash: String,
        }

        impl<T: Serialize> Block<T> {
            pub fn calculate_hash(&self) -> String {

                let data_as_json = serde_json::to_string(self).expect("Serialization error");
                
                let mut hasher = Sha256::new();
                hasher.update(data_as_json);
                let result = hasher.finalize();

                format!("{:x}", result)

            }  

        }
        
        
        pub struct Blockchain<T> {
            pub chain: Vec<Block<T>>
        }

        impl<T: Serialize + Default + Debug> Blockchain<T> {
            pub fn new(difficult: u32) -> Self
            {   
                let mut genesis = Block {
                    index: 0,
                    timestamp: Utc::now().to_string(),
                    data: T::default(),
                    previous_hash: String::from("0"),
                    nonce: 0,
                    hash: String::new(),
                 };
                genesis.hash = genesis.calculate_hash();
                println!("{:?}", genesis);
                Blockchain { chain: vec![genesis] }
            }
        }


    }
}