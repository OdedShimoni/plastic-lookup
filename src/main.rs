mod elastic_node;
mod elastic_store;
use elastic_store::ElasticStore;
extern crate serde_json;
use serde_json::{Value};
use std::{time::Instant, collections::HashMap, io::{self, BufRead}};

fn main() {
    let mut elastic_stores: HashMap<String, ElasticStore> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let args = line
            .unwrap();
        let args_vec = args.split(" ").collect::<Vec<&str>>();
        match args_vec[0] {
            "store" => {
                if args_vec.len() < 2 {
                    continue;
                }
                match args_vec[1] {
                    "add" => {
                        if args_vec.len() < 3 {
                            continue;
                        }
                        let store_name = args_vec[2];
                        elastic_stores.entry(store_name.to_owned())
                            .or_insert(
                                ElasticStore::new()
                            );
                        println!(
                            "New store '{}' was added successfuly.",
                            store_name
                        );
                    },
                    "lookup" => {
                        if args_vec.len() < 4 {
                            continue;
                        }
                        let store_name = args_vec[2];
                        let word = args_vec[3];
                        let elastic_store_option = elastic_stores.get(
                            store_name
                        );
                        if let Some(elastic_store) = elastic_store_option {
                        let now = Instant::now();
                        let found_option = elastic_store.nodes.lookup(word);
                        let elapsed = now.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        if let Some(found) = found_option {
                            match &found.included_reference_key {
                                Some(reference_key) => {
                                    println!("2");
                                    println!(
                                        "{}",
                                        elastic_store.data.get(reference_key)
                                            .unwrap_or(&Value::String("".to_string())
                                        )
                                    );
                                },
                                None => {
                                    println!("1");
                                    println!("Found a partial word or a word without reference");
                                },
                            };
                        } else {
                            println!("0");
                            println!("Word wasn't found");
                        }
                        } else {
                            println!("0");
                            println!("Store name {} wasn't found.", store_name);
                            continue;
                        }
                    }
                    "list"|"ls" => {
                        println!("Stores:");
                        println!("{:?}", elastic_stores);
                    }
                    _ => {

                    }
                }
            },
            "word" => {
                if args_vec.len() < 2 {
                    continue;
                }
                match args_vec[1] {
                    "add" => {
                        if args_vec.len() < 5 {
                            continue;
                        }
                        let store_name = args_vec[2];
                        let word = args_vec[3];
                        let data_raw = args_vec[4];
                        let elastic_store_option = elastic_stores.get_mut(store_name);
                        if let Some(elastic_store) = elastic_store_option {
                            let data = serde_json::json!(data_raw);
                            elastic_store.add_word(word.to_owned(), word.to_owned(), data);
                        }
                        
                        println!("1");
                        println!(
                            "New word '{}' was added successfuly.",
                            word
                        );
                    }
                    _ => {

                    }
                }
            },
            _ => {

            }
        }
    }
}
