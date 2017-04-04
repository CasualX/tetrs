/*!
Super Rotation System, or SRS for rotating tetrominoes.

Based on https://tetris.wiki/SRS
*/

use ::{Point, Piece, Rot};

/// SRS offset data.
///
/// When the player desires to rotate the piece, this table is consulted for wall kicks.
pub struct SrsData {
	cw: [[Point; 5]; 4],
	ccw: [[Point; 5]; 4],
}

macro_rules! pt {
	(($x:expr, $y:expr)) => { Point { x: $x, y: $y } };
}

macro_rules! srs {
	(
		$a:tt $b:tt $c:tt $d:tt $e:tt
		$f:tt $g:tt $h:tt $i:tt $j:tt
		$k:tt $l:tt $m:tt $n:tt $o:tt
		$p:tt $q:tt $r:tt $s:tt $t:tt
	) => {
		[[pt!($a), pt!($b), pt!($c), pt!($d), pt!($e)],
		 [pt!($f), pt!($g), pt!($h), pt!($i), pt!($j)],
		 [pt!($k), pt!($l), pt!($m), pt!($n), pt!($o)],
		 [pt!($p), pt!($q), pt!($r), pt!($s), pt!($t)]]
	}
}

/// SRS offsets for all but the I piece.
pub static SRS_DATA_JLSTZ: SrsData = SrsData {
	cw: srs! {
		( 0, 0) 	(-1, 0) 	(-1, 1) 	( 0,-2) 	(-1,-2)
		( 0, 0) 	( 1, 0) 	( 1,-1) 	( 0, 2) 	( 1, 2)
		( 0, 0) 	( 1, 0) 	( 1, 1) 	( 0,-2) 	( 1,-2)
		( 0, 0) 	(-1, 0) 	(-1,-1) 	( 0, 2) 	(-1, 2)
	},
	ccw: srs! {
		( 0, 0) 	( 1, 0) 	( 1, 1) 	( 0,-2) 	( 1,-2)
		( 0, 0) 	(-1, 0) 	(-1,-1) 	( 0, 2) 	(-1, 2)
		( 0, 0) 	(-1, 0) 	(-1, 1) 	( 0,-2) 	(-1,-2)
		( 0, 0) 	( 1, 0) 	( 1,-1) 	( 0, 2) 	( 1, 2)
	},
};

/// SRS offsets for the I piece.
pub static SRS_DATA_I: SrsData = SrsData {
	cw: srs! {
		( 0, 0) 	(-2, 0) 	( 1, 0) 	(-2,-1) 	( 1, 2)
		( 0, 0) 	(-1, 0) 	( 2, 0) 	(-1, 2) 	( 2,-1)
		( 0, 0) 	( 2, 0) 	(-1, 0) 	( 2, 1) 	(-1,-2)
		( 0, 0) 	( 1, 0) 	(-2, 0) 	( 1,-2) 	(-2, 1)
	},
	ccw: srs! {
		( 0, 0) 	(-1, 0) 	( 2, 0) 	(-1, 2) 	( 2,-1)
		( 0, 0) 	(-2, 0) 	( 1, 0) 	(-2,-1) 	( 1, 2)
		( 0, 0) 	( 1, 0) 	(-2, 0) 	( 1,-2) 	(-2, 1)
		( 0, 0) 	( 2, 0) 	(-1, 0) 	( 2, 1) 	(-1,-2)
	},
};

/*
/// SRS offsets for the I piece under Arika rules.
pub static SRS_DATA_ARIKA: SrsData = SrsData {
	cw: srs! {
		( 0, 0) 	(-2, 0) 	( 1, 0) 	( 1, 2) 	(-2,-1)
		( 0, 0) 	(-1, 0) 	( 2, 0) 	(-1, 2) 	( 2,-1)
		( 0, 0) 	( 2, 0) 	(-1, 0) 	( 2, 1) 	(-1,-1)
		( 0, 0) 	(-2, 0) 	( 1, 0) 	(-2, 1) 	( 1,-2)
	},
	ccw: srs! {
		( 0, 0) 	( 2, 0) 	(-1, 0) 	(-1, 2) 	( 2,-1)
		( 0, 0) 	( 1, 0) 	(-2, 0) 	( 1, 2) 	(-2,-1)
		( 0, 0) 	(-2, 0) 	( 1, 0) 	(-2, 1) 	( 1,-1)
		( 0, 0) 	( 2, 0) 	(-1, 0) 	( 2, 1) 	(-1,-2)
	},
};
*/

pub fn srs_cw(piece: Piece, rot: Rot) -> &'static [Point; 5] {
	let src = if piece == Piece::I { &SRS_DATA_I } else { &SRS_DATA_JLSTZ };
	&src.cw[rot as u8 as usize]
}
pub fn srs_ccw(piece: Piece, rot: Rot) -> &'static [Point; 5] {
	let src = if piece == Piece::I { &SRS_DATA_I } else { &SRS_DATA_JLSTZ };
	&src.ccw[rot as u8 as usize]
}
