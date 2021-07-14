use std::{
    fs,
    io,
    fs::DirEntry,
    path::PathBuf,
};

use rayon::prelude::*;

use super::util::*;
use super::html_generation::*;

pub fn handle_layer(path: &PathBuf, current_depth: usize, max_depth: usize) -> Option<ActionRecord> {
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

    // first isolate all the directories and files in this dir
    let entries : Vec<Result<fs::DirEntry, io::Error>> = fs::read_dir(path).unwrap().collect();
    let directories : Vec<&DirEntry> = entries.iter().filter_map(|entry_res|
        if let Ok(d_entry) = entry_res {
            if d_entry.file_type().unwrap().is_dir() {
                return Some(d_entry)
            } else {
                return None;
            }
        } else {
            return None;
        }
    ).collect();
    let files : Vec<&DirEntry> = entries.iter().filter_map(|entry_res|
        if let Ok(d_entry) = entry_res {
            if d_entry.file_type().unwrap().is_file() {
                return Some(d_entry)
            } else {
                return None;
            }
        } else {
            return None;
        }
    ).collect();

    // recursively act on all the directories
    for directory in directories {
        if directory.file_name()!="cacheDir" {
            // is dir -> recurse
            if let Some(action) = handle_layer(&directory.path(),current_depth+1,max_depth) {
                action_record.add_subdir_action(action);
            }
        }
    }

    // once directories are finished, apply rayon to allow multi-threaded downsampling
    let pas : Vec<PhotoAction> = files.par_iter().filter_map(|file| {
        let file_path = file.path();
        if is_image_file(&file_path) {
            // is photo 
            let cache_path = get_cache_dir_path(&file_path, "cacheDir");
            // make record
            let pa = PhotoAction::new(file_path.clone(),cache_path.clone());
            // action_record.add_photo_action(pa);
            // downsize, save in cache dir
            downsize_image(&file_path, &cache_path, 300);
            return Some(pa);
        } else if is_html_file(&file_path) {
            // is html -> delete
            fs::remove_file(file_path).unwrap();
            return None;
        } else {
            return None;
        }
    }).collect();
    for pa in pas  {
        action_record.add_photo_action(pa);
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