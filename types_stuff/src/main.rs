#![allow(unused_variables)]

use std::{
    fmt::Display,
    fmt::{self, Formatter},
};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "File {{ name: {}, data: {}, state: {} }}",
            self.name,
            self.data.len(),
            self.state
        )
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data;
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open to read"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}
fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f3_data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let mut f3 = File::new_with_data("f3", f3_data);
    let mut buffer: Vec<u8> = vec![];
    f3 = open(f3).unwrap();
    let f3_length = f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();
    let text = String::from_utf8_lossy(&buffer);
    println!("f3: {:?}", f3);
    println!("{} is {} bytes long", f3.name, f3_length);
    println!("text: {:?}", text);
}
