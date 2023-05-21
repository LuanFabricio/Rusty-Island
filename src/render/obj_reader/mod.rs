use std::fs::File;
use std::io::BufReader;

pub struct ObjReader {
    object: obj::Obj,
}

impl ObjReader {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let input = BufReader::new(file);

        Ok(Self {
            object: obj::load_obj(input).unwrap(),
        })
    }

    pub fn get_obj(&self) -> &obj::Obj {
        &self.object
    }
}
