# My tantivy benchmarks

# Quick start
- Install rust
- `sudo apt install heaptrack`

```sh
cargo run --profile release

# get mem usage
heaptrack ./target/release/tantivy-bench
heaptrack --analyze "/home/woz/woz/tantivy-bench/heaptrack.tantivy-bench.45181.zst"
```