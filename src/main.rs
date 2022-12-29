extern crate clap;
extern crate glob;
extern crate rayon;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rayon::prelude::*;
use clap::{arg, command, value_parser};
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use indicatif::ProgressBar;


#[derive(Deserialize)]
struct PackageJson {
    dependencies: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!() 
        .arg(
            arg!(
                -p --path <PATH> "Path to the project"
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .get_matches();


    // Get the path to the project
    let path = matches.get_one::<String>("path").unwrap();
    let project_path = PathBuf::from(path);

    // Check that the path exists and is a directory
    if !project_path.exists() || !project_path.is_dir() {
        return Err(format!("{} is not a valid directory", path.as_str()).into());
    }

    // Read the package.json file and extract the dependencies
    let package_json_path = format!("{}/package.json", path.as_str());
    let package_json_str = fs::read_to_string(package_json_path)?;
    let package_json: PackageJson = serde_json::from_str(&package_json_str)?;
    let dependencies = package_json.dependencies;

    // Create a map of dependencies to counts
    let mut dependency_counts = HashMap::new();
    for (dependency, _) in dependencies {
        dependency_counts.insert(dependency.to_string(), 0);
    }
    let dependency_counts = Mutex::new(dependency_counts);


    // Recursively search the project directory for JS and TS files filtering node_modules and dist
    let file_regex = Regex::new(r"(\.jsx?|\.tsx?)$")?;
    let remove_dts = Regex::new(r"(\.d.ts)$")?;
    let entries: Vec<_> = walkdir::WalkDir::new(project_path)
        .into_iter()
        .map(Result::unwrap)
        .filter(|entry| !entry.path().starts_with(format!("{}/node_modules", path)))
        .filter(|entry| !entry.path().starts_with(format!("{}/dist", path)))
        .filter(|entry| !entry.path().starts_with(format!("{}/.next", path)))
        .filter(|entry| file_regex.is_match(entry.path().to_str().unwrap()))
        .filter(|entry| !remove_dts.is_match(entry.path().to_str().unwrap()))
        .collect();
    let num_files = entries.len();

    // Set up a progress bar
    let total_progress = num_files * dependency_counts.lock().unwrap().len();
    let pb = ProgressBar::new(total_progress as u64);

    // Iterate over the files and count the number of imports for each dependency
    entries
        .par_iter()
        .enumerate()
        .for_each(|(_i, entry)| {
            let path = entry.path();

            // println!("Processing file: {}", path.to_string_lossy());

            let mut file = fs::File::open(path).unwrap();
            let mut contents = String::new();
            // handle read_to_string error gracefully
            if file.read_to_string(&mut contents).is_err() {
                return;
            }
            file.read_to_string(&mut contents).unwrap();


            for (dependency, count) in dependency_counts.lock().unwrap().iter_mut() {
                let contents: Vec<&str> = contents.split('\n').collect();
                let contents: Vec<&str> = contents.into_iter().filter(
                | line | line.contains(dependency)
                )
                .collect();

                *count += if contents.into_iter().len() > 0 { 1 } else { 0 };

                // Update the progress bar
                pb.inc(1);
            }
        });

    pb.finish_with_message("done");

    // Print the results in a colored table
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Dependency"),
        Cell::new("Count"),
    ]));
    for (dependency, count) in dependency_counts.lock().unwrap().iter_mut() {
        let row = Row::new(vec![
    Cell::new(dependency).style_spec("Fc"),
    Cell::new(&count.to_string()).style_spec(if *count > 0 { "Fg" } else { "Fr" }),
    ]);
    table.add_row(row);
    }

    table.printstd();

    Ok(())
}
