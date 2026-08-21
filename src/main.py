import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Gtk, Adw

def on_activate(app):
    root = Adw.ApplicationWindow(application=app)
    root.set_title("Shelfies")
    root.set_default_size(400, 300)

    label = Gtk.Label(label="Hello dude")
    root.set_content(label)
    root.present()

app = Adw.Application(application_id="com.test.shelfies")
app.connect('activate', on_activate)
app.run(None)

print("Hello")