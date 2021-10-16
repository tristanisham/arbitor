use crate::formats::Jpeg;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

impl Jpeg {
    pub(crate) fn check(file: &str) -> Result<Option<Self>, String> {
        match Self::is_raw(&file) {
            Ok(x) => match x {
                true => Ok(Some(Jpeg::JPEGraw)),
                false => match Self::is_2000(&file) {
                    true => Ok(Some(Jpeg::Jpeg2000)),
                    false => Ok(None),
                },
            },
            Err(s) => Err(s),
        }
    }
    todo!("Rewrite to actually work with the io library!");
    fn is_raw(file: &str) -> Result<bool, String> {
        let file_path = Path::new(&file);
        match File::open(file_path) {
            Ok(mut f) => {
                println!("opened file");
                let mut buffer = [0; 4];
                match f.read(&mut buffer) {
                    Ok(_) => {
                        let val = vec![255, 216, 244, 219];
                        if val == buffer {
                            return Ok(true);
                        } else {
                            return Ok(false);
                        }
                    },
                    Err(_) => Err("Unable to read bytes to Buffer!".to_owned()),
                }
            },
            Err(_) => Err("Unable to open file".to_owned()),
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
            Ok(Some(t)) => t,
            Ok(None) => super::Jpeg::Jpeg2000,
            Err(e) => println!("{}", e),
        };

        assert_eq!(2 + 2, 4);
    }
}
