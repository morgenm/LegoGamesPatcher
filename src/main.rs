use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

mod patch;

fn copy_to_old(path: &Path) {
    let old_copy_path = match path.extension() {
        Some(ext) => { // File has an extension
            // Copy old executable to _old
            let mut old_copy_name = String::from(path.file_name().unwrap().to_str().unwrap()); // Just the filename
            let mut new_ext = String::from("_old."); 
            new_ext += ext.to_str().unwrap(); // Add extension (.exe or whatever) to the renamed filename
            let mut old_ext = String::from(".");
            old_ext += ext.to_str().unwrap();
            old_copy_name = old_copy_name.replace(&old_ext, &new_ext); // Replace the original extension (.exe to _old.exe)
            let mut old_copy_path_string = String::from(path.parent().unwrap().to_str().unwrap());
            old_copy_path_string += &old_copy_name;

            old_copy_path_string
        }
        None => { // File has no extension, just add _old
            let mut old_copy_path_string = String::from(path.to_str().unwrap());
            old_copy_path_string += "_old";
            
            old_copy_path_string
        }
    };
    
    println!("Copying old file to {}",old_copy_path);
    std::fs::rename(path, old_copy_path).unwrap();
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // Open executable for read
        let path = Path::new(&args[1]);
        let mut file = match File::open(&path) {
            Err(_) => { return Err("Could not open file") },
            Ok(f) => f,
        };
        
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let mut patched = false;

        match patch::get_game_type(&buf) {
            Some((gt, file_pos)) => {
                match gt {
                    patch::GameType::TSS => {
                        println!("Detected that the executable is The Skywalker Saga!");

                        // Actual patch occurs here
                        patch::patch_tss_buf(&mut buf, file_pos);

                        // Copy old file
                        copy_to_old(&path);
                        

                        // Write the patched file file
                        let mut patched_file = match File::create(&path) {
                            Ok(f) => f,
                            Err(_) => { return Err("Could not open file") },
                        };

                        match patched_file.write_all(&buf) {
                            Ok(_) => {}
                            Err(_) => { return Err("Error writing new patched file!")}
                        }

                        patched = true;
                    }
                    patch::GameType::Other => {
                        println!("Detected that the executable is an older TT game!");

                        // Actual patch occurs here
                        patch::patch_other_buf(&mut buf, file_pos);

                        // Copy old file
                        copy_to_old(&path);
                        

                        // Write the patched file file
                        let mut patched_file = match File::create(&path) {
                            Ok(f) => f,
                            Err(_) => { return Err("Could not open file") },
                        };

                        match patched_file.write_all(&buf) {
                            Ok(_) => {}
                            Err(_) => { return Err("Error writing new patched file!")}
                        }
                        patched = true;
                    }
                }
            }
            None => {}
        }

        return match patched {
            true => {
                println!("Successfully patched!");
                Ok(())
            }
            false => {
                Err("The executable does not seem to be supported")
            }
        }

    }
    else if args.len() < 2 {
        println!("Not enough arguments were passed! Please specify the executable's location.");
        Err("Not enough args")
    }
    else {
        Err("Too many args")
    }
}
