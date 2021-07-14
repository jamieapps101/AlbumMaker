use std::path::PathBuf;


pub struct ActionRecord {
    dir: PathBuf,
    sub_dirs: Vec<ActionRecord>,
    photos: Vec<PhotoAction>,
}

impl ActionRecord {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            dir: path.clone(),
            sub_dirs: Vec::new(),
            photos: Vec::new(),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        return self.dir.clone();
    }

    pub fn add_photo_action(&mut self, pa: PhotoAction) {
        self.photos.push(pa);
    }

    pub fn add_subdir_action(&mut self, sda: ActionRecord) {
        self.sub_dirs.push(sda);
    }

    pub fn get_subdirs(&self) -> &[ActionRecord] {
        return &(self.sub_dirs)
    }

    pub fn get_photos(&self) -> &[PhotoAction] {
        return &(self.photos)
    }
}

pub struct PhotoAction {
    dir:       PathBuf,
    actual:    PathBuf,
    downsized: PathBuf,
}

impl PhotoAction {
    pub fn new(dir: PathBuf, actual: PathBuf, downsized: PathBuf) -> Self {
        PhotoAction {dir, actual, downsized}
    }

    pub fn get_actual(&self) -> PathBuf {
        self.actual.clone()
    }

    pub fn get_downsized(&self) -> PathBuf {
        self.downsized.clone()
    }

    pub fn get_dir(&self) -> PathBuf {
        return self.dir.clone();
    }
}

use image::{
    GenericImageView,
    imageops::FilterType,
};

// file system manipulation
pub fn downsize_image(in_file: &PathBuf, out_file: &PathBuf, width: u32) {
    if in_file.exists() && out_file.exists() {
        // println!("\tBoth exist");
        let in_file_creation_time = in_file.metadata().unwrap().created().unwrap();
        let out_file_creation_time = out_file.metadata().unwrap().created().unwrap();
        if let Err(_reason) = in_file_creation_time.duration_since(out_file_creation_time)  {
            // the file to be converted already seems to have been converted
            // the downsized file is newer than the original, so lets not
            // waste time
            println!("> {:?} - up to date", in_file.file_name().unwrap());
            return;
        }
    }
    println!("> {:?} - rendering", in_file.file_name().unwrap());
    let img = image::open(in_file).unwrap();
    let (img_height,img_width) = img.dimensions();
    let aspect_ratio : f32 = (img_height as f32)/(img_width as f32);
    let new_height: u32 = (aspect_ratio*(width as f32)) as u32;
    let resized_image = img.resize(width, new_height, FilterType::Triangle);
    resized_image.save(out_file).unwrap();
}

pub fn get_cache_dir_path<'a>(original_path: &'a PathBuf, cache_dir_name: &str) -> PathBuf {
    let file_name = original_path.file_name().unwrap();
    let file_path = original_path.parent().unwrap();
    let new_path = file_path.join(cache_dir_name).join(file_name);
    // println!("get_cache_dir_path");
    // println!("\t{:?}",original_path);
    // println!("\t{:?}",new_path);
    return new_path;
}

pub fn is_image_file(original_path: &PathBuf) -> bool {
    let file_extension = original_path.extension().unwrap().to_str().unwrap();
    for cand in ["jpg","jpeg","png"].iter() {
        if *cand == file_extension {
            return true;
        }
    }
    return false;
}

pub fn is_html_file(original_path: &PathBuf) -> bool {
    let file_extension = original_path.extension().unwrap().to_str().unwrap();
    if ".html" == file_extension {
        return true;
    } else {
        return false;
    }
}