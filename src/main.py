import gi
import json

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Gtk, Adw

app_version = "0.1"
shlf_logic = "0.1"

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
    create_button.connect("clicked", on_create)
    open_button.add_css_class("suggested-action")
    box.append(open_button)
    box.append(create_button)

    toolbar_view.set_content(outer_box)

    # Show
    root.set_content(toolbar_view)
    root.present()



def on_create(button):
    print("Starting the creation...")

    # Get back the root window's root
    window = button.get_root()

    # Just creating the dialog, not showing jet
    dialog = Gtk.FileDialog()
    dialog.set_title("Create a new library")
    dialog.set_initial_name("library1.shlf")

    dialog.save(window, None, on_save_done)



def on_save_done(dialog, result):
    print("Alrighttt I got the location, gonna save")

    try:
        file = dialog.save_finish(result)
        path = file.get_path()
        print("Saving here: ", path)

        # Basic data structure
        data = {
            "shlf_logic": shlf_logic,
            "books": []
        }

        # Write out the json
        with open(path, "w") as f:
            json.dump(data, f, indent=2)
    
    except Exception as e:
        print("Hmmmm, error: ", e)

app = Adw.Application(application_id="com.test.shelfies")
app.connect('activate', on_activate)
app.run(None)

print("Bye")