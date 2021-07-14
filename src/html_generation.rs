use super::html_builder::*;
use super::util::*;

use std::{
    fs::File,
    path::PathBuf,
    io::BufWriter,
};


pub fn create_html_index(new_file: &PathBuf, ar: ActionRecord) {
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
    for action_record in ar.get_subdirs() {
        list = list.add_element(HtmlElement::new(HtmlElementType::Li)
                    .add_element(format_dir_template(action_record))
                );
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
        list = list.add_element(HtmlElement::new(HtmlElementType::Li)
                    .add_element(format_image_template(photo_action))
                );
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
    HtmlElement::new(HtmlElementType::P)
        .set_text("this is unimplemented")
}

