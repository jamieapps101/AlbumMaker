use super::html_builder::*;
use super::util::*;
use std::iter::FromIterator;
use std::{
    fs::File,
    path::PathBuf,
    io::BufWriter,
};


pub fn create_html_index(new_file: &PathBuf, ar: &ActionRecord) {
    let mut dom = HtmlDom::new();
        dom.add_element(
            HtmlElement::new(HtmlElementType::Head)
                .add_element(HtmlElement::new(HtmlElementType::Link)
                    .set_rel("stylesheet")
                    .set_href("styles.css"))
                .add_element(HtmlElement::new(HtmlElementType::Script)
                    .set_src("main.js"))
                .add_element(HtmlElement::new(HtmlElementType::Title)
                    .set_text("Photo Album")));



    let mut body = HtmlElement::new(HtmlElementType::Body);
    // build up the top of the body
    body = body.add_element(HtmlElement::new(HtmlElementType::P)
                    .set_text("Sub Dirs")
                );
    // --- Sub Folders ---
    // for each folder, format the dir template and insert it
    let mut list = HtmlElement::new(HtmlElementType::Ul);
    for (index,action_record) in ar.get_subdirs().iter().enumerate() {
        println!("folder: {}",index);
        list = list.add_element(format_dir_template(action_record));
    }
    body = body.add_element(list);
    body = body.add_element(HtmlElement::new(HtmlElementType::Br));


    // --- Images ---
    body = body.add_element(HtmlElement::new(HtmlElementType::P)
        .set_text("Images")
    );
    // for each image, format the image template and insert it
    let mut list = HtmlElement::new(HtmlElementType::Ul);
    for photo_action in ar.get_photos() {
        list = list.add_element(format_image_template(photo_action));
    }
    body = body.add_element(list);

    // build up bottom of body
    body = body.add_element(HtmlElement::new(HtmlElementType::Br));

    // render to file
    // setup writer
    let mut writer = BufWriter::new(File::create(new_file).unwrap());
    dom.add_element(body);
    dom.render(&mut writer);
}

fn format_image_template(pa: &PhotoAction) -> HtmlElement {
    let downsizes_image_location = pa.get_downsized();
    let image_name = downsizes_image_location.file_name().unwrap().to_str().unwrap();
    let he = HtmlElement::new(HtmlElementType::Li)
        .add_class("images_item")
        .add_element(HtmlElement::new(HtmlElementType::A)
            .add_class("images_link")
            .set_href(pa.get_actual().to_str().unwrap())
            .add_element(HtmlElement::new(HtmlElementType::Img)
                .set_src(pa.get_downsized().to_str().unwrap())
                .set_alt(image_name)));
    return he;
}

fn format_dir_template(ar: &ActionRecord) -> HtmlElement {

    let photos = ar.get_photos();
    if photos.len() > 0 {
        let cover_photo_containing_path = photos[0].get_dir();
        let downsized_path = photos[0].get_downsized();
        let abs_downsized_path = cover_photo_containing_path.join(downsized_path);
        let cover_photo_path = abs_downsized_path.to_str().unwrap();

        let mut sub_folder_index_path = ar.get_path();
        sub_folder_index_path.push("index.html");

        if !sub_folder_index_path.is_absolute() {
            let iterator  = sub_folder_index_path.components().skip(2);
            let mut temp = PathBuf::from(".");
            temp.push(PathBuf::from_iter(iterator));
            sub_folder_index_path = temp;
        }


        let he = HtmlElement::new(HtmlElementType::Li)
        .add_class("images_item")
        .add_element(HtmlElement::new(HtmlElementType::A)
            .add_class("images_link")
            .set_href(sub_folder_index_path.to_str().unwrap())
            .add_element(HtmlElement::new(HtmlElementType::Img)
                .set_src(cover_photo_path)
                .set_alt(ar.get_path().file_name().unwrap().to_str().unwrap())));
        return he;
    }


    return HtmlElement::new(HtmlElementType::Li)
        .add_element(HtmlElement::new(HtmlElementType::P)
            .set_text("this is unimplemented"));
}

