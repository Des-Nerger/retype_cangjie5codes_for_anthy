#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

use {
	arrayvec::ArrayVec,
	enigo::{
		Direction::{Click, Press, Release},
		Enigo, Key, Keyboard, Settings,
	},
	std::{
		fmt::Write,
		io::{self, BufRead},
	},
};

fn main() {
	trait EnigoExt {
		fn switchOver_inputMethod(&mut self);
	}
	impl EnigoExt for Enigo {
		fn switchOver_inputMethod(&mut self) {
			_ = self.key(Key::Control, Press);
			_ = self.key(Key::Unicode(' '), Click);
			_ = self.key(Key::Control, Release);
		}
	}
	let (enigo, stdin, inpLineBuf, outpLineBuf) =
		&mut (Enigo::new(&Settings::default()).unwrap(), io::stdin().lock(), String::new(), String::new());
	while {
		inpLineBuf.clear();
		stdin.read_line(inpLineBuf).unwrap() != 0
	} {
		let inpLine = inpLineBuf.trim_end_matches(['\n', '\r']);
		if inpLine == "" || inpLine.starts_with('#') {
			continue;
		}
		const MAX_NUM_FIELDS: usize = 14;
		let fields = inpLine.splitn(MAX_NUM_FIELDS, ' ').collect::<ArrayVec<_, MAX_NUM_FIELDS>>();
		if ![fields[2], fields[5]].contains(&"1") {
			continue;
		}
		for code in fields[11].split(',') {
			if code.starts_with(['z', 'x', 'N']) {
				continue;
			}

			outpLineBuf.clear();
			_ = outpLineBuf.write_str(code);
			_ = outpLineBuf.write_char('\n');
			_ = enigo.text(outpLineBuf);
			enigo.switchOver_inputMethod();

			outpLineBuf.clear();
			_ = outpLineBuf.write_str(" #T35*500 ");
			_ = outpLineBuf.write_str(fields[0]);
			_ = outpLineBuf.write_char('\n');
			_ = enigo.text(outpLineBuf);
			enigo.switchOver_inputMethod();
		}
	}
}
