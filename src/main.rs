use crate::trie::Trie;
use clap::Parser;
use std::fs::File;
use std::{fs, io::BufRead, io::BufReader, io::stdin, time::Instant};

mod node;
mod trie;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to line-by-line words file to absorb by trie, optionally with priority
    #[arg(short, long, default_value = "directories.txt")]
    pub words_source_path: String,

    /// Path to line-by-line words file to search in trie
    #[arg(short, long)]
    pub search_words_path: Option<String>,

    /// Search by passing words in terminal
    #[arg(short, long, default_value_t = false)]
    pub interactive_search: bool,

    /// Default count of found results
    #[arg(short, long, default_value_t = 100)]
    pub max_results_count: i32,

    /// Output results to file, instead of terminal
    #[arg(short, long)]
    pub results_file: Option<String>,

    /// Do not output to terminal anything
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
}

fn main() {
    let mut args = Cli::parse();

    if args.words_source_path.is_empty() {
        println!("Words source path is empty");
        return;
    }

    if let Err(_) = fs::exists(&args.words_source_path) {
        println!("Words source path does not exist");
        return;
    }

    let mut trie = Trie::new();
    let start_of_load = Instant::now();
    let mut count_of_words = 0;
    let file = File::open(&args.words_source_path);
    let word_source_reader = BufReader::new(file.unwrap());
    word_source_reader.lines().for_each(|line| {
        let mut word_passed = false;
        let mut priority_passed = false;
        let mut passing_word = String::default();
        let mut passing_priority = 0;
        line.unwrap().split(" ").for_each(|word| {
            let trimmed = word.trim();

            if !word_passed {
                passing_word = trimmed.to_string();
                word_passed = true;
                count_of_words += 1;
                return;
            }

            if !priority_passed {
                passing_priority = trimmed.parse::<i64>().unwrap_or(0);
                priority_passed = true;
            }
        });

        trie.add(&passing_word, passing_priority);
    });
    let elapsed_of_load = start_of_load.elapsed();
    if !args.quiet {
        println!("Elapsed time of building trie: {:?}", elapsed_of_load);
        println!("Count of words: {}", count_of_words);
    }

    if args.interactive_search {
        println!("Interactive search is on");

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let start = Instant::now();
            let result = trie.search(&input.trim(), args.max_results_count);
            let elapsed = start.elapsed();
            if !args.quiet {
                println!("Elapsed time: {:?}", elapsed);
            }

            result.iter().for_each(|r| {
                let borrowed = r.borrow();
                println!(
                    "Value: {}, Priority: {}",
                    borrowed.transitional_node.full_value_as_string(), borrowed.transitional_node.priority
                )
            });
        }
    }

    if args.search_words_path.is_none() {
        println!("Words source path is empty");
    }

    if let Err(_) = fs::exists(&args.search_words_path.unwrap()) {
        println!("Search words path does not exist");
        return;
    }

    let results_to_terminal = args.results_file.is_none();
    let mut results_vector = vec![];
    let file = File::open(&args.words_source_path);
    if file.is_err() {
        println!("Failed to open words source file");
    }

    let reader = BufReader::new(file.unwrap());
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if !args.quiet {
            println!("Search term: {}", line);
        }

        let trimmed = line.trim();
        let start_of_search = Instant::now();
        let result = trie.search(&trimmed, args.max_results_count);
        let elapsed = start_of_search.elapsed();
        if !args.quiet {
            println!("Elapsed time of search: {:?}, term {}", elapsed, &trimmed);
        }

        if results_to_terminal {
            result.iter().for_each(|r| {
                let borrowed = r.borrow();
                println!(
                    "Value: {}, Priority: {}",
                    borrowed.transitional_node.full_value_as_string(), borrowed.transitional_node.priority
                )
            });
        } else {
            result.iter().for_each(|r| {
                let borrowed = r.borrow();
                results_vector.push(borrowed.transitional_node.full_value_as_string());
            })
        }
    });

    if !results_to_terminal && args.results_file.is_some() {
        if let Ok(_) = fs::write(
            &args.results_file.unwrap(),
            results_vector.join("\n").as_bytes(),
        ) {
            println!("result of search is written to file");
        }
    }
}
