use std::fmt::Display;
use std::{fs::File, io::Read};
pub enum FileErrors {
    FileNotFound,
    UnableToReadFile,
}

impl Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileErrors::FileNotFound => write!(f, "File not found"),
            FileErrors::UnableToReadFile => write!(f, "Unable to read file"),
        }
    }
}

pub fn read_file_and_return_content(
    filename: &str,
    directory: &str,
) -> Result<Vec<u8>, FileErrors> {
    // let mut file = File::open(filename).expect("Unable to open file");
    let paths = std::fs::read_dir(directory).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    match File::open(format!("{}/{}", directory, filename)) {
        Ok(mut file) => {
            let mut contents: Vec<u8> = Vec::new();
            if file.read_to_end(&mut contents).is_ok() {
                return Ok(contents);
            }
            Err(FileErrors::UnableToReadFile)
        }
        Err(_) => Err(FileErrors::FileNotFound),
    }
}
