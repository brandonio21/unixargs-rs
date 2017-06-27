unixargs-rs
===========
Simple UNIX-style args for Rust.


Philosophy
----------
A program is called with positional arguments which are filenames. There may
be more than one argument. In the case that there are no filename arguments,
input is instead read from stdin.

Usage
-----
As an example, let's implement `cat`
```rust
match parse_args() {
    Some(args) => match args.read_text() {
        Ok(text) => println!("{}", text),
	Err(why) => panic!(why)
    },
    None => panic!("Could not read arguments")
}
```
