
use std::process::Command;
use std::path::Path;


pub fn ffmpeg_available() -> bool {
    command_available("ffmpegthumbnailer")
}


fn command_available(cmd: &str) -> bool {
    let check_result = Command::new("which")
            .arg(cmd)
            .output();
    if let Ok(output) = check_result {
        if output.status.code().unwrap_or(1) == 0 {
            return true;
        }
    }
    return false;
} 

pub fn create_thumbnail<P: AsRef<Path>>(input_file: P, output_file: P, width: u32) {

    let command_output = Command::new("ffmpegthumbnailer")
                            .args(&["-i", input_file.as_ref().to_str().unwrap()])
                            .args(&["-o", output_file.as_ref().to_str().unwrap()])
                            .arg(format!("-s {}", width))
                            .output();

    if command_output.is_ok() {
        println!("> {:?} - rendering", input_file.as_ref().file_name().unwrap());
    } else {
        println!("> {:?} - could not process", input_file.as_ref().file_name().unwrap());
    }
}

// wrapper to match the util non-ffmpeg function 
pub fn downsize_image<P: AsRef<Path>>(input_file: P, output_file: P, width: u32, force_regen:bool) {
    let in_file = input_file.as_ref();
    let out_file = output_file.as_ref();
    if !force_regen && in_file.exists() && out_file.exists() {
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
    create_thumbnail(in_file, out_file, width); 
}


// its ffmpeg, its probably compatible
#[allow(dead_code)]
pub fn is_compatible<P: AsRef<Path>>(_input_file: P) -> bool {
    unimplemented!();
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn command_available_test() {
        assert_eq!(command_available("not_command"),false);
        assert_eq!(command_available("echo"),true);
    }

}


