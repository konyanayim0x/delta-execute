use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};
use std::process::Command;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Delta Executor");
    window.set_default_size(400, 200);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&layout);

    let script_entry = Entry::new();
    layout.pack_start(&script_entry, false, false, 0);

    let execute_button = Button::with_label("Execute Script");
    layout.pack_start(&execute_button, false, false, 0);

    let output_label = Label::new("");
    layout.pack_start(&output_label, false, false, 0);

    execute_button.connect_clicked(move |_| {
        let script = script_entry.get_text().unwrap();
        let output = execute_script(&script);
        output_label.set_text(&output);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn execute_script(script: &str) -> String {
    let output = Command::new("roblox_exe_path")
        .arg("-execute")
        .arg(script)
        .output()
        .expect("Failed to execute script");

    String::from_utf8_lossy(&output.stdout).to_string()
}