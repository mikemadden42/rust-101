use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mapping_path = "src/mapping.txt";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./url_shortener <url>");
        return;
    }

    let url = &args[1];

    if url.starts_with("http") {
        // Reuse an existing short URL if this long URL was already shortened.
        if let Some(short_url) = lookup_long(mapping_path, url) {
            println!("Long URL: {url}");
            println!("Short URL: {short_url} (already shortened)");
            return;
        }

        // Convert long-form URL to short-form URL.
        let mut rng = rand::thread_rng();
        let short_url: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char)
            .take(8)
            .collect();

        println!("Long URL: {url}");
        println!("Short URL: {short_url}");

        // Store the mapping between the short URL and the long URL in a file.
        // `create(true)` so the first run works before the file exists.
        let Ok(mut mapping_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(mapping_path)
        else {
            println!("Error opening mapping file");
            return;
        };
        let mapping = format!("{short_url},{url}\n");
        if mapping_file.write_all(mapping.as_bytes()).is_err() {
            println!("Error writing to mapping file");
        }
    } else {
        // Read the mapping file and redirect the user to the corresponding long URL.
        match lookup_short(mapping_path, url) {
            Ok(Some(long)) => println!("Redirecting to {long}"),
            Ok(None) => println!("Short URL not found"),
            Err(err) => println!("Error reading mapping file: {err}"),
        }
    }
}

// Return the long URL mapped to a given short URL, if any.
fn lookup_short(mapping_path: &str, short_url: &str) -> std::io::Result<Option<String>> {
    let reader = BufReader::new(File::open(mapping_path)?);
    for line in reader.lines() {
        let mapping = line?;
        if let Some((short, long)) = mapping.split_once(',') {
            if short == short_url {
                return Ok(Some(long.to_owned()));
            }
        }
    }
    Ok(None)
}

// Return the short URL already mapped to a given long URL, if any.
fn lookup_long(mapping_path: &str, long_url: &str) -> Option<String> {
    let reader = BufReader::new(File::open(mapping_path).ok()?);
    for line in reader.lines() {
        let mapping = line.ok()?;
        if let Some((short, long)) = mapping.split_once(',') {
            if long == long_url {
                return Some(short.to_owned());
            }
        }
    }
    None
}
