    //use gtk4::AccessibleAutocomplete::None;
    use gtk4::prelude::*;
    use gtk4::{/*Application, ApplicationWindow,*/ Box as GtkBox, Button, Image, Label, Orientation};
    use libadwaita::prelude::*;
    use libadwaita::{Application as AdwApplication, ApplicationWindow as AdwApplicationWindow, ToolbarView, HeaderBar};
    use serde::{Serialize, Deserialize};

    mod css;

    #[derive(Serialize, Deserialize)]
    struct Book {
        name: String,
        status: String,
        stars: u8,
    }

    #[derive(Serialize, Deserialize)]
    struct Shelf {
        version: String,
        shlf_logic: String,
        books: Vec<Book>,
    }

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

        css::load_css();

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

        let window_for_open = window.clone();
        open_button.connect_clicked(move |_|
        {
            open_file(&window_for_open, String::new());
        });

        //  New button
        let new_button = Button::builder().label("Create new").build();
        button_box.append(&new_button);

        let window_for_new = window.clone();
        new_button.connect_clicked(move |_|
        {
            save_file(&window_for_new, true, String::new());
        });

        //  Render
        toolbar_view.set_content(Some(&outer_box));
        window.set_content(Some(&toolbar_view));
        window.present();
    }

    fn save_file(window: &AdwApplicationWindow, is_new_file: bool, file_path: String)
    {
        //  If is_new_file = true, creates an empty file, otherwise saves the data.
        //  If path is empty, it will be asked to be provided.

        if file_path.is_empty()
        {
            let window = window.clone();

            gtk4::glib::spawn_future_local(async move
            {
                let dialog = gtk4::FileDialog::builder()
                    .title("Save a shelf")
                    .initial_name("library1.shlf")
                    .build();

                // Waiting for the user choise
                if let Ok(file) = dialog.save_future(Some(&window)).await
                {
                    let path = file.path().unwrap().to_string_lossy().to_string();
                    save_file(&window, is_new_file, path);
                }
            });
            return;
        }

        let data = Shelf {
            version: "0.1".to_string(),
            shlf_logic: "1.0".to_string(),
            books: vec![],
        };
        std::fs::write(&file_path, serde_json::to_string_pretty(&data).unwrap()).unwrap();
        println!("Saved to: {}", file_path);
    }

    fn open_file(window: &AdwApplicationWindow, file_path: String)
    {
        //  If path is empty, it will be asked to be provided.
    if file_path.is_empty()
    {
        let window = window.clone();

        gtk4::glib::spawn_future_local(async move
        {
            let dialog = gtk4::FileDialog::builder()
                .title("Open a shelf")
                .build();

            if let Ok(file) = dialog.open_future(Some(&window)).await
            {
                let path = file.path().unwrap().to_string_lossy().to_string();
                open_file(&window, path);
            }
        });
        return;
    }

    //  Checking JSON
    let content = std::fs::read_to_string(&file_path).unwrap();
    let shelf: Shelf = serde_json::from_str(&content).unwrap();
    println!("Opened {} ({} books)", file_path, shelf.books.len());

    //  Title
    let title = std::path::Path::new(&file_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    //  Working window
    let app = window.application().unwrap();
    let working_window = AdwApplicationWindow::builder()
        .application(&app)
        .title(&title)
        .default_width(1100)
        .default_height(700)
        .build();

    //  Toolbar
    let session_toolbar = ToolbarView::new();
    session_toolbar.add_top_bar(&HeaderBar::new());
    working_window.set_content(Some(&session_toolbar));

    //  Top container
    let content_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    //  Top bar inside of the Top container
    let top_bar = gtk4::Fixed::builder()
        .height_request(80)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();
    top_bar.add_css_class("card");

    // ---- SAVE BUTTON ----
    // Conent
    let save_content = libadwaita::ButtonContent::builder()
    .icon_name("document-save-symbolic")
    .label("Save")
    .build();
    //  Button
    let save = Button::builder()
        .child(&save_content)
        .width_request(100)
        .height_request(50)
        .build();
    save.add_css_class("my-save");

    // ---- RELOAD BUTTON ----
    //  Content
    let reload_content = libadwaita::ButtonContent::builder()
    .icon_name("view-refresh-symbolic")
    .label("Reload")
    .build();
    //  Button
    let reload = Button::builder()
        .child(&reload_content)
        .width_request(100)
        .height_request(50)
        .build();
    reload.add_css_class("my-generic");

    // ---- INFO BUTTON ----
    //  Content
    let about_content = libadwaita::ButtonContent::builder()
    .icon_name("help-about-symbolic")
    .label("About me")
    .build();
    //  Button
    let about = Button::builder()
        .child(&about_content)
        .width_request(100)
        .height_request(50)
        .build();
    about.add_css_class("my-generic");

    top_bar.put(&save, 15.0, 15.0);
    top_bar.put(&reload, 125.0, 15.0);
    top_bar.put(&about, 235.0, 15.0);

    content_box.append(&top_bar);
    session_toolbar.set_content(Some(&content_box));
    working_window.present();

    //  Close selection window
    window.close();
}