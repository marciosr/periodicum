extern crate gio;
extern crate gtk;
extern crate pango;
extern crate gdk;

#[macro_use]
mod utils;
mod gui;
mod backend;
mod table;

use crate::table::*;
use crate::gui::*;
use crate::backend::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Builder, Grid, ApplicationWindow};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/resources"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "data/ui"]
struct Ui;

fn on_activate(application: &gtk::Application) {

	//let glade_src = include_str!("window.ui");
	string_from_resource!(glade_src, Ui, "window.ui");
	let builder = Builder::from_string(&glade_src);

	get_widget!(builder, ApplicationWindow, window);
	get_widget!(builder, Grid, grid);

	application.add_window(&window);

	let estrutura = match carrega_dados() {
		Ok(serializado) => desserializa(serializado),
		Err(_e) => desserializa(make_table().to_string()),
	};

	build_ui(&grid, estrutura, &window);
	window.show_all();
}

fn main() {

	let app = gtk::Application::new(Some("com.github.marciosr.periodicum"), Default::default());

	app.connect_activate(|app| {

		string_from_resource!(css, Asset, "style.css");

		let provider = gtk::CssProvider::new();

		provider
			.load_from_data(css.as_bytes())
			.expect("Failed to load CSS.");

		gtk::StyleContext::add_provider_for_screen(
			&gdk::Screen::default().expect("Error initializing gtk css provider."),
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);

		on_activate(app);
	});

	app.run();
}
