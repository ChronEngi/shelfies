pub fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "

        /* Fixes the vertical alignment of the text */
        .add-book-btn label 
        {
            margin-top: 2px;
        }

        button.my-save
        {
            background-color: #35a047;
            color: #ffffff;
            border-radius: 12px;
            font-size: 14px;
            -gtk-icon-size: 24px;
        }
        button.my-save:hover
        {
            background-color: #3fb953;
        }
        button.my-save:active
        {
            background-color: #2b8539;
        }

        button.my-generic
        {
            background-color: #4a4a4b;
            color: #ffffff;
            border-radius: 12px;
            font-size: 14px;
            -gtk-icon-size: 24px;
        }
        button.my-generic:hover
        {
            background-color: #7a7a7d;
        }
        button.my-generic:active
        {
            background-color: #2d2d2e;
        }

        dropdown.suggested-action > button
        {
            background-color: @accent_bg_color;
            color: @accent_fg_color;
        }
    ",
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
