# submit-runtime-upgrade
cli to submit a runtime upgrade easily
    

Tool submits a substrate wasm file using sudo to the local instance of a development node   



## Build and Run:  
Spin up a dev node that connects to the local network interface and binds: 127.0.0.1:9944, ./target/release/substrate-node-template --dev --base-path=/tmp/tmp_chain --ws-external.    
**Once the node is running on 127.0.0.1:9944 then you can compile**   
`$ cargo build --release`

