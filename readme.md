# My tantivy benchmarks

# Quick start
- Install rust
- `sudo apt install heaptrack`
- (optional) download [10000 markdown files](https://github.com/Zettelkasten-Method/10000-markdown-files)

```sh
cargo run --profile release -- <directory of md files>

# get mem usage
heaptrack ./target/release/tantivy-bench
heaptrack --analyze "/home/woz/woz/tantivy-bench/heaptrack.tantivy-bench.45181.zst"
```

# Current stats
```
After 5 runs. Time to index & search 10000 files:
index: 160.6ms, search: 0.0ms
peak heap memory consumption: 234.60M
```