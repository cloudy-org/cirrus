use egui::{ImageSource, Ui};

pub struct About<'a> {
    image: ImageSource<'a>,
    info: AboutApplicationInfo,
}

pub struct AboutApplicationInfo {
    name: String,
    description: String,
    license: String,
    version: String,
    authors: Vec<AboutAuthorInfo>,
    webpage: String,
    git_repo: String,
}

pub struct AboutAuthorInfo {
    name: String,
    email: Option<String>,
    webpage: Option<String>,
}

impl <'a>About<'a> {
    pub fn new(image: ImageSource<'a>, info: AboutApplicationInfo) -> Self {
        Self {
            image, info,
        }
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.image(self.image.clone());
        todo!();
    }
}