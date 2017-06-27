#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

/// A tool for interacting with commandline arguments 
/// in a simple, UNIX-like fashion. That is, positional 
/// arguments correspond to filenames. In the case of no positional
/// arguments, stdin is instead read from.
mod unixargs {
    extern crate std;

    use std::env::{args, Args};
    use std::path::{Path, PathBuf};
    use std::io::{stdin, BufReader, Read};
    use std::fs::File;

    pub struct UnixArgs {
        filenames: Vec<PathBuf>
    }

    pub fn parse_args_from(mut args: Args) -> Option<UnixArgs> {
        let executable_path_arg = match args.next() {
            Some(path) => path,
            None => return None
        };

        let executable_path = Path::new(&executable_path_arg);

        if !executable_path.exists() {
            return None;
        }

        let mut filenames = Vec::new();
        for argument in args {
            filenames.push(executable_path.join(argument));
        }

        Some(UnixArgs::new(filenames))
    }

    pub fn parse_args() -> Option<UnixArgs> {
        parse_args_from(args())
    }

    impl UnixArgs {
        pub fn new(paths: Vec<PathBuf>) -> UnixArgs {
            UnixArgs {
                filenames: paths
            }
        }

        pub fn read_text(mut self: UnixArgs) -> std::io::Result<String> {
            self.read_input()
        }

        fn read_input(mut self: UnixArgs) -> std::io::Result<String> {
            if self.filenames.len() > 0 {
                self.read_input_from_files()
            }
            else {
                self.read_input_from_stdin()
            }
        }

        fn read_input_from_files(mut self: UnixArgs) -> std::io::Result<String> {
            let mut result = String::new();
            for filename in self.filenames {
                let file = File::open(filename.as_path())?;
                let mut buf_reader = BufReader::new(file);
                buf_reader.read_to_string(&mut result)?;
            }

            Ok(result)
        }

        fn read_input_from_stdin(self: &mut UnixArgs) -> std::io::Result<String> {
            let mut result = String::new();
            stdin().read_to_string(&mut result)?;
            Ok(result)
        }

        pub fn has_filenames(self: &UnixArgs) -> bool {
            self.filenames.len() > 0
        }

        pub fn get_filenames(self: UnixArgs) -> Vec<PathBuf> {
            self.filenames
        }
    }
}