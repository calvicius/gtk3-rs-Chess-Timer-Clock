pub mod program;
use program::app::App;

fn main() {
    if gtk::init().is_err() {
        eprintln!("No se ha podido iniciar la aplicacion GTK");
        std::process::exit(1);
    }
    
    let _app = App::new();
    gtk::main();
}