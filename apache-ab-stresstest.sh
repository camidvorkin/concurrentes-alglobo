#! bash

# sudo apt install apache2-utils
# run `cargo run --bin threads` on a different terminal

ab -p ./apache-ab-req.json -H "Content-Type: application/json" -n 100 -c 2 http://localhost:8080/
