use gtk::{self, prelude::*};

use super::clocks;

const CSS: &str = "
button {
    background-image: none;
    box-shadow: none;
    border-radius: 0;
}
";

#[derive(Clone)]
pub struct App {
    pub window: gtk::Window,
}

impl App {
    // el constructor de la Aplicacion
    pub fn new() -> Self {
        
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSS.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let main_window = gtk::Window::new(gtk::WindowType::Toplevel);
        main_window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        main_window.set_border_width(5);
        main_window.set_position(gtk::WindowPosition::Center);
        main_window.set_default_size(300, 200);

        let mut chess_clock = clocks::Timer::new(5_i32);    // 5 minutes
        chess_clock.do_closures();
        
        main_window.add(&chess_clock.hbox_clocks);
        main_window.show_all();

		let app = App {
            window: main_window,
        };
        app
		
	}
}