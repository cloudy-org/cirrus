use egui::{Color32, Context, ImageSource, Margin, OpenUrl, Pos2, Response, Stroke, Ui};

pub struct About<'a> {
    image: ImageSource<'a>,
    info: AboutApplicationInfo,
    show_license: bool,

    pub license_window_response: Option<Response>
}

pub struct AboutApplicationInfo {
    pub name: String,
    pub description: String,
    pub license: String,
    pub version: String,
    pub authors: Vec<AboutAuthorInfo>,
    pub webpage: String,
    pub git_repo: String,
    pub copyright: String,
}

pub struct AboutAuthorInfo {
    pub name: String,
    pub email: Option<String>,
    pub webpage: Option<String>,
}

impl<'a> About<'a> {
    pub fn new(image: ImageSource<'a>, info: AboutApplicationInfo) -> Self {
        Self {
            image,
            info,
            show_license: false,
            license_window_response: None
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        if self.show_license {
            let response = egui::Window::new(
                egui::WidgetText::RichText(
                    egui::RichText::new("ℹ Licence").size(15.0)
                )
            )
                .open(&mut self.show_license)
                .default_pos(Pos2::new(100.0, 100.0))
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.code(self.info.license.clone());
                    });
                });

            self.license_window_response = Some(response.unwrap().response);
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let reusable_frame = egui::Frame::canvas(ui.style())
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::NONE);

        ui.vertical_centered(|ui| {
            ui.add(
                egui::Image::new(self.image.clone())
                    .max_width(120.0)
            );

            ui.heading(
                egui::RichText::new(self.info.name.clone())
                    .size(30.0)
                    .strong()
                    .monospace()
            );

            ui.label(
                egui::RichText::new(format!("v{}", self.info.version))
                    .monospace()
            );

            ui.separator();

            ui.horizontal(|ui| {
                let space = (ui.available_width() - (2.0 * 100.0 + 10.0)) / 2.0;

                ui.add_space(space - 10.0);

                if ui.button("Website").clicked() {
                    ui.ctx().open_url(
                        OpenUrl::new_tab(&self.info.webpage)
                    );
                }

                if ui.button("Source Code").clicked() {
                    ui.ctx().open_url(
                        OpenUrl::new_tab(&self.info.git_repo)
                    );
                }

                // TODO: this button should open a egui window that goes 
                // and grabs all the contributors from the git repo (github only for now)
                // to display their profile pictures like github's "contributors" list on repositories.
                // 
                // Then below that profile picture section have a list of all cargo packages used with 
                // hyperlinks to them.
                // 
                // (will create an issue for this soon tm, do remind me...).
                let _ = ui.button("Credits");
            });

            ui.add_space(10.0);
            ui.heading("Created with ❤ by:");

            reusable_frame.clone()
                .outer_margin(Margin::symmetric(15.0, 5.0))
                .show(ui, |ui| {
                    egui::Grid::new("about_authors_grid")
                        .num_columns(self.info.authors.len())
                        .min_row_height(60.0)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {        
                            for author_info in self.info.authors.iter() {
                                let github = format!("https://github.com/{}", author_info.name);

                                let image = egui::Image::from_uri(format!("{}.png", &github))
                                    .rounding(100.0)
                                    .fit_to_exact_size(
                                        egui::Vec2::new(50.0, 50.0)
                                    );

                                ui.add(image);
                                ui.hyperlink_to(
                                    egui::RichText::new(author_info.name.clone())
                                        .size(18.0),
                                    &github
                                );
                                ui.end_row();
                            }
                        }
                    );
                }
            );

            // TODO: this button will just be
            // an alias to the credits button.
            let _ = ui.button("❤ and you guys!");

            ui.add_space(20.0);

            if ui.button("License").clicked() {
                self.show_license = true;
            }
            ui.label(&self.info.copyright);
        });

    }
}

pub fn cargo_authors_to_about_authors(cargo_authors: &String) -> Vec<AboutAuthorInfo> {
    let mut about_author_infos: Vec<AboutAuthorInfo> = Vec::new();

    for cargo_author in cargo_authors.split(":") {
        let mut owo = cargo_author.split("<");

        let author_name = owo.next()
            .expect("Failed to parse cargo authors, expected '<' after name. Please format the authors correctly!")
            .trim()
            .to_string();

        let author_email = match owo.next() {
            Some(email) => Some(email.trim().to_string()),
            None => None,
        };

        let about_author_info = AboutAuthorInfo {
            name: author_name,
            email: author_email,
            webpage: None,
        };

        about_author_infos.push(about_author_info);
    }

    about_author_infos
}
