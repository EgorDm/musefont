use crate::font::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct KeySigEvent {
	key: Key,
	mode: KeyMode,
	custom: bool,
	key_symbols: Vec<KeySym>,
}

impl Default for KeySigEvent {
	fn default() -> Self { Self {
		key: Key::Invalid,
		mode: KeyMode::Unknown,
		custom: false,
		key_symbols: vec![]
	}}
}

impl KeySigEvent {
	pub fn from_key(key: Key) -> Self {
		let mut res = Self::default();
		res.set_key(key);
		res
	}

	pub fn key(&self) -> Key { self.key }
	pub fn mode(&self) -> KeyMode { self.mode }
	pub fn custom(&self) -> bool { self.custom}
	pub fn is_valid(&self) -> bool { self.key != Key::Invalid }
	pub fn is_atonal(&self) -> bool { self.mode == KeyMode::None }
	pub fn key_symbols(&self) -> &Vec<KeySym> { &self.key_symbols }

	pub fn set_key(&mut self, key: Key) {
		self.key = key;
		self.custom = false;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct KeySym {
	sym: SymId,
	spos: Point2F, // position in spatium units
	pos: Point2F,  // actual pixel position on screen (set by layout)
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
	C_B = -7,
	G_B = -6,
	D_B = -5,
	A_B = -4,
	E_B = -3,
	B_B = -2,
	F = -1,
	C = 0,
	G = 1,
	D = 2,
	A = 3,
	E = 4,
	B = 5,
	F_S = 6,
	C_S = 7,
	DeltaEnharmonic = 12,
	Invalid = 13
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyMode {
	Unknown = 0,
	None = 1,
	Major = 2,
	Minor = 3,
	Dorian = 4,
	Phrygian = 5,
	Lydian = 6,
	Mixolydian = 7,
	Aeolian = 8,
	Ionian = 9,
	Locrian = 10,
}

pub const KEY_MIN: Key = Key::C_B;
pub const KEY_MAX: Key = Key::C_S;
pub const KEY_COUNT: u32 = 15;