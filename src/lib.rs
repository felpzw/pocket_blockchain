pub mod pocket_chain {

    pub mod blockchain{
        use serde::Serialize;
        use sha2::{Digest, Sha256};


        #[derive(Debug, Clone, Serialize)]
        pub struct Block {
            pub index: u64,
            pub timestamp: String,
            pub data: String,
            pub previous_hash: String,
            pub nonce: u64,

            #[serde(skip_serializing)]
            pub hash: String,
        }

        impl Block {
            pub fn calculate_hash(&self) -> String {

                let data_as_json = serde_json::to_string(self).expect("Serialization error");
                
                let mut hasher = Sha256::new();
                hasher.update(data_as_json);
                let result = hasher.finalize();

                format!("{:x}", result)

            }  

        } 


    }
}