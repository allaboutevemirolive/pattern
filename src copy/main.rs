use read::read_file;

pub mod bm;
pub mod read;

fn main() {
    let file_path = "/home/nemesis/Documents/Github/Focus/algo/pattern/data/brown.txt";

    let text = match read_file(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let pattern = "preparations";

    // let text = "This is a sample text for searching.";
    // let pattern = "sample";

    match bm::boyer_moore_search(&text, pattern) {
        Some(index) => println!("Pattern found at index: {}", index),
        None => println!("Pattern not found in the text."),
    }
}
