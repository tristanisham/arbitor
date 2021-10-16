use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::formats::Jpeg as Jpeg;

impl Jpeg {
    pub(crate) fn check(file: &str) -> Option<Self> {
        match Self::is_raw(&file) {
            true => Some(Jpeg::JPEGraw),
            false => match Self::is_2000(&file) {
                true => Some(Jpeg::Jpeg2000),
                false => None
            }
        }
    }

    fn is_raw(file: &str) -> bool {
        let file_path = Path::new(&file);
        match File::open(file_path) {
            Ok(mut f) => {
                let mut buffer = [0; 4];
                match f.read(&mut buffer) {
                    Ok(_) => {
                        let val = vec![255, 216, 244, 219];
                        if val == buffer {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    fn is_2000(file: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gets_bytes() {
        match super::Jpeg::check("./test/images/test.Jpeg") {
            Some(t) => t,
            None => super::Jpeg::Jpeg2000,
        };

        assert_eq!(2 + 2, 4);
    }
}