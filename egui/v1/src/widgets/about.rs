use egui::{Color32, ImageSource, Margin, OpenUrl, Pos2, Response, Stroke, Ui, Vec2};

use cirrus_authors::Authors;

pub struct About {
    pub license_window_response: Option<Response>
}

pub struct AboutApplicationInfo<'a> {
    pub name: String,
    pub description: String,
    pub license: String,
    pub version: String,
    pub authors: &'a Authors,
    pub webpage: String,
    pub git_repo: String,
    pub copyright: String,
}

impl About {
    pub fn new() -> Self {
        Self {
            license_window_response: None
        }
    }

    pub fn show<'a>(
        &mut self,
        ui: &mut Ui,
        image: ImageSource<'a>,
        info: AboutApplicationInfo<'a>,
        show_license: &mut bool
    ) {
        let reusable_frame = egui::Frame::canvas(ui.style())
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::NONE);

        ui.vertical_centered(|ui| {
            ui.add(
                egui::Image::new(image)
                    .max_width(120.0)
            );

            ui.heading(
                egui::RichText::new(info.name)
                    .size(30.0)
                    .strong()
                    .monospace()
            );

            ui.label(
                egui::RichText::new(
                    format!("v{}", info.version)
                ).monospace()
            );

            ui.separator();

            ui.horizontal(|ui| {
                let space = (ui.available_width() - (2.0 * 100.0 + 10.0)) / 2.0;

                ui.add_space(space - 10.0);

                if ui.button("Website").clicked() {
                    ui.ctx().open_url(
                        OpenUrl::new_tab(info.webpage)
                    );
                }

                if ui.button("Source Code").clicked() {
                    ui.ctx().open_url(
                        OpenUrl::new_tab(info.git_repo)
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
                .outer_margin(Margin::symmetric(15, 5))
                .show(ui, |ui| {
                    egui::Grid::new("about_authors_grid")
                        .min_row_height(60.0)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            let author =  &info.authors.author;

                            // TODO: download and cache author image in cache directory
                            // TODO: after that is done you can remove "http" feature in egui_extras

                            let author_image = match &author.git_tag {
                                Some(git_tag) => egui::Image::from_uri(format!("{}.png", git_tag.get_owner_link())),
                                None => egui::Image::new(
                                    egui::include_image!("../../../../assets/no_author_image.jpg")
                                ),
                            };

                            ui.add(
                                author_image
                                    .corner_radius(100.0)
                                    .fit_to_exact_size(Vec2::new(70.0, 70.0))
                            );

                            let name_label_text = egui::RichText::new(&author.name).size(18.0);

                            match &author.git_tag {
                                Some(git_tag) => {
                                    ui.hyperlink_to(
                                        name_label_text,
                                        git_tag.get_owner_link()
                                    );
                                },
                                None => {
                                    ui.label(name_label_text);
                                },
                            }

                            ui.end_row();
                        }
                    );
                }
            );

            // TODO: this button will just be
            // an alias to the credits button.
            let _ = ui.button("❤ and you guys!");

            ui.add_space(20.0);

            if ui.button("License").clicked() {
                *show_license = true;
            }
            ui.label(info.copyright);
        });

        if *show_license {
            let response = egui::Window::new(
                egui::WidgetText::RichText(
                    egui::RichText::new("ℹ Licence").size(15.0).into()
                )
            )
                .open(show_license)
                .default_pos(Pos2::new(100.0, 100.0))
                .show(ui.ctx(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.code(info.license.clone());
                    });
                });

            self.license_window_response = Some(response.unwrap().response);
        }
    }
}