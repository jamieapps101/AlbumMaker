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
                    .arg(Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Force regeneration of files, ignoring timestaps"))
                    .arg(Arg::with_name("depth")
                        .long("depth")
                        .value_name("DEPTH")
                        .help("Sets the maximum depth to search for photos")
                        .takes_value(true)
                        .default_value("5"))
                    .arg(Arg::with_name("threads")
                        .long("threads")
                        .value_name("THREADS")
                        .help("Sets the max number of threads used for downsizing images")
                        .takes_value(true)
                        .default_value("1"))
                    .arg(Arg::with_name("clean")
                        .long("clean")
                        .help("Removes artifacts from this program, overides all other args"))
                    .arg(Arg::with_name("local")
                        .long("local")
                        .help("Inserts js+css files into each directory, to allow other computers to access if shared over a network"))
                    .get_matches();

    //set number of threads for rayon
    let n_threads : u32 = matches.value_of("threads").unwrap_or_default().parse().unwrap();
    let n_threads_str = format!("{}",n_threads);
    env::set_var("RAYON_NUM_THREADS", n_threads_str);

    let clean : bool = match matches.index_of("clean") {
        Some(_count) => true,
        None => false,
    };

    let local : bool = match matches.index_of("local") {
        Some(_count) => true,
        None => false,
    };

    let force_regen : bool = match matches.index_of("force") {
        Some(_count) => true,
        None => false,
    };

    println!("force_regen: {:?}",force_regen);
                    
    let tld = matches.value_of("dir").unwrap_or_default();
    let top_level_path = PathBuf::from(&tld).canonicalize().unwrap();
    let search_depth : usize = 
        match matches.value_of("depth").unwrap_or_default().parse() {
            Ok(value) => value,
            Err(_) => panic!("did not understand depth arguement"),
        };


    let resources_path: PathBuf = match env::var_os("RESOURCE_PATH") {
        Some(path) => {
            PathBuf::from(path)
        },
        None => {
            let exe_path = env::current_exe().unwrap();
            let cand_resource_path = exe_path.parent().unwrap().join("resources");
            if cand_resource_path.exists() {
                cand_resource_path
            } else {
                PathBuf::from("/home/jamie/workspace/projects/AlbumMaker/resources")
            }
        }
    };
    

    println!("resources_path: {:?}",resources_path);


    let _fs = handle_layer(&top_level_path, 0, search_depth,clean,&resources_path,local,force_regen);
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
        let _fs = handle_layer(&test_files_path, 0, search_depth,false,&resources_path,false,false);
    }
}