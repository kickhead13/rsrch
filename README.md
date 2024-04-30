# rsrch

## Description

``rsrch`` is a multithreaded file finder command-line utility program for Unix-like Operating Systems written exclusively in the Rust programming language. It gives the user the possibilty to querry through their files on two different modes of execution:
 - querry through file names:
 ```sh
 $ rsrch name ".rs"
found in ./src/automata.rs at 14 
found in ./src/cl_handler.rs at 16 
found in ./src/main.rs at 10 
found in ./src/output_formatting.rs at 23 
found in ./src/file_search.rs at 17 
 ```
 - querry through file contents
 ```sh
 $ rsrch content "automata"
found in ./README.md at 357 
found in ./target/debug/rsrch.d at 98 
found in ./target/debug/deps/rsrch-9dbb208e2ad9939d.d at 114 286 379 
found in ./target/debug/.fingerprint/rsrch-9dbb208e2ad9939d/dep-bin-rsrch at 52 
found in ./src/main.rs at 139 
found in ./src/output_formatting.rs at 11 
 ```

For additional features, such as parallelization, run ```rsrch --help```.

## Implementation

The matching algorithm is based on String Matching Automata, with a personal implemenatation of the structures and routines needed for the algorithm.

## Installation

Just clone the repo

```sh
 $ git clone "https://github.com/kickhead13/rsrch.git"
```

build the program

```sh
 $ cd rsrch
 $ cargo build --release
```

and run the executable

```sh
 $ ./target/release/rsrch --help
```

It's that easy!
