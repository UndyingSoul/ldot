use std::{path::{PathBuf, Path}, env, fs::File};

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return env::current_dir()
}

pub fn get_file_name_check_if_parent_dir_exists() -> String {
    let mut filename = String::new();
    let mut error = true;
    while error {
        get_line_from_console_allow_blank();
        if filename == "" {
            return filename;
        }
        let mut path = Path::new(&filename);
        if path.is_dir() {
            filename = filename + "/ldot_stack.json";
            path = Path::new(&filename);
            println!("{:?} is a directory, appending default ldot stack file name. ({})",filename.replace("//", "/"), path.to_string_lossy().replace("//", "/"));
        }

        //println!("{:?} is a file", path);
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
        filename = "".to_string();
    }
    return filename;
}

pub fn get_line_from_console_allow_blank() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.replace("\n", "");
    return input;
}