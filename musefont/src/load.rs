use font_kit::loaders::freetype::Font;
use std::{path::Path, fs::File};
use crate::*;

type Error = FontLoadingError;

pub fn load(path: &Path, filename: &str, config: &FontConfig) -> Result<ScoreFont, Error> {
	let mut font_file = File::open(path.join(filename)).map_err(Error::IO)?;
	let font = Font::from_file(&mut font_file, 0).map_err(Error::Font)?;
	let mut font = ScoreFont::new(font);
	font.name = font.font.full_name();
	font.family = font.font.family_name();

	// Compute metrics for all symbols
	for (id, code) in config.sym_codes.iter().cloned().enumerate() {
		compute_metrics(&mut font.symbols[id], code, &font.font)?;
	}

	let meta_str = std::fs::read_to_string(path.join("metadata.json")).map_err(Error::IO)?;
	let meta = json::parse(&meta_str).map_err(Error::Json)?;

	// Load symbol data
	for (ident, v) in meta["glyphsWithAnchors"].entries() {
		let sym_id = config.sym_lut.get(ident).cloned().unwrap_or(SymId::NoSym);
		if sym_id == SymId::NoSym { continue; }
		parse_sym(&mut font.symbols[sym_id as usize], v)?;
	}

	// TODO: engravings

	// Load compound data and recalculte the bounding boxes
	for (sym_id, children) in COMPOSED_SYMBOLS.iter().cloned() {
		if !font.symbols[sym_id as usize].is_valid() {
			let bb = font.bounding_box_combined(children.iter().cloned(), &Size2F::new(1., 1.));
			let sym = &mut font.symbols[sym_id as usize];
			sym.compound_ids = children.iter().cloned().collect();
			sym.bbox = bb;

		}
	}

	// TODO: style

	compute_metrics(&mut font.symbols[SymId::Space as usize], 32, &font.font)?;
	Ok(font)
}

fn compute_metrics(sym: &mut Sym, code: u32, font: &Font) -> Result<(), Error> {
	if let Some(char) = std::char::from_u32(code) {
		if let Some(glyph_id) = font.glyph_for_char(char) {
			let bb = font.typographic_bounds(glyph_id).map_err(Error::Glyph)?;
			sym.code = code as i32;
			sym.index = glyph_id;
			sym.bbox = bb;
			sym.advance = font.advance(glyph_id).map_err(Error::Glyph)?.x;
		}
	}
	Ok(())
}

fn parse_sym(sym: &mut Sym, data: &json::JsonValue) -> Result<(), Error> {
	const SCALE: f32 = SPATIUM20;
	for (k, v) in data.entries() {
		match k {
			"stemDownNW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.stem_down_nw = Point2F::new(4.0 * DPI_F * x, 4.0 * DPI_F * -y);
			},
			"stemUpSE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.stem_up_se = Point2F::new(4.0 * DPI_F * x, 4.0 * DPI_F * -y);
			},
			"cutOutNE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_ne = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutNW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_nw = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutSE" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_se = Point2F::new(SCALE * x, SCALE * -y);
			},
			"cutOutSW" => {
				let (x, y) = (v[0].as_f32().unwrap_or_default(), v[1].as_f32().unwrap_or_default());
				sym.cut_out_sw = Point2F::new(SCALE * x, SCALE * -y);
			},
			_ => {},
		}
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use std::path::PathBuf;
	use crate::load::load;
	use crate::*;

	#[test]
	pub fn test_load() {
		let config = PathBuf::from("./assets/fonts/smufl");
		let path = PathBuf::from("./assets/fonts/gootville");
		let filename = "gootville.otf";
		let config = FontConfig::new(&config).unwrap();
		let mut font = load(&path, filename, &config).unwrap();

		let test = font.sym(SymId::NoteheadBlack);
		pretty_print(&mut font, SymId::NoteheadBlack);
		pretty_print(&mut font, SymId::Rest32nd);
		let i = 0;
	}

	pub fn pretty_print(font: &mut ScoreFont, sym_id: SymId) {
		let pixels = font.pixmap(sym_id, &SIZE_ONE, 64.).unwrap();

		let mut res = String::new();
		for y in 0..pixels.size.height as usize {
			for x in 0..pixels.size.width as usize {
				let idx = x + y * pixels.stride;
				if pixels.pixels[idx] > 0 {
					res.push('#');
				} else {
					res.push('.');
				}
			}
			res.push('\n');
		}
		println!("{}", res);

	}
}