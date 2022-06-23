
mod spin;
mod letter_rect;
use spin::Spinner;
use letter_rect::Alphabet;
use egui::Direction;

fn main() {
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: true,
        // To have rounded corners we need transparency:
    
        transparent: true,
        min_window_size: Some(egui::vec2(700.0, 350.0)),
        ..Default::default()
    };
    eframe::run_native(
        "GCiphers", // unused title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    to_encrypt : String,
    encrypted : String,
    cipher : String,
}
impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let Self {cipher,to_encrypt,encrypted,..} = self;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.menu_button("Ciphers", |ui| {
                        if ui.button("Caesar").clicked() {
                            *cipher = String::from("Caesar");
                            ui.close_menu();
                        }
                        if ui.button("Vigenere").clicked() {
                            *cipher = String::from("Vigenere");
                            ui.close_menu();
                        }
                    });   
                });
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                });
               
            });
         
        });

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.add(Spinner::new().size(200.0));
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(
                egui::Direction::LeftToRight,
                egui::Align::Center), |ui| {
                ui.add(Alphabet::new());
                // ui.add(LColumn::new());
                // ui.add(LColumn::new());
                
            });
        });

       
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
     

   
    }
}

     // The top panel is often a good place for a menu bar:
            // if ui.button(RichText::new("delete").color(Color32::RED)).clicked() {
            //     // …
            // }
            // ui.add(egui::Button::new("sss").fill(Color32::RED));