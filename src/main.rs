use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io;


fn main() {
    let resp_result  = reqwest::get("https://cdn.bulbagarden.net/upload/thumb/2/24/474Porygon-Z.png/250px-474Porygon-Z.png");

    match resp_result {
        Ok(mut resp) => {
            let path = Path::new("porygon.png");

            match File::create(&path) {
                Err(why) => {
                    let display = path.display();

                    panic!("couldn't create {}: {}", display, why.description())
                },
                Ok(mut file) => {
                    match io::copy(&mut resp, &mut file) {
                        Ok(_) => {
                            println!("Success")
                        },
                        Err(error) => {
                            println!("Download error {}", error)
                        },
                    }
                },
            };
        },
        Err(error) => {
            println!("Http error {}", error)
        },
    }
}

