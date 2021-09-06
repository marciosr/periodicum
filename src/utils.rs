macro_rules! get_widget {
	($builder:expr, $wtype:ty, $name:ident) => {
		let $name: $wtype = $builder.object(stringify!($name))
			.expect(&format!("Could not find widget \"{}\"", stringify!($name)));
	};
}

macro_rules! build_widget {
	($name:ident, $wtype:ident, $vetor:ident, $window:ident) => {

		let $name: gtk::$wtype = gtk::$wtype::new();

		let emto = $vetor.clone();
		let window_clone = $window.clone(); // Compila, mas não cria os botões.

		let lb_sy: gtk::Label = gtk::Label::new(Some(&emto.symbol));
		lb_sy.set_widget_name("symbol");
		let lb_an: gtk::Label = gtk::Label::new(Some(&emto.atomic_number.to_string()));
		lb_an.set_widget_name("atomic_number");
		let lb_aw: gtk::Label = gtk::Label::new(Some(&emto.atomic_weigth));
		lb_aw.set_widget_name("atomic_weight");
		let boxe: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 1);

		boxe.append(&lb_an);
		boxe.append(&lb_sy);
		boxe.append(&lb_aw);

		$name.set_child(Some(&boxe));

		match emto.atomic_number {
			// 1 | 6 | 7 | 8 | 15 | 16 | 34 => gtk::prelude::WidgetExtManual::set_name(&$name,"nm"),
			1 | 6 | 7 | 8 | 15 | 16 | 34 => $name.set_widget_name("nm"),
			3 | 11 | 19 | 37 | 55 | 87 => $name.set_widget_name("ma"),
			4 | 12 | 20 | 38 | 56 | 88 => $name.set_widget_name("mat"),
			21..=30 | 39..=48 | 72..=80 | 104..=112  => $name.set_widget_name("mt"),
			5 | 14 | 32 | 33 | 51 | 52 | 84 => $name.set_widget_name("sm"),
			13 | 31 | 49 | 50 | 81 | 82 | 83 | 113 | 114 | 115 | 116 => $name.set_widget_name("om"),
			9 | 17 | 35 | 53 | 85 | 117 => $name.set_widget_name("hg"),
			2 | 10 | 18 | 36 | 54 | 86 | 118 => $name.set_widget_name("gn"),
			57..=71 => $name.set_widget_name("lt"),
			89..=103 => $name.set_widget_name("ac"),
			_ => println!("Erro, grupo inexistente"),
		}

		$name.connect_clicked (move|_| {
			println!("O botão {}", emto.name);
			let dialogo = ElementDialog::new();
			dialogo.run(&emto, &window_clone);
		});
	};
}

macro_rules! wid {
	($nome_button:expr) => {
		$nome_button
	};
}
// O uso da macro é:
// string_from_resource( variaval,
// 											 estrutura dos recursos,
// 											 "nome do arquivo")
macro_rules! string_from_resource {
	($string_var_name:ident, $resource:ident, $file_name:expr) => {

		let content = $resource::get($file_name).unwrap();

		let content_data = content.data.as_ref();

		let $string_var_name: String = String::from(std::str::from_utf8(content_data).unwrap());
	};
}
