use crate::VERSION;
use libadwaita::{AboutDialog, ApplicationWindow as AdwApplicationWindow, prelude::*};

//  Standard libadwaita "About" window
pub fn show_about(window: &AdwApplicationWindow) {
    let dialog = AboutDialog::builder()
        .application_name("Shelfies")
        .version(VERSION)
        .developer_name("Chronengi")
        .comments("A simple catalog to keep track of your books.")
        .website("https://gitlab.com/Chronengi/shelfies")
        .license_type(gtk4::License::Gpl30)
        .build();

    dialog.present(Some(window));
}