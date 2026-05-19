//! A list of all of the named colors available with links to the color map on the Pebble Developer website.
//!
//! > **Bindings Note**
//! >
//! > I rearranged this module a bit, to hopefully make it easier to use.
//! >
//! > Also fixed the colour picker links!

use crate::graphics::graphics_types::GColor8;

#[repr(u8)]
pub enum GColorArgb8 {
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#000000>
	Black = 0b11000000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#000055>
	OxfordBlue = 0b11000001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0000AA>
	DukeBlue = 0b11000010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0000FF>
	Blue = 0b11000011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#005500>
	DarkGreen = 0b11000100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#005555>
	MidnightGreen = 0b11000101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0055AA>
	CobaltBlue = 0b11000110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0055FF>
	BlueMoon = 0b11000111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AA00>
	IslamicGreen = 0b11001000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AA55>
	JaegerGreen = 0b11001001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AAAA>
	TiffanyBlue = 0b11001010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AAFF>
	VividCerulean = 0b11001011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FF00>
	Green = 0b11001100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FF55>
	Malachite = 0b11001101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FFAA>
	MediumSpringGreen = 0b11001110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FFFF>
	Cyan = 0b11001111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#550000>
	BulgarianRose = 0b11010000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#550055>
	ImperialPurple = 0b11010001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5500AA>
	Indigo = 0b11010010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5500FF>
	ElectricUltramarine = 0b11010011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#555500>
	ArmyGreen = 0b11010100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#555555>
	DarkGray = 0b11010101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5555AA>
	Liberty = 0b11010110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5555FF>
	VeryLightBlue = 0b11010111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AA00>
	KellyGreen = 0b11011000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AA55>
	MayGreen = 0b11011001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AAAA>
	CadetBlue = 0b11011010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AAFF>
	PictonBlue = 0b11011011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FF00>
	BrightGreen = 0b11011100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FF55>
	ScreaminGreen = 0b11011101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FFAA>
	MediumAquamarine = 0b11011110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FFFF>
	ElectricBlue = 0b11011111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0000>
	DarkCandyAppleRed = 0b11100000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0055>
	JazzberryJam = 0b11100001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA00AA>
	Purple = 0b11100010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA00FF>
	VividViolet = 0b11100011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0AA0>
	WindsorTan = 0b11100100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA5555>
	RoseVale = 0b11100101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA55AA>
	Purpureus = 0b11100110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA55FF>
	LavenderIndigo = 0b11100111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAA00>
	Limerick = 0b11101000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAA55>
	Brass = 0b11101001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAAAA>
	LightGray = 0b11101010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAAFF>
	BabyBlueEyes = 0b11101011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFF00>
	SpringBud = 0b11101100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFF55>
	Inchworm = 0b11101101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFFAA>
	MintGreen = 0b11101110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFFFF>
	Celeste = 0b11101111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF0000>
	Red = 0b11110000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF0055>
	Folly = 0b11110001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF00AA>
	FashionMagenta = 0b11110010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF00FF>
	Magenta = 0b11110011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF5500>
	Orange = 0b11110100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF5555>
	SunsetOrange = 0b11110101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF55AA>
	BrilliantRose = 0b11110110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF55FF>
	ShockingPink = 0b11110111,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAA00>
	ChromeYellow = 0b11111000,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAA55>
	Rajah = 0b11111001,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAAAA>
	Melon = 0b11111010,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAAFF>
	RichBrilliantLavender = 0b11111011,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFF00>
	Yellow = 0b11111100,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFF55>
	Icterine = 0b11111101,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFFAA>
	PastelYellow = 0b11111110,
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFFFF>
	White = 0b11111111,
}

impl GColorArgb8 {
	pub const fn into_u8(self) -> u8 {
		self as u8
	}

	pub const fn into_g_color(self) -> GColor8 {
		GColor8 {
			argb: self.into_u8(),
		}
	}
}

pub mod g_color {
	use super::{super::GColor8, GColorArgb8};

	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#000000>
	pub const BLACK: GColor8 = GColorArgb8::Black.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#000055>
	pub const OXFORD_BLUE: GColor8 = GColorArgb8::OxfordBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0000AA>
	pub const DUKE_BLUE: GColor8 = GColorArgb8::DukeBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0000FF>
	pub const BLUE: GColor8 = GColorArgb8::Blue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#005500>
	pub const DARK_GREEN: GColor8 = GColorArgb8::DarkGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#005555>
	pub const MIDNIGHT_GREEN: GColor8 = GColorArgb8::MidnightGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0055AA>
	pub const COBALT_BLUE: GColor8 = GColorArgb8::CobaltBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#0055FF>
	pub const BLUE_MOON: GColor8 = GColorArgb8::BlueMoon.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AA00>
	pub const ISLAMIC_GREEN: GColor8 = GColorArgb8::IslamicGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AA55>
	pub const JAEGER_GREEN: GColor8 = GColorArgb8::JaegerGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AAAA>
	pub const TIFFANY_BLUE: GColor8 = GColorArgb8::TiffanyBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00AAFF>
	pub const VIVID_CERULEAN: GColor8 = GColorArgb8::VividCerulean.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FF00>
	pub const GREEN: GColor8 = GColorArgb8::Green.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FF55>
	pub const MALACHITE: GColor8 = GColorArgb8::Malachite.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FFAA>
	pub const MEDIUM_SPRING_GREEN: GColor8 = GColorArgb8::MediumSpringGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#00FFFF>
	pub const CYAN: GColor8 = GColorArgb8::Cyan.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#550000>
	pub const BULGARIAN_ROSE: GColor8 = GColorArgb8::BulgarianRose.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#550055>
	pub const IMPERIAL_PURPLE: GColor8 = GColorArgb8::ImperialPurple.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5500AA>
	pub const INDIGO: GColor8 = GColorArgb8::Indigo.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5500FF>
	pub const ELECTRIC_ULTRAMARINE: GColor8 = GColorArgb8::ElectricUltramarine.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#555500>
	pub const ARMY_GREEN: GColor8 = GColorArgb8::ArmyGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#555555>
	pub const DARK_GRAY: GColor8 = GColorArgb8::DarkGray.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5555AA>
	pub const LIBERTY: GColor8 = GColorArgb8::Liberty.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#5555FF>
	pub const VERY_LIGHT_BLUE: GColor8 = GColorArgb8::VeryLightBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AA00>
	pub const KELLY_GREEN: GColor8 = GColorArgb8::KellyGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AA55>
	pub const MAY_GREEN: GColor8 = GColorArgb8::MayGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AAAA>
	pub const CADET_BLUE: GColor8 = GColorArgb8::CadetBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55AAFF>
	pub const PICTON_BLUE: GColor8 = GColorArgb8::PictonBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FF00>
	pub const BRIGHT_GREEN: GColor8 = GColorArgb8::BrightGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FF55>
	pub const SCREAMIN_GREEN: GColor8 = GColorArgb8::ScreaminGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FFAA>
	pub const MEDIUM_AQUAMARINE: GColor8 = GColorArgb8::MediumAquamarine.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#55FFFF>
	pub const ELECTRIC_BLUE: GColor8 = GColorArgb8::ElectricBlue.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0000>
	pub const DARK_CANDY_APPLE_RED: GColor8 = GColorArgb8::DarkCandyAppleRed.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0055>
	pub const JAZZBERRY_JAM: GColor8 = GColorArgb8::JazzberryJam.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA00AA>
	pub const PURPLE: GColor8 = GColorArgb8::Purple.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA00FF>
	pub const VIVID_VIOLET: GColor8 = GColorArgb8::VividViolet.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA0AA0>
	pub const WINDSOR_TAN: GColor8 = GColorArgb8::WindsorTan.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA5555>
	pub const ROSE_VALE: GColor8 = GColorArgb8::RoseVale.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA55AA>
	pub const PURPUREUS: GColor8 = GColorArgb8::Purpureus.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AA55FF>
	pub const LAVENDER_INDIGO: GColor8 = GColorArgb8::LavenderIndigo.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAA00>
	pub const LIMERICK: GColor8 = GColorArgb8::Limerick.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAA55>
	pub const BRASS: GColor8 = GColorArgb8::Brass.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAAAA>
	pub const LIGHT_GRAY: GColor8 = GColorArgb8::LightGray.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAAAFF>
	pub const BABY_BLUE_EYES: GColor8 = GColorArgb8::BabyBlueEyes.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFF00>
	pub const SPRING_BUD: GColor8 = GColorArgb8::SpringBud.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFF55>
	pub const INCHWORM: GColor8 = GColorArgb8::Inchworm.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFFAA>
	pub const MINT_GREEN: GColor8 = GColorArgb8::MintGreen.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#AAFFFF>
	pub const CELESTE: GColor8 = GColorArgb8::Celeste.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF0000>
	pub const RED: GColor8 = GColorArgb8::Red.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF0055>
	pub const FOLLY: GColor8 = GColorArgb8::Folly.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF00AA>
	pub const FASHION_MAGENTA: GColor8 = GColorArgb8::FashionMagenta.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF00FF>
	pub const MAGENTA: GColor8 = GColorArgb8::Magenta.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF5500>
	pub const ORANGE: GColor8 = GColorArgb8::Orange.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF5555>
	pub const SUNSET_ORANGE: GColor8 = GColorArgb8::SunsetOrange.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF55AA>
	pub const BRILLIANT_ROSE: GColor8 = GColorArgb8::BrilliantRose.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FF55FF>
	pub const SHOCKING_PINK: GColor8 = GColorArgb8::ShockingPink.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAA00>
	pub const CHROME_YELLOW: GColor8 = GColorArgb8::ChromeYellow.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAA55>
	pub const RAJAH: GColor8 = GColorArgb8::Rajah.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAAAA>
	pub const MELON: GColor8 = GColorArgb8::Melon.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFAAFF>
	pub const RICH_BRILLIANT_LAVENDER: GColor8 = GColorArgb8::RichBrilliantLavender.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFF00>
	pub const YELLOW: GColor8 = GColorArgb8::Yellow.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFF55>
	pub const ICTERINE: GColor8 = GColorArgb8::Icterine.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFFAA>
	pub const PASTEL_YELLOW: GColor8 = GColorArgb8::PastelYellow.into_g_color();
	/// <https://developer.repebble.com/guides/tools-and-resources/color-picker/#FFFFFF>
	pub const WHITE: GColor8 = GColorArgb8::White.into_g_color();
}
