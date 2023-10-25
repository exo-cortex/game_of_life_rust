# An implementation of "Conway's Game of Life" written in Rust.

▄  ▀▀                ▄▀▀▄         ▄▀▄ ▄  █ ▄▄      ▄           ▀
 ▀              █▄▄   ▄ █▄         ▀ ▀  ▀ ▀▄▄▀   ▄█▀▀▄          
         ▀▀█    ▄▄ ▄   ▀                       ▄█   ▀▀          
          ▀    ▀  ▀ ██                        ▀██           ▄▄▄ 
                   ▀▀         █                 ▀▄▄ ▀█▄ ▄▄███▄▄▀
             ▄▄▀█▄▀                               ▀█▀  ▄▀▀      
 █▄          ▀▀ ▄▄▄           █▄█                  ▄ ▄▄▄▄       
 ▄█▀ ▄▀▄     ▄▀█ █ ██                            ▀▀▄▀▀  ▀       
▄▀  ▄▀  █▄   ▀█▀█▀▀▀                                  ▄▄   ▄    
     ▀  ▄      ▀▀        ▄                           ▀▄ ▀▄ ▀▄█  
   ▄▀▀█▀                ▀▄    ▄                   █   ▀▀██ ▀▀▀  
   █  ▀              ▄▄▄    ▄█      ▀█                          
▄█▀ ▄▀                   ▄▄▄   ▀   ▀ ██▄                        
▀▀ ▀▀                    ▀▀ ▄ █     ▀▄▄▄               ▄▄▀██▀▄█▀
  ▄ ▀▄     ▄             █▀ ▄ ▄   ▄██▀██                ▀▀▀▀▀▀▀ 
▄▀   █      ▀▀       ▄    ▀▄   ▄█▀ ▄▄ ▀  ▀                      



This program should run a game of life simulation and display the
"playground" in your terminal window every 100ms (see above)

## How to use

with rustup installed go into the project folder and execute
```bash
$ cargo run --release
```

cargo should then
- download needed dependencies
- compile everything with optimizations ("--release")
- run the program

Cargo's `build` command compiles the project and puts it into a `target` directory.
The release version gets put into `target/release`, the normal version (without `--release`) is put into `target/debug`. Those binaries can be run directly as well.

## Basic Rust usage
`Cargo.toml` is the package's configuration file. Here the name of the project, dependencies and build options are specified.

`cargo` is Rust's build system and package manager. Useful commands are
- `cargo check`: perform basic compiler checks)
- `cargo build`: compile the project - use `--release` for optimized release version
- `cargo run`: compile project and execute it - `--release` also works here.

There exist numerous additional commands that can be installed as well.