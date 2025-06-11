use eframe::{egui, epi};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Contact {
    name: String,
    phone_number: String,
}
#[derive(Default)]
struct PhoneBookApp {
    contacts: Vec<Contact>,
    name_input: String,
    phone_input: String,
    search_query: String,
    path: String,
}

impl PhoneBookApp {
    fn load_contacts(&mut self) {
        if let Ok(mut file) = File::open(&self.path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                self.contacts = serde_json::from_str(&contents).unwrap_or_default();
            }
        }
    }

    fn save_contacts(&self) {
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&self.path) {
            if let Ok(json) = serde_json::to_string_pretty(&self.contacts) {
                let _ = file.write_all(json.as_bytes());
            }
        }
    }
}

impl epi::App for PhoneBookApp {
    fn name(&self) -> &str {
        "Phonebook App"
    }

    fn setup(&mut self, _: &eframe::CreationContext<'_>) {
        self.path = "contacts.json".into();
        self.load_contacts();
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📒 Phonebook");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name_input);
                ui.label("Phone:");
                ui.text_edit_singleline(&mut self.phone_input);
                if ui.button("➕ Add").clicked() {
                    if !self.name_input.trim().is_empty() && !self.phone_input.trim().is_empty() {
                        self.contacts.push(Contact {
                            name: self.name_input.clone(),
                            phone_number: self.phone_input.clone(),
                        });
                        self.name_input.clear();
                        self.phone_input.clear();
                        self.save_contacts();
                    }
                }
            });

            ui.separator();
            ui.label("🔍 Search:");
            ui.text_edit_singleline(&mut self.search_query);

            for contact in self.contacts.iter().filter(|c| c.name.contains(&self.search_query)) {
                ui.horizontal(|ui| {
                    ui.label(format!("{} -> {}", contact.name, contact.phone_number));
                });
            }
        });
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Phonebook App",
        options,
        Box::new(|_cc| Box::new(PhoneBookApp::default())),
    )
}
