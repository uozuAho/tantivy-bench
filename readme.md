# My tantivy benchmarks

[Tantivy](https://github.com/quickwit-oss/tantivy).
Intended to compete with https://github.com/uozuAho/ts-fulltext-compare

# Quick start
- Install rust
- `sudo apt install heaptrack`
- (optional) download [10000 markdown files](https://github.com/Zettelkasten-Method/10000-markdown-files)

```sh
cargo run --profile release -- <directory of md files>

# get mem usage
heaptrack ./target/release/tantivy-bench
heaptrack --analyze <use output of above command>
```

# Current stats
```
Using tokenizer TEXT
After 5 runs. Time to index & search 10000 files:
index: 162.4ms, search: 0.0ms

Using tokenizer wozregex
After 5 runs. Time to index & search 10000 files:
index: 690.2ms, search: 0ms

peak heap memory consumption: 310.66M (testing both indexes)
```
