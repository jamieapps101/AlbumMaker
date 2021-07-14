use std::{
    fmt::{Debug,Formatter},
    iter::FromIterator,
};

pub trait HtmlElement<E:HtmlElement<E>> : Debug {
    fn new() -> Self;
    fn add_class(self, c: CssClass) -> Self;
    fn set_id(self, id: &str) -> Self;
    fn add_element(self, e: E) -> Self;
    fn render(self,indent: usize, f: &mut Formatter<'_>);
}

#[derive(Debug)]
pub struct CssClass {
    pub name: String,
}

pub fn space_pad(spaces: usize) -> String {
    String::from_iter((0..spaces).map(|_|" "))
}

pub fn to_class_string(classes: Vec<CssClass>) -> String {
 let mut class_string = String::from("\"");
 let final_class_index = classes.len()-1;
 for (index,class_name) in classes.iter().enumerate() {
    class_string.push_str(&class_name.name);
    if index == final_class_index {
        class_string.push(' ');
    }
 }
 class_string.push('\"');
 return class_string;
}