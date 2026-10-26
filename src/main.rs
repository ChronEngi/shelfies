use gtk4::{Box as GtkBox, Button, DropDown, Label, Orientation};
use libadwaita::{
    Application as AdwApplication, ApplicationWindow as AdwApplicationWindow, HeaderBar,
    StatusPage, ToolbarStyle, ToolbarView,};
use libadwaita::{ButtonContent, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,};

use crate::error::error_fn;

mod book_list;
mod css;
mod about;
mod recents;
mod error;

#[derive(Serialize, Deserialize, Clone)]
struct Shelf {
    info: Info,
    books: Vec<Book>,
}
#[derive(Serialize, Deserialize, Clone)]
struct Book {
    name: String,
    status: String,
    stars: u8,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    custom_text: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct Info {
    version: String,
    shlf_logic: String,
}

const VERSION: &str = "0.4.1";
const SHLF_LOGIC: &str = "2.0";
const APP_ID: &str = "io.github.ChronEngi.shelfies";
const STATUSES: [&str; 7] = [
    "Undefined",
    "To read",
    "Reading",
    "Finished",
    "To buy",
    "Sold",
    "Reserved",
];

fn main() {
    println!("Initialization.");
    //  Creating the application
    let app = AdwApplication::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &AdwApplication) {
    println!("Building main ui.");

    //  Main window
    let window = AdwApplicationWindow::builder()
        .application(app)
        .title("Shelfies")
        .default_width(420)
        .default_height(440)
        .build();

    css::load_css();

    //  Toolbar view: flat header, merged with the content
    let toolbar_view = ToolbarView::new();
    toolbar_view.set_top_bar_style(ToolbarStyle::Flat);

    //  Header bar: the status page already shows the title
    let header = HeaderBar::new();
    header.set_show_title(false);
    toolbar_view.add_top_bar(&header);

    //  About button, top left
    let about = Button::builder()
        .icon_name("help-about-symbolic")
        .tooltip_text("About me?")
        .build();
    about.add_css_class("flat");

    let window_for_about = window.clone();
    about.connect_clicked(move |_| {
        about::show_about(&window_for_about);
    });
    header.pack_start(&about);

    //  Open button
    let open_button = Button::builder().label("Open").build();
    open_button.add_css_class("suggested-action");
    open_button.add_css_class("pill");

    let window_for_open = window.clone();
    open_button.connect_clicked(move |_| {
        open_file(&window_for_open, String::new());
    });

    //  New button
    let new_button = Button::builder().label("Create new").build();
    new_button.add_css_class("pill");

    let window_for_new = window.clone();
    new_button.connect_clicked(move |_| {
        save_file(
            &window_for_new,
            true,
            String::new(),
            Shelf {
                info: Info {
                    version: VERSION.to_string(),
                    shlf_logic: SHLF_LOGIC.to_string(),
                },
                books: vec![],
            },
        );
    });

    //  Buttons box
    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .build();
    button_box.append(&open_button);
    button_box.append(&new_button);

    //  Welcome page: icon + title + description + buttons
    let status_page = StatusPage::builder()
        .icon_name("accessories-dictionary-symbolic")
        .title("Shelfies")
        .description("Open a library or create a new one.")
        .child(&button_box)
        .vexpand(true)
        .build();

    //  Recent files menu
    let recent_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .build();
    let recent_popover = gtk4::Popover::builder().child(&recent_box).build();

    let recent = recents::load();
    if recent.is_empty() {
        let empty = Button::builder().label("Empty").sensitive(false).build();
        empty.add_css_class("flat");
        recent_box.append(&empty);
    }
    for path in recent {
        let name = std::path::Path::new(&path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let item = Button::builder().label(name.as_str()).build();
        item.add_css_class("flat");

        let window_for_recent = window.clone();
        let popover_for_recent = recent_popover.clone();
        item.connect_clicked(move |_| {
            popover_for_recent.popdown();
            open_file(&window_for_recent, path.clone());
        });

        recent_box.add_css_class("pixel-fix-btn");
        recent_box.add_css_class("flat");
        recent_box.append(&item);
    }

    let recent_button = gtk4::MenuButton::builder()
        .label("Recent")
        .popover(&recent_popover)
        .build();
    recent_button.add_css_class("flat");
    header.pack_start(&recent_button);

    //  Render
    toolbar_view.set_content(Some(&status_page));
    window.set_content(Some(&toolbar_view));
    window.present();
}

fn save_file(window: &AdwApplicationWindow, is_new_file: bool, file_path: String, shelf: Shelf) {
    //  If is_new_file = true, creates an empty file, otherwise saves the data.
    //  If path is empty, it will be asked to be provided.
    if file_path.is_empty() {
        let window = window.clone();

        gtk4::glib::spawn_future_local(async move {
            let dialog = gtk4::FileDialog::builder()
                .title("Save a shelf")
                .initial_name("library1.shlf")
                .build();

            // Waiting for the user choise
            if let Ok(file) = dialog.save_future(Some(&window)).await {
                let path = file.path().unwrap().to_string_lossy().to_string();
                save_file(&window, is_new_file, path, shelf);
            }
        });
        return;
    }

    //  Save as an uncompressed archive: info.json + books.json
    use std::io::Write;

    //  Serialize first, so a failure doesn't leave an empty file
    let info_json = serde_json::to_string_pretty(&shelf.info).unwrap();
    let books_json = serde_json::to_string_pretty(&shelf.books).unwrap();
    let file = std::fs::File::create(&file_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("info.json", options).unwrap();
    zip.write_all(info_json.as_bytes()).unwrap();
    zip.start_file("books.json", options).unwrap();
    zip.write_all(books_json.as_bytes()).unwrap();
    zip.finish().unwrap();
    println!("Saved to: {}", file_path);
}

fn open_file(window: &AdwApplicationWindow, file_path: String) {
    //  If path is empty, it will be asked to be provided.
    if file_path.is_empty() {
        let window = window.clone();

        gtk4::glib::spawn_future_local(async move {
            let dialog = gtk4::FileDialog::builder().title("Open a shelf").build();

            if let Ok(file) = dialog.open_future(Some(&window)).await {
                let path = file.path().unwrap().to_string_lossy().to_string();
                    
                open_file(&window, path);
            }
        });
        return;
    }

    //  Checking JSON
    //  Open the archive and read info.json + books.json
    use std::io::Read;
    
    // Try to open the file.
    let file = match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(e)=> {
            error_fn(&window, &format!("{e}"));
            return;
        }
    };

    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut info_content = String::new();
    archive
        .by_name("info.json")
        .unwrap()
        .read_to_string(&mut info_content)
        .unwrap();
    let info: Info = serde_json::from_str(&info_content).unwrap();

    //  Try to find books.json, send and error otherwise.
    let mut books_file = match archive.by_name("books.json") {
        Ok(file) => file,
        Err(e) => {
            error_fn(&window, &format!("{e}. The archive is corrupted!"));
            return;
        }
    };

    //  Try to read books.json's content, send and error otherwise.
    let mut books_content = String::new();
    match books_file.read_to_string(&mut books_content) {
        Ok(_) => {},
        Err(e) => {
            error_fn(&window, &format!("{e}"));
            return;
        }
    }

    let books: Vec<Book> = serde_json::from_str(&books_content).unwrap();

    let mut shelf = Shelf { info, books };

    //  Remember this file as recent
    recents::add(&file_path);

    //  Title
    let short_path = std::path::Path::new(&file_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let title = format!("{} ({} books)", short_path, shelf.books.len());

    //  Books live in the store from now on
    let store = book_list::new_store(std::mem::take(&mut shelf.books));

    //  Recall to save
    let shelf = Rc::new(RefCell::new(shelf));

    //  Working window
    let app = window.application().unwrap();
    let working_window = AdwApplicationWindow::builder()
        .application(&app)
        .title(&title)
        .default_width(800)
        .default_height(700)
        .build();

    //  Title follows the number of books
    let weak_window = working_window.downgrade();
    store.connect_items_changed(move |store, _, _, _| {
        if let Some(window) = weak_window.upgrade() {
            window.set_title(Some(&format!("{} ({} books)", short_path, store.n_items())));
        }
    });

    //  Toolbar
    let session_toolbar = ToolbarView::new();
    let session_header = HeaderBar::new();
    session_toolbar.add_top_bar(&session_header);

    // Save Button
    let save = Button::builder()
        .icon_name("document-save-symbolic")
        .tooltip_text("Save your work.")
        .build();
    save.add_css_class("flat");

    let saved_label = Label::new(Some("Saved! :)"));
    saved_label.add_css_class("dim-label");
    saved_label.set_visible(false);

    let window_for_save = working_window.clone();
    let shelf_for_save = shelf.clone();
    let store_for_save = store.clone();
    let saved_for_click = saved_label.clone();
    let path_for_save = file_path.clone();
    save.connect_clicked(move |btn| {
        let mut shelf_to_save = shelf_for_save.borrow().clone();
        shelf_to_save.books = book_list::collect_books(&store_for_save);
        save_file(&window_for_save, false, path_for_save.clone(), shelf_to_save);

        btn.remove_css_class("suggested-action");
        btn.add_css_class("flat");

        saved_for_click.set_visible(true);
        let label = saved_for_click.clone();
        gtk4::glib::timeout_add_seconds_local_once(5, move || {
            label.set_visible(false);
        });
    });

    //  Ask before closing with unsaved changes (unsaved = save button is highlighted)
    let save_for_close = save.clone();
    working_window.connect_close_request(move |window| {
        if !save_for_close.has_css_class("suggested-action") {
            return gtk4::glib::Propagation::Proceed;
        }

        let window = window.clone();
        let save = save_for_close.clone();

        gtk4::glib::spawn_future_local(async move {
            let dialog = libadwaita::AlertDialog::builder()
                .heading("Unsaved changes")
                .body("Do you want to save before closing?")
                .default_response("save")
                .close_response("cancel")
                .build();
            dialog.add_responses(&[
                ("cancel", "Cancel"),
                ("discard", "Discard"),
                ("save", "Save"),
            ]);
            dialog.set_response_appearance("discard", libadwaita::ResponseAppearance::Destructive);
            dialog.set_response_appearance("save", libadwaita::ResponseAppearance::Suggested);

            match dialog.choose_future(Some(&window)).await.as_str() {
                "save" => {
                    save.emit_clicked();
                    window.close();
                }
                "discard" => window.destroy(),
                _ => {}
            }
        });
        gtk4::glib::Propagation::Stop
    });

    let save_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    save_box.append(&save);
    save_box.append(&saved_label);

    // ---- ADD BOOK (main action) ----
    let add_book_content = ButtonContent::builder()
        .label("Add book")
        .icon_name("list-add-symbolic")
        .tooltip_text("Add a new book to the list.")
        .build();
    let add_book = Button::builder().child(&add_book_content).build();
    add_book.add_css_class("suggested-action");
    add_book.add_css_class("pixel-fix-btn");
    let window_for_add = working_window.clone();
    let store_for_add = store.clone();
    let save_for_add = save.clone();
    add_book.connect_clicked(move |_| {
        book_dialog(
            &window_for_add,
            store_for_add.clone(),
            save_for_add.clone(),
            None,
        );
    });

    // ---- FILTER ----
    let filter = filter_button();

    // ---- ABOUT ----
    let about = Button::builder()
        .icon_name("help-about-symbolic")
        .tooltip_text("About me?")
        .build();
    about.add_css_class("flat");

    let window_for_about = working_window.clone();
    about.connect_clicked(move |_| {
        about::show_about(&window_for_about);
    });

    //  Actions on the left
    session_header.pack_start(&add_book);
    session_header.pack_start(&save_box);

    //  Actions on the right
    session_header.pack_end(&about);
    session_header.pack_end(&filter);

    //  Book list: the pencil of a row opens the same dialog, with that book's values
    let window_for_edit = working_window.clone();
    let store_for_edit = store.clone();
    let save_for_edit = save.clone();
    let content = book_list::book_list(&store, &filter, move |obj| {
        book_dialog(
            &window_for_edit,
            store_for_edit.clone(),
            save_for_edit.clone(),
            Some(obj),
        );
    });

    session_toolbar.set_content(Some(&content));
    working_window.set_content(Some(&session_toolbar));
    working_window.present();

    //  Close selection window
    window.close();
}

//  editing = None: add a new book. editing = Some(book): edit that book.
fn book_dialog(
    window: &AdwApplicationWindow,
    store: gtk4::gio::ListStore,
    save_btn: Button,
    editing: Option<gtk4::glib::BoxedAnyObject>,
) {
    let dialog = libadwaita::Dialog::builder()
        .title("Edit Book")
        .content_width(420)
        .build();

    //  Fields
    let title_row = libadwaita::EntryRow::builder().title("Title").build();

    let status_row = libadwaita::ComboRow::builder()
        .title("Status")
        .model(&gtk4::StringList::new(&STATUSES))
        .build();

    let custom_row = libadwaita::EntryRow::builder().title("Custom text").build();

    //  Editing: start from the current values of the book
    let editing_book = editing.as_ref().map(|obj| obj.borrow::<Book>().clone());
    if let Some(book) = &editing_book {
        title_row.set_text(&book.name);
        custom_row.set_text(&book.custom_text);
        let index = STATUSES.iter().position(|s| *s == book.status).unwrap_or(0);
        status_row.set_selected(index as u32);
    }

    let list = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .build();
    list.add_css_class("boxed-list");
    list.append(&title_row);
    list.append(&status_row);
    list.append(&custom_row);

    //  Stars
    let initial_stars = editing_book.as_ref().map(|b| b.stars).unwrap_or(1);
    let stars = Rc::new(Cell::new(initial_stars));
    let stars_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(gtk4::Align::Center)
        .build();

    let star_buttons: Vec<Button> = (0..3)
        .map(|i| {
            let icon = if i < initial_stars {
                "starred-symbolic"
            } else {
                "non-starred-symbolic"
            };
            let star = Button::builder().icon_name(icon).build();
            star.add_css_class("flat");
            stars_box.append(&star);
            star
        })
        .collect();

    for (i, star) in star_buttons.iter().enumerate() {
        let stars = stars.clone();
        let all = star_buttons.clone();
        star.connect_clicked(move |_| {
            stars.set(i as u8 + 1);
            for (j, s) in all.iter().enumerate() {
                s.set_icon_name(if j <= i {
                    "starred-symbolic"
                } else {
                    "non-starred-symbolic"
                });
            }
        });
    }

    //  Cancel / Save
    let cancel = Button::builder().label("Cancel").build();
    let dialog_for_cancel = dialog.clone();
    cancel.connect_clicked(move |_| {
        dialog_for_cancel.close();
    });

    //  Delete (only when editing a book)
    let delete_button = editing.clone().map(|obj| {
        let delete = Button::builder().label("Delete").build();
        delete.add_css_class("destructive-action");

        let store_for_delete = store.clone();
        let save_for_delete = save_btn.clone();
        let dialog_for_delete = dialog.clone();
        delete.connect_clicked(move |_| {
            book_list::delete_book(&store_for_delete, &obj);

            save_for_delete.remove_css_class("flat");
            save_for_delete.add_css_class("suggested-action");

            dialog_for_delete.close();
        });
        delete
    });

    let save = Button::builder().label("Save").build();
    save.add_css_class("suggested-action");

    let dialog_for_save = dialog.clone();
    save.connect_clicked(move |_| {
        let title = title_row.text().to_string();
        if title.is_empty() {
            return;
        }

        let book = Book {
            name: title,
            status: STATUSES[status_row.selected() as usize].to_string(),
            stars: stars.get(),
            custom_text: custom_row.text().to_string(),
        };
        match &editing {
            Some(obj) => book_list::update_book(&store, obj, book),
            None => book_list::add_book(&store, book),
        }

        save_btn.remove_css_class("flat");
        save_btn.add_css_class("suggested-action");

        println!("Books: {}", store.n_items());
        dialog_for_save.close();
    });

    let buttons_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::End)
        .build();

    if let Some(delete) = &delete_button {
        buttons_box.append(delete);
    }
    buttons_box.append(&cancel);
    buttons_box.append(&save);

    //  Layout
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(18)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();
    content.append(&list);
    content.append(&stars_box);
    content.append(&buttons_box);

    let toolbar = ToolbarView::new();
    toolbar.add_top_bar(&HeaderBar::new());
    toolbar.set_content(Some(&content));

    dialog.set_child(Some(&toolbar));
    dialog.present(Some(window));
}

fn filter_button() -> DropDown {
    let filter = DropDown::from_strings(&[
        "All",
        "Undefined",
        "To read",
        "Reading",
        "Finished",
        "To buy",
        "Sold",
        "Reserved"
    ]);
    filter.set_tooltip_text(Some("Filter the book list."));
    filter.add_css_class("pixel-fix-btn");

    filter.connect_selected_notify(|f| {
        if f.selected() == 0 {
            f.remove_css_class("suggested-action");
            f.add_css_class("flat");
        } else {
            f.remove_css_class("flat");
            f.add_css_class("suggested-action");
        }
    });

    filter
}
