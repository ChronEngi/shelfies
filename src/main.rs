use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Image, Label, Orientation};
use libadwaita::prelude::*;
use libadwaita::{Application as AdwApplication, ApplicationWindow as AdwApplicationWindow, ToolbarView, HeaderBar};

const APP_ID: &str = "com.test.shelfies";

fn main()
{
    println!("Initialization.");
    //  Creating the application
    let app = AdwApplication::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app:&AdwApplication)
{
    println!("Building main ui.");

    //  Main window
    let window = AdwApplicationWindow::builder()
        .application(app)
        .title("Shelfies")
        .default_width(400)
        .default_height(300)
        .build();

    //  Toolbar view
    let toolbar_view = ToolbarView::new();

    //  Header bar
    let header = HeaderBar::new();
    toolbar_view.add_top_bar(&header);

    //  Outside box
    let outer_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    //  Icon
    let icon = Image::from_icon_name("book-symbolic");
    icon.set_pixel_size(128);
    outer_box.append(&icon);

    //  Label
    let label = Label::new(Some("What are you gonna do with your library today?"));
    outer_box.append(&label);

    //  Buttons box
    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    outer_box.append(&button_box);

    //  Open button
    let open_button = Button::builder().label("Open").build();
    open_button.add_css_class("suggested-action");
    button_box.append(&open_button);

    //  New button
    let new_button = Button::builder().label("Create new").build();
    button_box.append(&new_button);

    //  Render
    toolbar_view.set_content(Some(&outer_box));
    window.set_content(Some(&toolbar_view));
    window.present();
}