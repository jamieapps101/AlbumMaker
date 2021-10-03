use std::{
    fs,
    io,
    fs::DirEntry,
    path::PathBuf,
};

use rayon::prelude::*;

use super::util::*;
use super::html_generation::*;

pub fn handle_layer(path: &PathBuf, current_depth: usize, max_depth: usize,clean: bool,resources_path: &PathBuf,make_local:bool) -> Option<ActionRecord> {
    println!("starting work in {:?}",path);
    if current_depth == max_depth+1 {
        return None;
    }
    let mut action_record = ActionRecord::new(path);
    // look for existing cache dir
    let cache_dir_path = path.join("cacheDir");
    if clean {
        if cache_dir_path.exists() {
            // delete it
            fs::remove_dir_all(cache_dir_path.clone()).unwrap();
        }
        // make cache dir
        fs::create_dir(cache_dir_path.clone()).unwrap();
    } else if !cache_dir_path.exists() {
        // make cache dir
        fs::create_dir(cache_dir_path.clone()).unwrap();
    }

    // handle previous versions of this program
    let prev_cache_dir_name = path.join("imageCache");
    if prev_cache_dir_name.exists() {
        fs::remove_dir_all(prev_cache_dir_name).unwrap();
    }

    // first isolate all the directories and files in this dir
    let entries : Vec<Result<fs::DirEntry, io::Error>> = fs::read_dir(path).unwrap().collect();
    let mut directories : Vec<&DirEntry> = entries.iter().filter_map(|entry_res|
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
    directories.sort_by(|a,b| {
        a.file_name().to_str().unwrap().to_lowercase().cmp(&b.file_name().to_str().unwrap().to_lowercase())
    });
    for directory in directories {
        if directory.file_name()!="cacheDir" {
            // is dir -> recurse
            if let Some(action) = handle_layer(&directory.path(),current_depth+1,max_depth,clean,resources_path,make_local) {
                action_record.add_subdir_action(action);
            }
        }
    }


    // once directories are finished, apply rayon to allow multi-threaded downsampling
    // to images in this directory
    let mut pas : Vec<PhotoAction> = files.par_iter().filter_map(|file| {
        // this needs to be canonicalisaed
        let abs_file_path = file.path();
        if !abs_file_path.is_absolute() {
            panic!("this should be abs {:?}",abs_file_path);
        }

        let containing_dir : PathBuf = abs_file_path.parent().unwrap().into();
        let relative_path  : PathBuf = abs_file_path.file_name().unwrap().into();

        if is_image_file(&abs_file_path) {
            // is photo 
            let abs_cache_path      = get_cache_dir_path(&abs_file_path, "cacheDir");
            let relative_cache_path = get_cache_dir_path(&relative_path, "cacheDir");
            // make record
            let pa = PhotoAction::new(containing_dir, relative_path,relative_cache_path);
            // action_record.add_photo_action(pa);
            // downsize, save in cache dir
            downsize_image(&abs_file_path, &abs_cache_path, 500);
            return Some(pa);
        } else if is_html_file(&abs_file_path) {
            // is html -> delete
            fs::remove_file(abs_file_path).unwrap();
            return None;
        } else {
            return None;
        }
    }).collect();
    pas.sort_by(|a, b| {
        a.get_name().unwrap().to_lowercase().cmp(&b.get_name().unwrap().to_lowercase())
        // a.to_lowercase().cmp(&b.to_lowercase()
    });
    for pa in pas  {
        action_record.add_photo_action(pa);
    }

    if make_local {
        // todo, copy js+css from resources path into the cache dir
        // mabe check if theres a new version first
        fs::copy(resources_path.join("main.js"),    cache_dir_path.clone()).unwrap();
        fs::copy(resources_path.join("styles.css"), cache_dir_path).unwrap();
    }

    if current_depth==0 {
        // if this is the first layer
        // create html
        let mut file_path = path.clone();
        file_path.push("index.html");
        create_html_index(&file_path, &action_record,resources_path,make_local);
        // copy over the css

        // return status
        return None;
    } else {
        // if this is not the first layer
        // create html
        let mut file_path = path.clone();
        file_path.push("index.html");
        create_html_index(&file_path, &action_record,resources_path,make_local);
        // return status
        return Some(action_record);
    }
}