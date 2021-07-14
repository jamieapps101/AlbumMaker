use std::{
    path::PathBuf,
    env,
};
use clap::{Arg, App};

mod html_builder;
mod html_generation;
mod util;
mod layer_hander;

use layer_hander::*;

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

    //set number of threads for rayon
    let n_threads : u32 = matches.value_of("threads").unwrap_or_default().parse().unwrap();
    let n_threads_str = format!("{}",n_threads);
    env::set_var("RAYON_NUM_THREADS", n_threads_str);

    let clean : bool = match matches.index_of("clean") {
        Some(_count) => true,
        None => false,
    };
                    
    let tld = matches.value_of("dir").unwrap_or_default();
    let top_level_path = PathBuf::from(&tld).canonicalize().unwrap();
    let search_depth : usize = 
        match matches.value_of("depth").unwrap_or_default().parse() {
            Ok(value) => value,
            Err(_) => panic!("did not understand depth arguement"),
        };
    let resources_path = PathBuf::from("/home/jamie/workspace/projects/album_maker/resources");
    let _fs = handle_layer(&top_level_path, 0, search_depth,clean,&resources_path);
}


#[derive(Clone,Debug)]
struct FSBranch {
    path: PathBuf,
    sub_dirs: Vec<FSBranch>
}



#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn test_on_test_files() {
        let n_threads : u32 = 4;
        let n_threads_str = format!("{}",n_threads);
        env::set_var("RAYON_NUM_THREADS", n_threads_str);

        println!("RAYON_NUM_THREADS: {:?}",env::var("RAYON_NUM_THREADS").unwrap());

        let test_files_path = PathBuf::from("./test_files").canonicalize().unwrap();
        let search_depth = 2;
        let resources_path = PathBuf::from("./resources");
        let _fs = handle_layer(&test_files_path, 0, search_depth,false,&resources_path);
    }
}