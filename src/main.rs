use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use iui::prelude::*;
use iui::controls::{VerticalBox, Label, Button, LayoutGrid, GridAlignment, GridExpand, HorizontalSeparator, Group, HorizontalBox};

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
            let mut old_copy_path_string = String::from(path.parent().unwrap().to_str().unwrap())+"/";
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

fn patch_executable(path: &Path) -> Result<(), &'static str> {
    // Open executable for read
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

fn run_cli() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = Path::new(&args[1]);
        patch_executable(path)
    }

    else if args.len() < 2 {
        Err("Not enough arguments! Please specify the executable path!")
    }
    else {
        Err("Too many arguments!")
    }
}

fn run_gui() {
    let ui = UI::init().unwrap();

    let mut grid = LayoutGrid::new(&ui);
    grid.set_padded(&ui, true);

    // Create the input controls
    let mut button = Button::new(&ui, "Patch executable");
    let mut quit_button = Button::new(&ui, "Quit");
    let label = Label::new(&ui, "Lego Games Patcher");
    let label_about = Label::new(&ui, "This program patches TT Lego Game executables so custom DAT content\nand extracted DAT content can be loaded. This program can be found at\nGitHub at https://github.com/morgenm/LegoGamesPatcher and is licensed\nunder the MIT License.");

    // Set up the application's layout
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut window = Window::new(&ui, &format!("Lego Games Patcher {}", VERSION), 300, 200, WindowType::NoMenubar);
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);
    vbox.append(&ui, label_about.clone(), LayoutStrategy::Compact);

    let mut group_vbox = VerticalBox::new(&ui);
    let mut group = Group::new(&ui, "");
    group_vbox.append(&ui, button.clone(), LayoutStrategy::Compact);
    group_vbox.append(&ui, quit_button.clone(), LayoutStrategy::Compact);
    group.set_child(&ui, group_vbox);
    vbox.append(&ui, group, LayoutStrategy::Stretchy);

    window.set_child(&ui, vbox);
    window.show(&ui);
    
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    // Patch button gets path and patches the chosen file
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            if let path = Path::new(&window.open_file(&ui).unwrap()) {
                match patch_executable(&path) {
                    Ok(()) => {
                        window.modal_msg(&ui, "Success", &format!("Successfully patched file {:?}!", path));
                    }
                    Err(e) => {
                        window.modal_err(&ui, "Error Patching", &format!("Could not patch file {:?}! Error: {}", path, e));
                    }
                }
            }
            else {
            }
        }
    });

    ui.main();
}

fn main() -> Result<(), &'static str> {
    run_gui();
    Ok(())
}
