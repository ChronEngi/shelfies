use libadwaita::{prelude::*, AlertDialog, ApplicationWindow};

pub fn error_fn(parent: &ApplicationWindow, message: &str) {
    let dialog = AlertDialog::builder()
        .heading("Error!")
        .body(message)
        .build();

    dialog.present(Some(parent));
}