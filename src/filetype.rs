use crate::formats::Jpeg;


pub struct Check {
    file: String,
}

impl Check {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn jpeg(&self) -> Option<Jpeg> {
        if let Ok(x) = Jpeg::check(&self.file) {
            return x;
        } else {
            return None;
        }
    }
}


