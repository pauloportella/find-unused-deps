# Find Unused Deps
This is a command line tool that counts the number of times each of the dependencies specified in the package.json file of a project are imported in the project's JavaScript or TypeScript code. It allows you to identify which dependencies are not being used in your code and can potentially be removed.

![2022-12-28 12 14 22](https://user-images.githubusercontent.com/22947229/209810733-7b425bbb-2846-4d5f-b815-8a7f69f44736.gif)

## Prerequisites
Before using this tool, you will need to have Rust installed on your machine.

## Installation
To use this tool, clone this repository and build the project using cargo build --release. This will create an executable file in the target/release directory.

To make the tool available globally, you can add the executable to your system's bin directory. On Unix-like systems, this is typically /usr/local/bin. To do this, run the following command:


```
cp target/release/find-unused-deps /usr/local/bin/find-unused-deps
```
This will allow you to use the find-unused-deps command from any directory on your machine.

## Usage
To use the tool, navigate to the root directory of your project and run the following command:

```
find-unused-deps -p <path-to-project>
```
For example, if the root directory of your project is ~/my-project, you would run:

```
find-unused-deps -p ~/my-project
```
The tool will search the project directory and its subdirectories for JavaScript and TypeScript files, and count the number of times each dependency is imported. The results will be printed in a table, showing the dependency name and the number of times it was imported.

## Additional Arguments
-h or --help: Display a help message and exit.

## Dependencies
This tool uses the following dependencies:

clap: for parsing command line arguments
glob: for searching the project directory for files
rayon: for parallel processing of files
regex: for searching the contents of files for import statements
serde: for parsing the package.json file
prettytable: for printing the results in a table
indicatif: for displaying a progress bar
walkdir: for traversing the project directory and its subdirectories

## License
This project is licensed under the MIT License - see the LICENSE file for details.
