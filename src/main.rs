use std::{
    fs,
    path::PathBuf,
};
use clap::{Arg, App};

mod html_builder;

mod html_generation;
use html_generation::*;

mod util;
use util::*;

fn main() {
    let matches = App::new("Album Builder")
                    .version("0.1")
                    .author("Jamie Apps")
                    .about("Creates in situ HTML photo albums")
                    .arg(Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .value_name("DIR")
                        .help("Sets the top level dir to build an album within")
                        .takes_value(true)
                        .default_value("."))
                    .arg(Arg::with_name("depth")
                        .long("depth")
                        .value_name("DEPTH")
                        .help("Sets the maximum depth to search for photos")
                        .takes_value(true)
                        .default_value("3"))
                    .arg(Arg::with_name("threads")
                        .long("threads")
                        .value_name("THREADS")
                        .help("Sets the max number of threads used for downsizing images")
                        .takes_value(true)
                        .default_value("1"))
                    .arg(Arg::with_name("clean")
                        .long("clean")
                        .help("Removes artifacts from this program, overides all other args"))
                    .get_matches();


    let tld = matches.value_of("dir").unwrap_or_default();
    let top_level_path = PathBuf::from(&tld);

    let search_depth : usize = 
        match matches.value_of("depth").unwrap_or_default().parse() {
            Ok(value) => value,
            Err(_) => panic!("did not understand depth arguement"),
        };


    println!("tld: {:?}",tld);
    println!("max depth: {:?}",search_depth);

    let _fs = handle_layer(&top_level_path, 0, search_depth);
}


#[derive(Clone,Debug)]
struct FSBranch {
    path: PathBuf,
    sub_dirs: Vec<FSBranch>
}

fn handle_layer(path: &PathBuf, current_depth: usize, max_depth: usize) -> Option<ActionRecord> {
    println!("handle layer on path: {:?}",path);
    if current_depth == max_depth+1 {
        return None;
    }
    let mut action_record = ActionRecord::new();
    // look for existing cache dir
    let cache_dir_path = path.join("cacheDir");
    if cache_dir_path.exists() {
        // delete it
        fs::remove_dir_all(cache_dir_path.clone()).unwrap();
    }
    // make cache dir
    fs::create_dir(cache_dir_path).unwrap();

    // iterate over every dir entry
    for d_entry_res in fs::read_dir(path).unwrap() {
        if let Ok(d_entry) = d_entry_res {
            let f_type = d_entry.file_type().unwrap();
            if f_type.is_dir() {
                if d_entry.file_name()!="cacheDir" {
                    // is dir -> recurse
                    if let Some(action) = handle_layer(&d_entry.path(),current_depth+1,max_depth) {
                        action_record.add_subdir_action(action);
                    }
                }
            } else if f_type.is_file() {
                let file_path = d_entry.path();

                if is_image_file(&file_path) {
                    // is photo 
                    let cache_path = get_cache_dir_path(&file_path, "cacheDir");
                    // make record
                    action_record.add_photo_action(PhotoAction::new(file_path.clone(),cache_path.clone()));
                    // downsize, save in cache dir
                    downsize_image(&file_path, &cache_path, 300);
                } else if is_html_file(&file_path) {
                    // is html -> delete
                    fs::remove_file(file_path).unwrap();
                }
            } else {
                panic!("this is a symlink!");
            }
        }
    }
    if current_depth==0 {
        // if this is the first layer
        // create html
        let mut file_path = path.clone();
        file_path.push("index.html");
        create_html_index(&file_path, action_record);
        // return status
        return None;
    } else {
        // if this is not the first layer
        // create html
        // return status
    }

    return None;
}

#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn test_on_test_files() {
        let test_files_path = PathBuf::from("./test_files");
        let search_depth = 2;
        let _fs = handle_layer(&test_files_path, 0, search_depth);
    }
}