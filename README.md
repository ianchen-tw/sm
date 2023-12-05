# sm - ssh manager

## Installation
+ Install `cargo install --git https://github.com/ianchen-tw/sm`
+ Run `sm --help`

## Testing
+ Unit Test: `cargo test`
+ Integration:
    1. Start an sample open-ssh server: `cd test-server && docker compose up`
    2. Use `sm --home ./test-server` to connect