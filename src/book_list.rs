use crate::Book;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, CustomFilter, DropDown, FilterChange, FilterListModel, Label, ListItem,
    ListView, NoSelection, Orientation, ScrolledWindow, SignalListItemFactory, StringObject, gio,
    glib,
};
use std::{cell::RefCell, rc::Rc};

//  Store creation, one item per book
pub fn new_store(books: Vec<Book>) -> gio::ListStore {
    let store = gio::ListStore::new::<glib::BoxedAnyObject>();
    for book in books {
        store.append(&glib::BoxedAnyObject::new(book));
    }
    store
}

pub fn add_book(store: &gio::ListStore, book: Book) {
    store.append(&glib::BoxedAnyObject::new(book));
}

//  Replace a book with its edited version and refresh its row
pub fn update_book(store: &gio::ListStore, obj: &glib::BoxedAnyObject, book: Book) {
    //  Position in the store (not in the filtered list, which can differ)
    let position = (0..store.n_items()).find(|&i| {
        store
            .item(i)
            .is_some_and(|o| &o == obj.upcast_ref::<glib::Object>())
    });

    match position {
        //  A brand new object in the same place: the list redraws that row
        Some(position) => store.splice(position, 1, &[glib::BoxedAnyObject::new(book)]),
        None => println!("update_book: book not found in the store"),
    }
}

//  Remove a book from the store (its row disappears)
pub fn delete_book(store: &gio::ListStore, obj: &glib::BoxedAnyObject) {
    let position = (0..store.n_items()).find(|&i| {
        store
            .item(i)
            .is_some_and(|o| &o == obj.upcast_ref::<glib::Object>())
    });
    if let Some(position) = position {
        store.remove(position);
    }
}

//  Used when saving: store -> Vec<Book>
pub fn collect_books(store: &gio::ListStore) -> Vec<Book> {
    (0..store.n_items())
        .filter_map(|i| store.item(i))
        .filter_map(|obj| obj.downcast::<glib::BoxedAnyObject>().ok())
        .map(|obj| obj.borrow::<Book>().clone())
        .collect()
}

//  The main widget. on_edit is called with the book of the row whose pencil was clicked.
pub fn book_list(
    store: &gio::ListStore,
    filter_dropdown: &DropDown,
    on_edit: impl Fn(glib::BoxedAnyObject) + 'static,
) -> ScrolledWindow {
    let on_edit = Rc::new(on_edit);
    let factory = SignalListItemFactory::new();

    //  Built once per row widget
    factory.connect_setup(move |_, item| {
        let item = item.downcast_ref::<ListItem>().unwrap();

        let row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .height_request(100)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)
            .build();
        row.add_css_class("card");

        //  Left column: title + custom text
        let text_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .valign(gtk4::Align::Center)
            .vexpand(true)
            .hexpand(true)
            .margin_start(12)
            .margin_end(12)
            .build();

        let title = Label::builder()
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();
        title.add_css_class("title-3");
        text_box.append(&title);

        let custom_text = Label::builder()
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();
        custom_text.add_css_class("dim-label");
        text_box.append(&custom_text);

        row.append(&text_box);

        //  Right column: status + stars
        let info_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .valign(gtk4::Align::Center)
            .halign(gtk4::Align::End)
            .build();

        let status = Label::builder().halign(gtk4::Align::End).build();
        info_box.append(&status);

        let stars = Label::builder().halign(gtk4::Align::End).build();
        info_box.append(&stars);

        row.append(&info_box);

        //  Edit button
        let edit = Button::builder()
            .icon_name("document-edit-symbolic")
            .tooltip_text("Edit this book.")
            .valign(gtk4::Align::Center)
            .margin_start(12)
            .margin_end(12)
            .build();
        edit.add_css_class("flat");

        //  The row is reused for different books: ask the item which one it shows right now
        //  (weak reference, otherwise row and item keep each other alive)
        let weak_item = item.downgrade();
        let on_edit = on_edit.clone();
        edit.connect_clicked(move |_| {
            let Some(list_item) = weak_item.upgrade() else {
                return;
            };
            if let Some(obj) = list_item.item().and_downcast::<glib::BoxedAnyObject>() {
                on_edit(obj);
            }
        });

        row.append(&edit);

        item.set_child(Some(&row));
    });

    //  Runs every time a row is associated with a book (also when reused)
    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<ListItem>().unwrap();
        let obj = item.item().and_downcast::<glib::BoxedAnyObject>().unwrap();
        let book = obj.borrow::<Book>();

        let row = item.child().and_downcast::<GtkBox>().unwrap();
        let text_box = row.first_child().and_downcast::<GtkBox>().unwrap();
        let info_box = text_box.next_sibling().and_downcast::<GtkBox>().unwrap();

        let title = text_box.first_child().and_downcast::<Label>().unwrap();
        let custom_text = title.next_sibling().and_downcast::<Label>().unwrap();
        let status = info_box.first_child().and_downcast::<Label>().unwrap();
        let stars = status.next_sibling().and_downcast::<Label>().unwrap();

        title.set_label(&book.name);
        custom_text.set_label(&book.custom_text);
        custom_text.set_visible(!book.custom_text.is_empty());
        status.set_label(&book.status);

        let filled = (book.stars as usize).min(3);
        stars.set_label(&format!("{}{}", "★".repeat(filled), "☆".repeat(3 - filled)));
    });

    //  Filter by status, driven by the dropdown
    let current_status = Rc::new(RefCell::new(String::from("All")));

    let status_for_filter = current_status.clone();
    let status_filter = CustomFilter::new(move |obj| {
        let wanted = status_for_filter.borrow();
        if *wanted == "All" {
            return true;
        }

        let obj = obj.downcast_ref::<glib::BoxedAnyObject>().unwrap();
        obj.borrow::<Book>().status == *wanted
    });

    let filtered = FilterListModel::new(Some(store.clone()), Some(status_filter.clone()));

    filter_dropdown.connect_selected_notify(move |f| {
        let selected = f
            .selected_item()
            .and_downcast::<StringObject>()
            .unwrap()
            .string()
            .to_string();

        *current_status.borrow_mut() = selected;
        status_filter.changed(FilterChange::Different);
    });

    let selection = NoSelection::new(Some(filtered));
    let list = ListView::new(Some(selection), Some(factory));

    ScrolledWindow::builder().vexpand(true).child(&list).build()
}