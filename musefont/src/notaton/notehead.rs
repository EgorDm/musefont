use crate::*;

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Type {
	Whole = 0,
	Half = 1,
	Quarter = 2,
	Brevis = 3,
	Auto = 4,
}

impl Type {
	pub fn get_symid(&self, dir: DirectionV, group: Group) -> SymId {
		self.get_keyed_symid(dir, group, Scheme::Normal, 0, Key::C)
	}

	pub fn get_keyed_symid(&self, dir: DirectionV, group: Group, scheme: Scheme, tpc: i32, key: Key) -> SymId {
		(match scheme {
			Scheme::Normal => NOTE_HEADS[dir as usize][group as usize][*self as usize],
			Scheme::Pitchname | Scheme::PitchnameGerman => unimplemented!(),
			Scheme::ShapeNote4 => unimplemented!(),
			Scheme::ShapeNote7Aikin | Scheme::ShapeNote7Funk | Scheme::ShapeNote7Walker => unimplemented!(),
			Scheme::Solfege => unimplemented!(),
			Scheme::SolfegeFixed => unimplemented!(),
		}) as SymId
	}
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Scheme {
	Normal = 0,
	Pitchname = 1,
	PitchnameGerman = 2,
	Solfege = 3,
	SolfegeFixed = 4,
	ShapeNote4 = 5,
	ShapeNote7Aikin = 6,
	ShapeNote7Funk = 7,
	ShapeNote7Walker = 8,
}

#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum Group {
	Normal = 0,
	Cross = 1,
	Plus = 2,
	XCircle = 3,
	Withx = 4,
	TriangleUp = 5,
	TriangleDown = 6,
	Slashed1 = 7,
	Slashed2 = 8,
	Diamond = 9,
	DiamondOld = 10,
	Circled = 11,
	CircledLarge = 12,
	LargeArrow = 13,
	BrevisAlt = 14,

	Slash = 15,

	Sol = 16,
	La = 17,
	Fa = 18,
	Mi = 19,
	Do = 20,
	Re = 21,
	Ti = 22,

	DoWalker = 23,
	ReWalker = 24,
	TiWalker = 25,
	DoFunk = 26,
	ReFunk = 27,
	TiFunk = 28,

	DoName = 29,
	ReName = 30,
	MiName = 31,
	FaName = 32,
	SolName = 33,
	LaName = 34,
	TiName = 35,
	SiName = 36,

	ASharp = 37,
	A = 38,
	AFlat = 39,
	BSharp = 40,
	B = 41,
	BFlat = 42,
	CSharp = 43,
	C = 44,
	CFlat = 45,
	DSharp = 46,
	D = 47,
	DFlat = 48,
	ESharp = 49,
	E = 50,
	EFlat = 51,
	FSharp = 52,
	F = 53,
	FFlat = 54,
	GSharp = 55,
	G = 56,
	GFlat = 57,
	H = 58,
	HSharp = 59,

	Custom = 60,
	Invalid = 61,
}

pub const NOTE_HEAD_GROUP_COUNT: usize = 60;
pub const NOTE_HEAD_COUNT: usize = 4;

const NOTE_HEADS: [[[SymIdent; NOTE_HEAD_COUNT]; NOTE_HEAD_GROUP_COUNT]; 2] = {[
	{[     // down stem
		[SymIdent::NoteheadWhole, SymIdent::NoteheadHalf, SymIdent::NoteheadBlack, SymIdent::NoteheadDoubleWhole],
		[SymIdent::NoteheadXWhole, SymIdent::NoteheadXHalf, SymIdent::NoteheadXBlack, SymIdent::NoteheadXDoubleWhole],
		[SymIdent::NoteheadPlusWhole, SymIdent::NoteheadPlusHalf, SymIdent::NoteheadPlusBlack, SymIdent::NoteheadPlusDoubleWhole],
		[SymIdent::NoteheadCircleXWhole, SymIdent::NoteheadCircleXHalf, SymIdent::NoteheadCircleX, SymIdent::NoteheadCircleXDoubleWhole],
		[SymIdent::NoteheadWholeWithX, SymIdent::NoteheadHalfWithX, SymIdent::NoteheadVoidWithX, SymIdent::NoteheadDoubleWholeWithX],
		[SymIdent::NoteheadTriangleUpWhole, SymIdent::NoteheadTriangleUpHalf, SymIdent::NoteheadTriangleUpBlack, SymIdent::NoteheadTriangleUpDoubleWhole],
		[SymIdent::NoteheadTriangleDownWhole, SymIdent::NoteheadTriangleDownHalf, SymIdent::NoteheadTriangleDownBlack, SymIdent::NoteheadTriangleDownDoubleWhole],
		[SymIdent::NoteheadSlashedWhole1, SymIdent::NoteheadSlashedHalf1, SymIdent::NoteheadSlashedBlack1, SymIdent::NoteheadSlashedDoubleWhole1],
		[SymIdent::NoteheadSlashedWhole2, SymIdent::NoteheadSlashedHalf2, SymIdent::NoteheadSlashedBlack2, SymIdent::NoteheadSlashedDoubleWhole2],
		[SymIdent::NoteheadDiamondWhole, SymIdent::NoteheadDiamondHalf, SymIdent::NoteheadDiamondBlack, SymIdent::NoteheadDiamondDoubleWhole],
		[SymIdent::NoteheadDiamondWholeOld, SymIdent::NoteheadDiamondHalfOld, SymIdent::NoteheadDiamondBlackOld, SymIdent::NoteheadDiamondDoubleWholeOld],
		[SymIdent::NoteheadCircledWhole, SymIdent::NoteheadCircledHalf, SymIdent::NoteheadCircledBlack, SymIdent::NoteheadCircledDoubleWhole],
		[SymIdent::NoteheadCircledWholeLarge, SymIdent::NoteheadCircledHalfLarge, SymIdent::NoteheadCircledBlackLarge, SymIdent::NoteheadCircledDoubleWholeLarge],
		[SymIdent::NoteheadLargeArrowUpWhole, SymIdent::NoteheadLargeArrowUpHalf, SymIdent::NoteheadLargeArrowUpBlack, SymIdent::NoteheadLargeArrowUpDoubleWhole],
		[SymIdent::NoteheadWhole, SymIdent::NoteheadHalf, SymIdent::NoteheadBlack, SymIdent::NoteheadDoubleWholeSquare],
		[SymIdent::NoteheadSlashWhiteWhole, SymIdent::NoteheadSlashWhiteHalf, SymIdent::NoteheadSlashHorizontalEnds, SymIdent::NoteheadSlashWhiteWhole],
		[SymIdent::NoteShapeRoundWhite, SymIdent::NoteShapeRoundWhite, SymIdent::NoteShapeRoundBlack, SymIdent::NoteShapeRoundDoubleWhole],
		[SymIdent::NoteShapeSquareWhite, SymIdent::NoteShapeSquareWhite, SymIdent::NoteShapeSquareBlack, SymIdent::NoteShapeSquareDoubleWhole],
		[SymIdent::NoteShapeTriangleRightWhite, SymIdent::NoteShapeTriangleRightWhite, SymIdent::NoteShapeTriangleRightBlack, SymIdent::NoteShapeTriangleRightDoubleWhole],
		[SymIdent::NoteShapeDiamondWhite, SymIdent::NoteShapeDiamondWhite, SymIdent::NoteShapeDiamondBlack, SymIdent::NoteShapeDiamondDoubleWhole],
		[SymIdent::NoteShapeTriangleUpWhite, SymIdent::NoteShapeTriangleUpWhite, SymIdent::NoteShapeTriangleUpBlack, SymIdent::NoteShapeTriangleUpDoubleWhole],
		[SymIdent::NoteShapeMoonWhite, SymIdent::NoteShapeMoonWhite, SymIdent::NoteShapeMoonBlack, SymIdent::NoteShapeMoonDoubleWhole],
		[SymIdent::NoteShapeTriangleRoundWhite, SymIdent::NoteShapeTriangleRoundWhite, SymIdent::NoteShapeTriangleRoundBlack, SymIdent::NoteShapeTriangleRoundDoubleWhole],
		[SymIdent::NoteShapeKeystoneWhite, SymIdent::NoteShapeKeystoneWhite, SymIdent::NoteShapeKeystoneBlack, SymIdent::NoteShapeKeystoneDoubleWhole],
		[SymIdent::NoteShapeQuarterMoonWhite, SymIdent::NoteShapeQuarterMoonWhite, SymIdent::NoteShapeQuarterMoonBlack, SymIdent::NoteShapeQuarterMoonDoubleWhole],
		[SymIdent::NoteShapeIsoscelesTriangleWhite, SymIdent::NoteShapeIsoscelesTriangleWhite, SymIdent::NoteShapeIsoscelesTriangleBlack, SymIdent::NoteShapeIsoscelesTriangleDoubleWhole],
		[SymIdent::NoteShapeMoonLeftWhite, SymIdent::NoteShapeMoonLeftWhite, SymIdent::NoteShapeMoonLeftBlack, SymIdent::NoteShapeMoonLeftDoubleWhole],
		[SymIdent::NoteShapeArrowheadLeftWhite, SymIdent::NoteShapeArrowheadLeftWhite, SymIdent::NoteShapeArrowheadLeftBlack, SymIdent::NoteShapeArrowheadLeftDoubleWhole],
		[SymIdent::NoteShapeTriangleRoundLeftWhite, SymIdent::NoteShapeTriangleRoundLeftWhite, SymIdent::NoteShapeTriangleRoundLeftBlack, SymIdent::NoteShapeTriangleRoundLeftDoubleWhole],
		[SymIdent::NoteDoWhole, SymIdent::NoteDoHalf, SymIdent::NoteDoBlack, SymIdent::NoSym],
		[SymIdent::NoteReWhole, SymIdent::NoteReHalf, SymIdent::NoteReBlack, SymIdent::NoSym],
		[SymIdent::NoteMiWhole, SymIdent::NoteMiHalf, SymIdent::NoteMiBlack, SymIdent::NoSym],
		[SymIdent::NoteFaWhole, SymIdent::NoteFaHalf, SymIdent::NoteFaBlack, SymIdent::NoSym],
		[SymIdent::NoteSoWhole, SymIdent::NoteSoHalf, SymIdent::NoteSoBlack, SymIdent::NoSym],
		[SymIdent::NoteLaWhole, SymIdent::NoteLaHalf, SymIdent::NoteLaBlack, SymIdent::NoSym],
		[SymIdent::NoteTiWhole, SymIdent::NoteTiHalf, SymIdent::NoteTiBlack, SymIdent::NoSym],
		[SymIdent::NoteSiWhole, SymIdent::NoteSiHalf, SymIdent::NoteSiBlack, SymIdent::NoSym],
		[SymIdent::NoteASharpWhole, SymIdent::NoteASharpHalf, SymIdent::NoteASharpBlack, SymIdent::NoSym],
		[SymIdent::NoteAWhole, SymIdent::NoteAHalf, SymIdent::NoteABlack, SymIdent::NoSym],
		[SymIdent::NoteAFlatWhole, SymIdent::NoteAFlatHalf, SymIdent::NoteAFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteBSharpWhole, SymIdent::NoteBSharpHalf, SymIdent::NoteBSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteBWhole, SymIdent::NoteBHalf, SymIdent::NoteBBlack, SymIdent::NoSym],
		[SymIdent::NoteBFlatWhole, SymIdent::NoteBFlatHalf, SymIdent::NoteBFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteCSharpWhole, SymIdent::NoteCSharpHalf, SymIdent::NoteCSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteCWhole, SymIdent::NoteCHalf, SymIdent::NoteCBlack, SymIdent::NoSym],
		[SymIdent::NoteCFlatWhole, SymIdent::NoteCFlatHalf, SymIdent::NoteCFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteDSharpWhole, SymIdent::NoteDSharpHalf, SymIdent::NoteDSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteDWhole, SymIdent::NoteDHalf, SymIdent::NoteDBlack, SymIdent::NoSym],
		[SymIdent::NoteDFlatWhole, SymIdent::NoteDFlatHalf, SymIdent::NoteDFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteESharpWhole, SymIdent::NoteESharpHalf, SymIdent::NoteESharpBlack, SymIdent::NoSym],
		[SymIdent::NoteEWhole, SymIdent::NoteEHalf, SymIdent::NoteEBlack, SymIdent::NoSym],
		[SymIdent::NoteEFlatWhole, SymIdent::NoteEFlatHalf, SymIdent::NoteEFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteFSharpWhole, SymIdent::NoteFSharpHalf, SymIdent::NoteFSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteFWhole, SymIdent::NoteFHalf, SymIdent::NoteFBlack, SymIdent::NoSym],
		[SymIdent::NoteFFlatWhole, SymIdent::NoteFFlatHalf, SymIdent::NoteFFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteGSharpWhole, SymIdent::NoteGSharpHalf, SymIdent::NoteGSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteGWhole, SymIdent::NoteGHalf, SymIdent::NoteGBlack, SymIdent::NoSym],
		[SymIdent::NoteGFlatWhole, SymIdent::NoteGFlatHalf, SymIdent::NoteGFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteHWhole, SymIdent::NoteHHalf, SymIdent::NoteHBlack, SymIdent::NoSym],
		[SymIdent::NoteHSharpWhole, SymIdent::NoteHSharpHalf, SymIdent::NoteHSharpBlack, SymIdent::NoSym]
	]},
	{[     // up stem
		[SymIdent::NoteheadWhole, SymIdent::NoteheadHalf, SymIdent::NoteheadBlack, SymIdent::NoteheadDoubleWhole],
		[SymIdent::NoteheadXWhole, SymIdent::NoteheadXHalf, SymIdent::NoteheadXBlack, SymIdent::NoteheadXDoubleWhole],
		[SymIdent::NoteheadPlusWhole, SymIdent::NoteheadPlusHalf, SymIdent::NoteheadPlusBlack, SymIdent::NoteheadPlusDoubleWhole],
		[SymIdent::NoteheadCircleXWhole, SymIdent::NoteheadCircleXHalf, SymIdent::NoteheadCircleX, SymIdent::NoteheadCircleXDoubleWhole],
		[SymIdent::NoteheadWholeWithX, SymIdent::NoteheadHalfWithX, SymIdent::NoteheadVoidWithX, SymIdent::NoteheadDoubleWholeWithX],
		[SymIdent::NoteheadTriangleUpWhole, SymIdent::NoteheadTriangleUpHalf, SymIdent::NoteheadTriangleUpBlack, SymIdent::NoteheadTriangleUpDoubleWhole],
		[SymIdent::NoteheadTriangleDownWhole, SymIdent::NoteheadTriangleDownHalf, SymIdent::NoteheadTriangleDownBlack, SymIdent::NoteheadTriangleDownDoubleWhole],
		[SymIdent::NoteheadSlashedWhole1, SymIdent::NoteheadSlashedHalf1, SymIdent::NoteheadSlashedBlack1, SymIdent::NoteheadSlashedDoubleWhole1],
		[SymIdent::NoteheadSlashedWhole2, SymIdent::NoteheadSlashedHalf2, SymIdent::NoteheadSlashedBlack2, SymIdent::NoteheadSlashedDoubleWhole2],
		[SymIdent::NoteheadDiamondWhole, SymIdent::NoteheadDiamondHalf, SymIdent::NoteheadDiamondBlack, SymIdent::NoteheadDiamondDoubleWhole],
		[SymIdent::NoteheadDiamondWholeOld, SymIdent::NoteheadDiamondHalfOld, SymIdent::NoteheadDiamondBlackOld, SymIdent::NoteheadDiamondDoubleWholeOld],
		[SymIdent::NoteheadCircledWhole, SymIdent::NoteheadCircledHalf, SymIdent::NoteheadCircledBlack, SymIdent::NoteheadCircledDoubleWhole],
		[SymIdent::NoteheadCircledWholeLarge, SymIdent::NoteheadCircledHalfLarge, SymIdent::NoteheadCircledBlackLarge, SymIdent::NoteheadCircledDoubleWholeLarge],
		// different from down, find source?
		[SymIdent::NoteheadLargeArrowDownWhole, SymIdent::NoteheadLargeArrowDownHalf, SymIdent::NoteheadLargeArrowDownBlack, SymIdent::NoteheadLargeArrowDownDoubleWhole],
		[SymIdent::NoteheadWhole, SymIdent::NoteheadHalf, SymIdent::NoteheadBlack, SymIdent::NoteheadDoubleWholeSquare],
		[SymIdent::NoteheadSlashWhiteWhole, SymIdent::NoteheadSlashWhiteHalf, SymIdent::NoteheadSlashHorizontalEnds, SymIdent::NoteheadSlashWhiteDoubleWhole],
		[SymIdent::NoteShapeRoundWhite, SymIdent::NoteShapeRoundWhite, SymIdent::NoteShapeRoundBlack, SymIdent::NoteShapeRoundDoubleWhole],
		[SymIdent::NoteShapeSquareWhite, SymIdent::NoteShapeSquareWhite, SymIdent::NoteShapeSquareBlack, SymIdent::NoteShapeSquareDoubleWhole],
		// different from down
		[SymIdent::NoteShapeTriangleLeftWhite, SymIdent::NoteShapeTriangleLeftWhite, SymIdent::NoteShapeTriangleLeftBlack, SymIdent::NoteShapeTriangleLeftDoubleWhole],
		[SymIdent::NoteShapeDiamondWhite, SymIdent::NoteShapeDiamondWhite, SymIdent::NoteShapeDiamondBlack, SymIdent::NoteShapeDiamondDoubleWhole],
		[SymIdent::NoteShapeTriangleUpWhite, SymIdent::NoteShapeTriangleUpWhite, SymIdent::NoteShapeTriangleUpBlack, SymIdent::NoteShapeTriangleUpDoubleWhole],
		[SymIdent::NoteShapeMoonWhite, SymIdent::NoteShapeMoonWhite, SymIdent::NoteShapeMoonBlack, SymIdent::NoteShapeMoonDoubleWhole],
		[SymIdent::NoteShapeTriangleRoundWhite, SymIdent::NoteShapeTriangleRoundWhite, SymIdent::NoteShapeTriangleRoundBlack, SymIdent::NoteShapeTriangleRoundDoubleWhole],
		[SymIdent::NoteShapeKeystoneWhite, SymIdent::NoteShapeKeystoneWhite, SymIdent::NoteShapeKeystoneBlack, SymIdent::NoteShapeKeystoneDoubleWhole],
		[SymIdent::NoteShapeQuarterMoonWhite, SymIdent::NoteShapeQuarterMoonWhite, SymIdent::NoteShapeQuarterMoonBlack, SymIdent::NoteShapeQuarterMoonDoubleWhole],
		[SymIdent::NoteShapeIsoscelesTriangleWhite, SymIdent::NoteShapeIsoscelesTriangleWhite, SymIdent::NoteShapeIsoscelesTriangleBlack, SymIdent::NoteShapeIsoscelesTriangleDoubleWhole],
		[SymIdent::NoteShapeMoonLeftWhite, SymIdent::NoteShapeMoonLeftWhite, SymIdent::NoteShapeMoonLeftBlack, SymIdent::NoteShapeMoonLeftDoubleWhole],
		[SymIdent::NoteShapeArrowheadLeftWhite, SymIdent::NoteShapeArrowheadLeftWhite, SymIdent::NoteShapeArrowheadLeftBlack, SymIdent::NoteShapeArrowheadLeftDoubleWhole],
		[SymIdent::NoteShapeTriangleRoundLeftWhite, SymIdent::NoteShapeTriangleRoundLeftWhite, SymIdent::NoteShapeTriangleRoundLeftBlack, SymIdent::NoteShapeTriangleRoundLeftDoubleWhole],
		[SymIdent::NoteDoWhole, SymIdent::NoteDoHalf, SymIdent::NoteDoBlack, SymIdent::NoSym],
		[SymIdent::NoteReWhole, SymIdent::NoteReHalf, SymIdent::NoteReBlack, SymIdent::NoSym],
		[SymIdent::NoteMiWhole, SymIdent::NoteMiHalf, SymIdent::NoteMiBlack, SymIdent::NoSym],
		[SymIdent::NoteFaWhole, SymIdent::NoteFaHalf, SymIdent::NoteFaBlack, SymIdent::NoSym],
		[SymIdent::NoteSoWhole, SymIdent::NoteSoHalf, SymIdent::NoteSoBlack, SymIdent::NoSym],
		[SymIdent::NoteLaWhole, SymIdent::NoteLaHalf, SymIdent::NoteLaBlack, SymIdent::NoSym],
		[SymIdent::NoteTiWhole, SymIdent::NoteTiHalf, SymIdent::NoteTiBlack, SymIdent::NoSym],
		[SymIdent::NoteSiWhole, SymIdent::NoteSiHalf, SymIdent::NoteSiBlack, SymIdent::NoSym],
		[SymIdent::NoteASharpWhole, SymIdent::NoteASharpHalf, SymIdent::NoteASharpBlack, SymIdent::NoSym],
		[SymIdent::NoteAWhole, SymIdent::NoteAHalf, SymIdent::NoteABlack, SymIdent::NoSym],
		[SymIdent::NoteAFlatWhole, SymIdent::NoteAFlatHalf, SymIdent::NoteAFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteBSharpWhole, SymIdent::NoteBSharpHalf, SymIdent::NoteBSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteBWhole, SymIdent::NoteBHalf, SymIdent::NoteBBlack, SymIdent::NoSym],
		[SymIdent::NoteBFlatWhole, SymIdent::NoteBFlatHalf, SymIdent::NoteBFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteCSharpWhole, SymIdent::NoteCSharpHalf, SymIdent::NoteCSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteCWhole, SymIdent::NoteCHalf, SymIdent::NoteCBlack, SymIdent::NoSym],
		[SymIdent::NoteCFlatWhole, SymIdent::NoteCFlatHalf, SymIdent::NoteCFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteDSharpWhole, SymIdent::NoteDSharpHalf, SymIdent::NoteDSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteDWhole, SymIdent::NoteDHalf, SymIdent::NoteDBlack, SymIdent::NoSym],
		[SymIdent::NoteDFlatWhole, SymIdent::NoteDFlatHalf, SymIdent::NoteDFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteESharpWhole, SymIdent::NoteESharpHalf, SymIdent::NoteESharpBlack, SymIdent::NoSym],
		[SymIdent::NoteEWhole, SymIdent::NoteEHalf, SymIdent::NoteEBlack, SymIdent::NoSym],
		[SymIdent::NoteEFlatWhole, SymIdent::NoteEFlatHalf, SymIdent::NoteEFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteFSharpWhole, SymIdent::NoteFSharpHalf, SymIdent::NoteFSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteFWhole, SymIdent::NoteFHalf, SymIdent::NoteFBlack, SymIdent::NoSym],
		[SymIdent::NoteFFlatWhole, SymIdent::NoteFFlatHalf, SymIdent::NoteFFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteGSharpWhole, SymIdent::NoteGSharpHalf, SymIdent::NoteGSharpBlack, SymIdent::NoSym],
		[SymIdent::NoteGWhole, SymIdent::NoteGHalf, SymIdent::NoteGBlack, SymIdent::NoSym],
		[SymIdent::NoteGFlatWhole, SymIdent::NoteGFlatHalf, SymIdent::NoteGFlatBlack, SymIdent::NoSym],
		[SymIdent::NoteHWhole, SymIdent::NoteHHalf, SymIdent::NoteHBlack, SymIdent::NoSym],
		[SymIdent::NoteHSharpWhole, SymIdent::NoteHSharpHalf, SymIdent::NoteHSharpBlack, SymIdent::NoSym]
	]}
]};