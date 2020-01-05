use crate::*;
use crate::score::*;

/// Global staff data not directly related to drawing.
#[derive(Debug, Clone)]
pub struct Staff {
	element: ElementData,

	/// Part the staff element belongs to
	part: Option<ElWeak<Part>>,

	/// List of Clefs indexed using Ticks
	clefs: ClefTypeList,
	default_clef_type: ClefTypeGroup,

	/// List of Keys indexed using Ticks
	keys: KeyList,
	key_default: KeySigEvent,
	/// List of TimeSignatures indexed using Ticks
	timesigs: TimesigList,

	/// span barline to next staff
	bar_line_span: i32,
	/// line of start staff to draw the barline from (0 = staff top line, ...)
	bar_line_from: i32,
	/// line of end staff to draw the bar line to (0= staff bottom line, ...)
	bar_line_to: i32,

	color: Color,

	/// List of Staff Types indexed using Ticks
	staff_type_list: StaffTypeList,
	staff_type_default: StaffType,
}

impl Staff {
	pub fn part(&self) -> &Option<ElWeak<Part>> { &self.part }
	pub fn set_part(&mut self, v: Option<ElWeak<Part>>) { self.part = v }

	pub fn clefs(&self) -> &ClefTypeList { &self.clefs }
	pub fn set_clefs(&mut self, v: ClefTypeList) { self.clefs = v }

	pub fn default_clef_type(&self) -> &ClefTypeGroup { &self.default_clef_type }
	pub fn set_default_clef_type(&mut self, v: ClefTypeGroup) { self.default_clef_type = v }

	pub fn keys(&self) -> &KeyList { &self.keys }
	pub fn set_keys(&mut self, v: KeyList) { self.keys = v }

	pub fn timesigs(&self) -> &TimesigList { &self.timesigs }
	pub fn set_timesigs(&mut self, v: TimesigList) { self.timesigs = v }

	pub fn bar_line_span(&self) -> i32 { self.bar_line_span }
	pub fn set_bar_line_span(&mut self, v: i32) { self.bar_line_span = v }
	pub fn bar_line_from(&self) -> i32 { self.bar_line_from }
	pub fn set_bar_line_from(&mut self, v: i32) { self.bar_line_from = v }
	pub fn bar_line_to(&self) -> i32 { self.bar_line_to }
	pub fn set_bar_line_to(&mut self, v: i32) { self.bar_line_to = v }

	pub fn color(&self) -> &Color { &self.color }
	pub fn set_color(&mut self, v: Color) { self.color = v }

	pub fn key(&self, tick: &Fraction) -> Key { self.key_sig_event(tick).key() }

	pub fn lines(&self, tick: &Fraction) -> u32 { self.staff_type(tick).lines() }
	pub fn set_lines(&mut self, tick: &Fraction, v: u32) { self.staff_type_mut(tick).set_lines(v) }

	pub fn spatium(&self, tick: &Fraction) -> f32 {
		self.score().spatium() * self.mag(tick)
	}
	pub fn mag(&self, tick: &Fraction) -> f32 {
		(if self.small(tick) { self.style().value_f32(StyleName::SmallStaffMag) } else { 1.0}) * self.user_mag(tick)
	}
	pub fn user_mag(&self, tick: &Fraction) -> f32 { self.staff_type(tick).user_mag() }
	pub fn set_user_mag(&mut self, tick: &Fraction, v: f32) { self.staff_type_mut(tick).set_user_mag(v) }
	pub fn small(&self, tick: &Fraction) -> bool { self.staff_type(tick).small() }
	pub fn set_small(&mut self, tick: &Fraction, v: bool) { self.staff_type_mut(tick).set_small(v) }

	pub fn staff_type(&self, tick: &Fraction) -> &StaffType {
		self.staff_type_list.get(tick.ticks()).unwrap_or(&self.staff_type_default)
	}
	pub fn staff_type_mut(&mut self, tick: &Fraction) -> &mut StaffType {
		self.staff_type_list.get_mut(tick.ticks()).unwrap_or(&mut self.staff_type_default)
	}
	pub fn set_staff_type(&mut self, tick: &Fraction, v: StaffType) {
		self.staff_type_list.set(tick.ticks(), v)
	}

	pub fn key_sig_event(&self, tick: &Fraction) -> &KeySigEvent {
		self.keys.get(tick.ticks()).unwrap_or(&self.key_default)
	}
	pub fn key_sig_event_mut(&mut self, tick: &Fraction) -> &mut KeySigEvent {
		self.keys.get_mut(tick.ticks()).unwrap_or(&mut self.key_default)
	}
}

impl Element for Staff {
	fn el_data(&self) -> &ElementData { &self.element }
	fn el_data_mut(&mut self) -> &mut ElementData { &mut self.element }

	fn element_type(&self) -> ElementType { ElementType::Staff }

	fn staff(&self) -> Option<El<Staff>> { self.get_ref_ty() }
	fn part(&self) -> Option<El<Part>> { self.part.as_ref().and_then(ElWeak::upgrade) }
}

pub type ClefTypeList = OrderedCollecton<ClefTypeGroup>;
pub type KeyList = OrderedCollecton<KeySigEvent>;
pub type TimesigList = OrderedCollecton<El<TimeSig>>;
pub type StaffTypeList = OrderedCollecton<StaffType>;