# rsrch
`rsrch` is a multi-threaded file finder utility program for Linux systems, written in Rust. It looks up files containing a specified text in:
  - the file path name
    ```
    $ rsrch name text
    /home/user/Desktop/text 19
    ```
  - file contents
    ```
    $ rsrch name text
    found /home/user/Desktop/file at 0
    $ cat /home/user/Desktop/file
    text
    ```
In both cases it returns both the file path for which it matched the string given, and the byte position of the string withing the file or the file path.
By default the search is done linearly by the program, but by adding the ```-p``` or ```--parallelize``` arguments the search can be parallelized.

## Why rsrch
in addition to there always being use for file finding programs for operating systems, `rsrch` provides an opportunity to combine some of my favorite chapters of computer science. Those being:
  - finite automata
  - multi-threading
  - Rust

## Run
To run the program it is only neccessary to clone this repo, build the cargo, and then run the released executable.
```
$ git clone https://github.com/kickhead13/rsrch.git
$ cd rsrch
$ cargo build --release
$ ./target/release/rsrch mode --help
```

## How it works
The program implements a finite automata for the string given by the user. Which in code, looks like this:
```rs
let positions_matched: Vec<usize> = Automata::new(pattern.to_string()).eval(file_content.to_string());
```
Then it recursively looks through the directories annd matches either the file name of its contents depending on the mode chosen byt the user. This is done using the function:
```rs
pub fn recursion_search(mode: &ExecMode, path: &String, pattern: &String) -> ();
```
The parallelization is only on the first level of the path tree, meaning there is a thread assigned for each subdirectory of the directory where the search starts. This implementation is not yet CPU specific meaning it doesn't take in account the number of cores available for the system. This might mean that for certain directory structures the parallelized search could be slower.
Also, while searching through file contents, if encountering files too big, the program might crash as of current version. This is due to there not being a buffered read of files yet implemented.
