use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    match fs::metadata(&args[1]) {
        Err(why) => panic!("failed to get file {}", why),
        Ok(metadata) => {
            let file_type = metadata.file_type();

            if file_type.is_dir() == true {
                println!("file type : directory");
            } else if file_type.is_file() == true {
                println!("file type : file");
            } else if file_type.is_block_device() == true {
                println!("file type : block device");
            }
        },
    };
}
