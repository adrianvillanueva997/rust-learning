struct File;

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File;
    let mut buffer = vec![];
    let n_bytes: usize = f.read(&mut buffer).unwrap();
    println!("{} bytes read", n_bytes)
}
