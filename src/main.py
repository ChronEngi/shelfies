import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Gtk, Adw

def on_activate(app):
    root = Adw.ApplicationWindow(application=app)
    root.set_title("Shelfies")
    root.set_default_size(400, 300)

    # main toolbar view
    toolbar_view = Adw.ToolbarView()

    # Header of the toolbar
    header = Adw.HeaderBar()
    toolbar_view.add_top_bar(header)

    # Core of the toolbar

    # General box
    outer_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=12)
    outer_box.set_margin_top(20)
    outer_box.set_margin_bottom(20)
    outer_box.set_margin_start(20)
    outer_box.set_margin_end(20)

    # Icon
    icon = Gtk.Image.new_from_icon_name("book-symbolic")
    icon.set_pixel_size(128)
    outer_box.append(icon)

    # Text
    label = Gtk.Label(label="What are you gonna do with your library today?")
    outer_box.append(label)

    # Selection box
    box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12,)
    box.set_halign(Gtk.Align.CENTER)
    box.set_margin_top(20)
    box.set_margin_bottom(20)
    box.set_margin_start(20)
    box.set_margin_end(20)
    outer_box.append(box)

    # Create button
    open_button = Gtk.Button(label="Open")
    create_button = Gtk.Button(label="Create new")
    open_button.add_css_class("suggested-action")
    box.append(open_button)
    box.append(create_button)

    toolbar_view.set_content(outer_box)

    # Show
    root.set_content(toolbar_view)
    root.present()

app = Adw.Application(application_id="com.test.shelfies")
app.connect('activate', on_activate)
app.run(None)

print("Hello")