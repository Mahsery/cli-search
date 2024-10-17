use std::io;
use std::fs::{OpenOptions, File};
use std::io::Write;
use google_search_rs::search;
use serde_json::{json, Value};

fn main() {
    loop {
        let mut input: String = String::new();

        println!("select one: \n1.Name search 2.Phone search 3.Email search");
  
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("Name search selected!");
                let mut name_input: String = String::new();
                println!("Enter the name to search:");
                io::stdin().read_line(&mut name_input).expect("Failed to read line");
                let name = name_input.trim();

                // Check if the file exists
                let mut file = match OpenOptions::new()
                    .read(true)
                    .open("search_data.json") {
                    Ok(file) => file,
                    Err(_) => {
                        // File doesn't exist, create a new one
                        let mut file = File::create("search_data.json").expect("Failed to create file");
                        file.write_all(b"[]\n").expect("Failed to write to file");
                        file
                    }
                };

                let mut contents = String::new();
                io::Read::read_to_string(&mut file, &mut contents).expect("Failed to read file");

                // If the file is empty, create a new JSON array
                let mut data: Value = if contents.is_empty() {
                    json!([])
                } else {
                    serde_json::from_str(&contents).expect("Failed to parse JSON")
                };

                // Add the new data to the JSON array
                data.as_array_mut().unwrap().push(json!({
                    "name": name
                }));

                // Write the updated JSON object back to the file
                let json_data = serde_json::to_string_pretty(&data).unwrap();
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open("search_data.json")
                    .expect("Failed to open file");
                io::Write::write_all(&mut file, json_data.as_bytes()).expect("Failed to write to file");

                // Search the name using Google Search
                let search_query = name;
                let max_pages = 10;

                let results = search(search_query, max_pages, Some("results.csv")).unwrap();

                // Append to the existing file instead of overwriting it
                let mut file = OpenOptions::new()
                    .append(true)
                    .open("results.csv")
                    .expect("Failed to open file");
                io::Write::write_all(&mut file, format!("\n{}", results).as_bytes()).expect("Failed to write to file");

                // Print the results
                for result in results.iter() {
                    println!("{:?}", result);
                }
            }
            _ => {}
        }
    }
}