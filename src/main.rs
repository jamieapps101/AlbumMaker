use std::{
    fs,
    fs::File,
    path::PathBuf,
    io::{BufWriter,BufReader,BufRead,Write},
};
use regex::Regex;
use clap::{Arg, App};
use image::{
    GenericImageView,
    imageops::FilterType,
};


mod html_builder;

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
                    .get_matches();


    let tld = matches.value_of("dir").unwrap_or_default();
    let top_level_path = PathBuf::from(&tld);

    let search_depth : usize = 
        match matches.value_of("depth").unwrap_or_default().parse() {
            Ok(value) => value,
            Err(_) => panic!("did not understand depth arguement"),
        };

    let depth = get_component_count(&top_level_path);

    println!("tld: {:?}",tld);
    println!("depth: {:?}",depth);
    println!("max depth: {:?}",search_depth);

    let _fs = handle_layer(&top_level_path, 0, search_depth);
}

fn get_component_count(p: &PathBuf) -> usize {
    let mut return_count = 0;
    for _ in p.components() {
        return_count+=1;
    }
    return_count
}

#[derive(Clone,Debug)]
struct FSBranch {
    path: PathBuf,
    sub_dirs: Vec<FSBranch>
}

struct PhotoAction {
    actual: PathBuf,
    downsized: PathBuf,
}

impl PhotoAction {
    fn get_actual(&self) -> PathBuf {
        self.actual.clone()
    }

    fn get_downsized(&self) -> PathBuf {
        self.downsized.clone()
    }
}

struct ActionRecord {
    sub_dirs: Vec<ActionRecord>,
    photos: Vec<PhotoAction>,
}

impl ActionRecord {
    fn new() -> Self {
        Self {
            sub_dirs: Vec::new(),
            photos: Vec::new(),
        }
    }

    fn add_photo_action(&mut self, pa: PhotoAction) {
        self.photos.push(pa);
    }

    fn add_subdir_action(&mut self, sda: ActionRecord) {
        self.sub_dirs.push(sda);
    }
}

fn handle_layer(path: &PathBuf, current_depth: usize, max_depth: usize) -> Option<ActionRecord> {
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
                // is dir -> recurse
                if let Some(action) = handle_layer(&d_entry.path(),current_depth+1,max_depth) {
                    action_record.add_subdir_action(action);
                }
            } else if f_type.is_file() {
                let file_path = d_entry.path();

                if is_image_file(&file_path) {
                    // is photo 
                    let cache_path = get_cache_dir_path(&file_path, "cacheDir");
                    // make record
                    action_record.add_photo_action(PhotoAction {
                        actual: file_path.clone(),
                        downsized: cache_path.clone(),
                    });
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
        // return status
        return None;
    } else {
        // if this is not the first layer
        // create html
        // return status
    }

    return None;
}

// file system manipulation
fn downsize_image(in_file: &PathBuf, out_file: &PathBuf, width: u32) {
    let img = image::open(in_file).unwrap();
    let (img_height,img_width) = img.dimensions();
    let aspect_ratio : f32 = (img_height as f32)/(img_width as f32);
    let new_height: u32 = (aspect_ratio*(width as f32)) as u32;
    let resized_image = img.resize(width, new_height, FilterType::Triangle);
    resized_image.save(out_file).unwrap();
}

fn get_cache_dir_path<'a>(original_path: &'a PathBuf, cache_dir_name: &str) -> PathBuf {
    let file_name = original_path.file_name().unwrap();
    let file_path = original_path.parent().unwrap();
    let new_path = file_path.join(cache_dir_name).join(file_name);
    return new_path;
}

fn is_image_file(original_path: &PathBuf) -> bool {
    let file_extension = original_path.extension().unwrap().to_str().unwrap();
    for cand in ["jpg","jpeg","png"].iter() {
        if *cand == file_extension {
            return true;
        }
    }
    return false;
}

fn is_html_file(original_path: &PathBuf) -> bool {
    let file_extension = original_path.extension().unwrap().to_str().unwrap();
    if ".html" == file_extension {
        return true;
    } else {
        return false;
    }
}

// html manipulation
struct TemplateInformation {
    html_index:     PathBuf,
    html_per_image: PathBuf,
    html_per_dir:   PathBuf,
}


fn create_html_index(new_file: &PathBuf, ti: TemplateInformation, ar: ActionRecord) {
    // setup initial vairables
    let re_begin = Regex::new(r"<!-- begin -->").unwrap();
    // let re_end = Regex::new(r"<!-- end -->").unwrap();

    // setup writer
    let mut writer = BufWriter::new(File::create(new_file).unwrap());

    // read in the head of the index file and insert it
    let reader = BufReader::new(File::open(ti.html_index.clone()).unwrap());
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        if re_begin.is_match(line.as_str()) {
            break;
        } else {
            writer.write_all(line.as_bytes()).unwrap();
        }
    }

    // for each folder, format the dir template and insert it
    for folder_action_record in ar.sub_dirs {
        let string = format_dir_template(&ti.html_per_dir, folder_action_record);
        writer.write_all(string.as_bytes()).unwrap();
    }

    // for each image, format the image template and insert it
    for photo_action in ar.photos {
        let string = format_image_template(&ti.html_per_image, photo_action);
        writer.write_all(string.as_bytes()).unwrap();
    }

    // read in tail end of the index file and insert it
    let reader = BufReader::new(File::open(ti.html_index).unwrap());
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        writer.write_all(line.as_bytes()).unwrap();
    }
}

fn format_dir_template(html_per_dir: &PathBuf, ar: ActionRecord) -> String {
    unimplemented!();
}

fn format_image_template(html_per_dir: &PathBuf, pa: PhotoAction) -> String {
    unimplemented!();
}