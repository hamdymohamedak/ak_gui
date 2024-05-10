use ak_macros::*;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button};
const APP_ID: &str = "org.gtk_rs.hamdymohamedak";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let GetCPUName = Button::builder()
        .label("Get Your CPU Name !")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    GetCPUName.connect_clicked(|GetCPUName| {
        let cpu = Get_CPU!();
        let label_text = format!("{}", cpu);
        GetCPUName.set_label(&label_text);
    });
    let Ram = Button::builder()
        .label("Get RAM Info")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    Ram.connect_clicked(|Ram| {
        let ramSize = Ram_size!();
        let label_text = format!("{}", ramSize);
        Ram.set_label(&label_text);
    });

    let porfolio_btn = Button::builder()
        .label("Go To My Portfolio")
        .margin_top(12)
        .build();

    porfolio_btn.connect_clicked(|porfolio_btn| open_Web!("https://askander.vercel.app"));

    // Create a box to hold the buttons
    let button_box = Box::new(gtk::Orientation::Vertical, 0);
    button_box.append(&GetCPUName);
    button_box.append(&Ram);
    button_box.append(&porfolio_btn);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hamdy Mohamed Askander")
        .child(&button_box)
        .build();

    // Present window
    window.present();
}
