pub fn load_css()
{
    let provider = gtk4::CssProvider::new();
    provider.load_from_data("
        button.my-save
        {
            background-color: #35a047;
            color: #ffffff;
            border-radius: 12px;
            font-size: 14px;
            -gtk-icon-size: 24px;
        }

        button.my-generic
        {
            background-color: #4a4a4b;
            color: #ffffff;
            border-radius: 12px;
            font-size: 14px;
            -gtk-icon-size: 24px;
        }
    ");
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}