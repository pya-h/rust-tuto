## Watch server

Watch for 'cargo run':

```bash
cargo watch -q -c -w src/ -x run
```


## Watch for Tests
```bash
cargo watch -q -c -w tests/ -x "test -q [test_func_name] -- --nocapture"
```