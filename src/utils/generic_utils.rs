use std::{path::{PathBuf, Path}, env, fs::File};

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return env::current_dir()
}

pub fn get_file_name_check_if_parent_dir_exists() -> String {
    let mut filename = String::new();
    let mut error = true;
    while error {
        std::io::stdin().read_line(&mut filename).unwrap();
        filename = filename.replace("\n", "");
        if filename == "" {
            return filename;
        }
        let mut path = Path::new(&filename);
        if path.is_dir() {
            filename = filename + "/ldot_stack.json";
            path = Path::new(&filename);
            println!("{:?} is a directory, appending default ldot stack file name. ({})",filename.replace("//", "/"), path.to_string_lossy());
        }

        println!("{:?} is a file", path);
        if path.is_file() && path.exists() {
            println!("File already exists, use: ldot load \"{}\"", path.to_string_lossy());
        }
        
        let _file = match File::create(&path) {
            Ok(file) => { drop(file);
                error = false;
            }
                ,
            Err(e) => {
                println!("Error creating file. {}", e.to_string());
                // std::process::exit(2);
            },
        };

        // if parent_path.exists() {
        //     error = false;
        // } else {
        //     println!("{}", parent_path.to_string_lossy());
        //     println!("Invalid path")
        // }
        filename = "".to_string();
    }
    return filename;
}