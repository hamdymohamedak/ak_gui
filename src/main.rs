use ak_macros::*;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Label};
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
        .label("Go To My Portfolio 🙃")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    porfolio_btn.connect_clicked(|porfolio_btn| open_Web!("https://askander.vercel.app"));

    let getOS = Button::builder()
        .label("OS")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    getOS.connect_clicked(|getOS| {
        let OS_Value = this_OS!();
        let label_text = format!("{}", OS_Value);
        getOS.set_label(&label_text);
    });

    let getIP = Button::builder()
        .label("Get ID")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    getIP.connect_clicked(|getIP| {
        let operatingSystem = this_OS!();

        if operatingSystem == "linux" {
            let terminalValue = terminal!("sh", "ip a");
            getIP.set_label(&terminalValue)
        } else if operatingSystem == "windows" {
            let terminalValue = terminal!("cmd", "ipconfig");
            getIP.set_label(&terminalValue)
        }
    });

    let App_title = Label::new(Some("Hello Dev..!"));

    // Present window
    let element_parent = Box::new(gtk::Orientation::Vertical, 0);
    element_parent.append(&App_title);
    element_parent.append(&GetCPUName);
    element_parent.append(&Ram);
    element_parent.append(&getOS);
    element_parent.append(&getIP);
    element_parent.append(&porfolio_btn);
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("AK-GUI")
        .child(&element_parent)
        .build();

    // Present window
    window.present();
}
