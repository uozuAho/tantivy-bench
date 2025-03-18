use std::{env, fs};
use std::io::Read;
use std::time::{Duration, Instant};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy};

fn main() -> tantivy::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        panic!("I dunno how to return an error");
    }

    let directory = &args[1];
    let index_mem_size = 400_000_000;
    let num_files = 10000;
    let num_runs = 5;

    let mut index_times: Vec<Duration> = vec![];
    let mut search_times: Vec<Duration> = vec![];

    for _ in 0..num_runs {
        let (index, search, _num) = build_and_search(directory, index_mem_size, num_files)?;
        index_times.push(index);
        search_times.push(search);
    }

    println!("After {} runs. Time to index & search {} files:", num_runs, num_files);
    println!("index: {:?}ms, search: {:?}ms", avg(&index_times), avg(&search_times));

    Ok(())
}

fn avg(times: &Vec<Duration>) -> f64 {
    times.iter().map(|d| d.as_millis() as f64).sum::<f64>() / times.len() as f64
}

fn build_and_search(directory: &String, index_mem_size: usize, num_files: usize) -> tantivy::Result<(Duration, Duration, usize)> {
    let mut schema_builder = Schema::builder();
    let body = schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema);
    let mut index_writer: IndexWriter = index.writer(index_mem_size)?;

    let start_time = Instant::now();

    for entry in fs::read_dir(directory)?.take(num_files) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let mut file = fs::File::open(&path)?;
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;

                    let mut doc = TantivyDocument::default();
                    doc.add_text(body, content);
                    index_writer.add_document(doc)?;
                }
            }
        }
    }

    index_writer.commit()?;

    let index_duration = start_time.elapsed();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![body]);
    let query = query_parser.parse_query("sea whale")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    let num_found = top_docs.len();

    let search_duration = start_time.elapsed() - index_duration;

    Ok((index_duration, search_duration, num_found))
}
