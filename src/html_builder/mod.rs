use std::{
    fmt::{Debug},
    iter::FromIterator,
    io::Write,
};


pub struct HtmlDom {
    elements: Vec<HtmlElement>,
}

impl HtmlDom {
    pub fn new() -> Self {
        HtmlDom {
            elements: Vec::new(),
        }
    }
    pub fn render<W:Write>(self, f: &mut W) {
        writeln!(f, "<!DOCTYPE html>").unwrap();
        for element in self.elements {
            element.render(0, f);
        }
    }
    pub fn add_element(&mut self, e: HtmlElement) {
        self.elements.push(e);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HtmlElementType {
    P, A, Ul, Li, Head, Body, Br,
    Link, Img, Script,Title,
    Div, 
}

#[derive(Debug)]
pub struct CssClass {
    pub name: String,
}

#[derive(Debug)]
pub struct HtmlElement {
    element_type: HtmlElementType,
    class:        Vec<String>,
    sub_elements: Vec<HtmlElement>,
    // id:           Option<String>,
    src:          Option<String>,
    alt:          Option<String>,
    href:         Option<String>,
    text:         Option<String>,
    rel:          Option<String>,
}


impl HtmlElement {
    pub fn new(element_type: HtmlElementType) -> Self {
        HtmlElement {
            element_type,
            class:        Vec::new(),
            sub_elements: Vec::new(),
            // id:           None,
            src:          None,
            alt:          None,
            href:         None,
            text:         None,
            rel:          None,
        }
    }
    pub fn set_src(mut self, src: &str) -> Self {
        self.src = Some(src.to_owned());
        return self;
    }
    pub fn set_alt(mut self, alt: &str) -> Self {
        self.alt = Some(alt.to_owned());
        return self;
    }
    pub fn add_class(mut self, c: &str) -> Self {
        self.class.push(c.to_owned());
        return self;
    }
    // pub fn set_id(mut self, id: &str) -> Self {
    //     self.id = Some(String::from(id));
    //     return self;
    // }
    pub fn add_element(mut self, e: HtmlElement) -> Self {
        self.sub_elements.push(e);
        return self;
    }
    pub fn set_rel(mut self, rel: &str) -> Self {
        self.rel = Some(rel.to_owned());
        return self;
    }
    pub fn set_href(mut self, href: &str) -> Self {
        self.href = Some(href.to_owned());
        return self;
    }
    pub fn set_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        return self;
    }
    pub fn render<W:Write>(self,indent: usize, f: &mut W) {
        match self.element_type {
            HtmlElementType::P => {
                writeln!(f,"{}<p class={}>{}</p>", 
                    space_pad(indent), 
                    to_class_string(self.class),
                    to_content_string(self.text)).unwrap();
            },
            HtmlElementType::Br => {
                writeln!(f,"{}<br>", 
                    space_pad(indent)).unwrap();
            }, 
            HtmlElementType::Ul => {
                write!(f, "{}<ul class={}>\n", 
                    space_pad(indent), 
                    to_class_string(self.class)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</ul>", space_pad(indent)).unwrap();
            }, 
            HtmlElementType::Li => {
                writeln!(f,"{}<li class={}>", 
                    space_pad(indent), 
                    to_class_string(self.class)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</li>", space_pad(indent)).unwrap();
            },
            HtmlElementType::Div => {
                writeln!(f,"{}<div class={}>", 
                    space_pad(indent), 
                    to_class_string(self.class)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</div>", space_pad(indent)).unwrap();
            }, 
            HtmlElementType::A => {
                writeln!(f,"{}<a class={} href={}>", 
                    space_pad(indent), 
                    to_class_string(self.class),
                    to_property_string(self.href)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</a>", space_pad(indent)).unwrap();
            }, 
            HtmlElementType::Img => {
                writeln!(f,"{}<img class={} src={} alt={}>", 
                    space_pad(indent), 
                    to_class_string(self.class),
                    to_property_string(self.src),
                    to_property_string(self.alt)).unwrap();
            },
            HtmlElementType::Head => {
                writeln!(f,"{}<head class={}>", 
                    space_pad(indent), 
                    to_class_string(self.class)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</head>", space_pad(indent)).unwrap();
            }, 
            HtmlElementType::Body => {
                writeln!(f,"{}<body class={}>", 
                    space_pad(indent), 
                    to_class_string(self.class)).unwrap();
                for element in self.sub_elements {
                    element.render(indent+1, f);
                }
                writeln!(f,"{}</body>", space_pad(indent)).unwrap();
            }, 

            HtmlElementType::Link => {
                writeln!(f, "{}<link rel={} href={}>", 
                    space_pad(indent), 
                    to_property_string(self.rel),
                    to_property_string(self.href)).unwrap();
            },

            HtmlElementType::Script => {
                writeln!(f,"{}<script src={}></script>", 
                    space_pad(indent), 
                    to_property_string(self.src)).unwrap();
            },
            HtmlElementType::Title => {
                writeln!(f,"{}<title>{}</title>", 
                    space_pad(indent), 
                    to_content_string(self.text)).unwrap();
            },
        }
    }
}

fn to_property_string(opt_prop: Option<String>) -> String {
    if let Some(prop) = opt_prop {
        return format!("\"{}\"",prop);
    } else {
        return String::from("\"\"");
    }
}

fn to_content_string(opt_prop: Option<String>) -> String {
    if let Some(prop) = opt_prop {
        return format!("{}",prop);
    } else {
        return String::from("");
    }
}

fn space_pad(spaces: usize) -> String {
    String::from_iter((0..spaces).map(|_|"    "))
}

fn to_class_string(classes: Vec<String>) -> String {
    let mut class_string = String::from("\"");
    if classes.len()>0 {
        let final_class_index = classes.len()-1;
        for (index,class_name) in classes.iter().enumerate() {
           class_string += class_name;
           if index == final_class_index {
               class_string.push(' ');
           }
        }
    }
    class_string.push('\"');
    return class_string;
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufWriter};
    use super::*;
    #[test]
    fn build_hello_world() {
        let mut dom = HtmlDom::new();
        dom.add_element(
            HtmlElement::new(HtmlElementType::Head)
                .add_element(HtmlElement::new(HtmlElementType::Link)
                    .set_rel("stylesheet")
                    .set_href("styles.css"))
                .add_element(HtmlElement::new(HtmlElementType::Script)
                    .set_src("main.js"))
                .add_element(HtmlElement::new(HtmlElementType::Title)
                    .set_text("Photo Album"))
        );
        dom.add_element(
            HtmlElement::new(HtmlElementType::Body)
                .add_element(HtmlElement::new(HtmlElementType::Ul)
                    .add_class("dirs_list"))
                .add_element(HtmlElement::new(HtmlElementType::Ul)
                    .set_text("images_list"))
        );
        let mut writer = BufWriter::new(
            File::create("./test_files_out/html_test.html").unwrap());
        dom.render(&mut writer);
    }
}