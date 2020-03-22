mod window;
mod painter;

use crate::window::start_window;
use musescore::*;
use musescore::font;
use musescore::drawing::Painter;
use musescore::score::*;
use pathfinder_geometry::vector::Vector2I;
use std::path::{PathBuf};
use crate::painter::PfPainter;

pub fn main() {
	start_window(Vector2I::new(640, 480), "Musescore Demo", draw);
}

pub fn draw(painter: &mut PfPainter) {
	// TODO: font specific overrides
	let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/mscore");
	let config = font::FontMapping::load(&config).unwrap();
	let font = font::load(&path, "mscore.ttf", &config).expect("Font must load!");

	let mut state = RendererState::new();
	state.set_debug(false);
	painter.set_score_font(font.clone());
	painter.set_dpi(96.);
	painter.set_scale(6.);

	let score = Score::new(font.clone());
	//score.staves()

	let segment = Segment::new(score.clone()).with_mut_i(|mut segment| {
		segment.set_rel_time(Fraction::new(0, 4));
		segment.set_duration(Fraction::new(1, 4));
	});

	let chord = Chord::new(score.clone()).with_mut_i(|mut chord| {
		chord.set_pos(Point2F::new(100., 100.));
		chord.set_duration_type(Duration::new(DurationType::Eighth, 0))
	});
	Segment::add(segment.clone(), chord.clone().into());

	let note = Note::new(score.clone());
	chord.borrow_mut_el().add(note.clone().into());

	//ChordRenderer::layout(chord.clone());
	SegmentRenderer::layout(segment.clone());
	ChordRenderer::render(chord.clone(), &mut state, painter);
	// SegmentRenderer::render(segment.clone(), &mut state, painter);
}