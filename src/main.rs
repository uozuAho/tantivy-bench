use std::{env, fs};
use std::io::Read;
use std::time::Instant;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

fn main() -> tantivy::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <directory>", args[0]);
    //     panic!("I dunno how to return an error");
    // }
    // let directory = &args[1];

    let directory = "/home/woz/Downloads/10000_md_files";

    println!("Building index...");

    let mut schema_builder = Schema::builder();
    let body = schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema);
    let mut index_writer: IndexWriter = index.writer(400_000_000)?;

    let start_time = Instant::now();

    println!("{:.2?}: Loading files...", start_time.elapsed());

    for entry in fs::read_dir(directory)?.take(10000) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let mut file = fs::File::open(&path)?;
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;

                    let mut old_man_doc = TantivyDocument::default();
                    old_man_doc.add_text(body, content);
                    index_writer.add_document(old_man_doc)?;
                }
            }
        }
    }

    println!("{:.2?}: Commiting index...", start_time.elapsed());

    index_writer.commit()?;

    println!("{:.2?}: Preparing search...", start_time.elapsed());

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![body]);
    let query = query_parser.parse_query("sea whale")?;

    println!("{:.2?}: Running search...", start_time.elapsed());

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    let num_found = top_docs.len();

    println!("{:.2?}: Done. Found {} docs.", start_time.elapsed(), num_found);

    Ok(())
}
