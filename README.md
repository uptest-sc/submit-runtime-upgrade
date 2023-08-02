# submit-runtime-upgrade
cli to submit a runtime upgrade easily
    

Tool submits a substrate wasm file using sudo to the local instance of a development node   



## Build and Run:  
Spin up a dev node that connects to the local network interface and binds: 127.0.0.1:9944, ./target/release/substrate-node-template --dev --base-path=/tmp/tmp_chain --ws-external.    
**Once the node is running on 127.0.0.1:9944 then you can compile**   
`$ cargo build --release`

```shell
./target/release/submit-runtime-upgrade 
Current Runtime Version: 108
sending tx
Sent tx: 0x00e21a73b8e016b27143f1b9051fcb66db36b601554d862a591870c3015514a7
Runtime is now
Runtime Version changed from 109 to 108
```
