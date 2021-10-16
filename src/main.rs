use arbitor::filetype::Check;
use arbitor::formats::Jpeg;


fn main() {
   let check = Check::new("./tests/imgages/test.jpg".to_owned());
   match check.jpeg() {
    Some(x) => match x {
        Jpeg::JPEGraw => println!("It's JPG Raw!"),
        Jpeg::Jpeg2000 => println!("It's a JPG 2000!")
    },
    None => println!("This is not a valid Jpeg image")
   }
}
