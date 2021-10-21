use super::html_builder::*;
use super::util::*;
use std::iter::FromIterator;
use std::{
    fs::File,
    path::PathBuf,
    io::BufWriter,
    str::FromStr
};


pub fn create_html_index(new_file: &PathBuf, ar: &ActionRecord, resources_path: &PathBuf,local_resources:bool) {
    
    let styles_path = if !local_resources {
        // resources_path.join("styles.css").canonicalize().unwrap()
        if let Ok(thing) = resources_path.join("styles.css").canonicalize() {
            thing
        } else {
            panic!("Error looking in\n{:?}\n",resources_path);
        }
    } else {
        PathBuf::from_str("./cacheDir/styles.css").unwrap()
    };

    let script_path = if !local_resources {
        resources_path.join("main.js").canonicalize().unwrap()
    } else {
        PathBuf::from_str("./cacheDir/main.js").unwrap()
    };

    let mut dom = HtmlDom::new();
        dom.add_element(
            HtmlElement::new(HtmlElementType::Head)
                .add_element(HtmlElement::new(HtmlElementType::Link)
                    .set_rel("stylesheet")
                    .set_href(styles_path.to_str().unwrap()))
                .add_element(HtmlElement::new(HtmlElementType::Script)
                    .set_src(script_path.to_str().unwrap()))
                .add_element(HtmlElement::new(HtmlElementType::Title)
                    .set_text("Photo Album")));



    let mut body = HtmlElement::new(HtmlElementType::Body);
    // build up the top of the body
    // --- Sub Folders ---
    if ar.get_subdirs().len() > 0 {
        body = body.add_element(HtmlElement::new(HtmlElementType::P)
                        .set_text("Sub Directories")
                    );
        // for each folder, format the dir template and insert it
        let mut list = HtmlElement::new(HtmlElementType::Div)
            .add_class("dirs_list");
        for action_record in ar.get_subdirs().iter() {
            // if let Some(element) = format_dir_template(action_record) {
            //     list = list.add_element(element);
            // }
            list = list.add_element(format_dir_template(action_record,local_resources));
        }
        body = body.add_element(list);
        body = body.add_element(HtmlElement::new(HtmlElementType::Br));
    }


    // --- Images ---
    if ar.get_photos().len() > 0 {
        body = body.add_element(HtmlElement::new(HtmlElementType::P)
            .set_text("Images")
        );
        // for each image, format the image template and insert it
        let mut list = HtmlElement::new(HtmlElementType::Div)
            .add_class("images_list");
        for photo_action in ar.get_photos() {
            list = list.add_element(format_image_template(photo_action));
        }
        body = body.add_element(list);
        // build up bottom of body
        body = body.add_element(HtmlElement::new(HtmlElementType::Br));
    }

    // render to file
    // setup writer
    let mut writer = BufWriter::new(File::create(new_file).unwrap());
    dom.add_element(body);
    dom.render(&mut writer);
}

fn format_image_template(pa: &PhotoAction) -> HtmlElement {
    let downsizes_image_location = pa.get_downsized();
    let image_name = downsizes_image_location.file_name().unwrap().to_str().unwrap();
    let he = HtmlElement::new(HtmlElementType::Div)
        .add_class("images_item")
        .add_element(HtmlElement::new(HtmlElementType::A)
            .add_class("images_link")
            .set_href(pa.get_actual().to_str().unwrap())
            .add_element(HtmlElement::new(HtmlElementType::Img)
                .set_src(pa.get_downsized().to_str().unwrap())
                .set_alt(image_name))
                .add_class("disp_img"));
    return he;
}

fn format_dir_template(ar: &ActionRecord,local_resources:bool) -> HtmlElement {

    let mut sub_folder_index_path = ar.get_path();
    sub_folder_index_path.push("index.html");

    let dir_path = ar.get_path();
    let dir_name = dir_path.file_name().unwrap().to_str().unwrap();

    if let Some(pa) = get_first_photo(ar) {
        let cover_photo_containing_path = pa.get_dir();
        let downsized_path = pa.get_downsized();
        let abs_downsized_path = cover_photo_containing_path.join(downsized_path);
        let mut cover_photo_path = abs_downsized_path;

        if !sub_folder_index_path.is_absolute() {
            let iterator  = sub_folder_index_path.components().skip(2);
            let mut temp = PathBuf::from(".");
            temp.push(PathBuf::from_iter(iterator));
            sub_folder_index_path = temp;
        }

        if local_resources {
            // subfolder path
            let temp = sub_folder_index_path.clone(); // borrow checker needs this
            let components :Vec<std::path::Component>= temp.components().collect();
            let components_count = components.len();
            let mut new_path = PathBuf::from(".");
            let sub_folder_name_component = components[components_count-2];
            new_path.push(components[components_count-2]);
            new_path.push(components[components_count-1]);
            sub_folder_index_path = new_path;

            // image path
            let components :Vec<std::path::Component>= cover_photo_path.components().collect();
            let mut new_path = PathBuf::from(".");
            let mut from_here = false;
            for component in components {
                if sub_folder_name_component == component {
                    from_here = true;
                }
                if from_here {
                    new_path.push(component);
                }
            }
            cover_photo_path = new_path;
        }

        let he = HtmlElement::new(HtmlElementType::Div)
        .add_class("dirs_item")
        .add_element(HtmlElement::new(HtmlElementType::A)
            .add_class("dirs_link")
            .set_href(sub_folder_index_path.to_str().unwrap())
            .add_element(HtmlElement::new(HtmlElementType::Img)
                .set_src(cover_photo_path.to_str().unwrap())
                .set_alt(dir_name)
                .add_class("dirs_img"))
                .add_element(HtmlElement::new(HtmlElementType::P)
                .set_text(dir_name))
            );
        return he;
    } else {
        let he = HtmlElement::new(HtmlElementType::Div)
        .add_class("dirs_item")
        .add_element(HtmlElement::new(HtmlElementType::A)
            .add_class("dirs_link")
            .add_element(HtmlElement::new(HtmlElementType::Img)
                .set_text(dir_name))
            );
        return he;
    }

}

fn get_first_photo(ar: &ActionRecord) -> Option<PhotoAction> {
    if ar.get_photos().len() > 0 {
        return Some(ar.get_photos()[0].clone())
    } else if ar.get_subdirs().len() > 0 {
        for i in 0..ar.get_subdirs().len() {
            if let Some(pa) = get_first_photo(&ar.get_subdirs()[i]) {
                return Some(pa)
            }
        }
        return None;
    } else {
        return None;
    }
}
