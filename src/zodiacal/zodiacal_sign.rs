use strum_macros::{EnumString, EnumIter, FromRepr};

#[derive(Debug, Clone, EnumIter, EnumString, FromRepr)]
#[repr(isize)]
pub enum ZodiacalSign {
    Aries       = 0,
    Taurus      = 1,
    Gemini      = 2,
    Cancer      = 3,
    Leo         = 4,
    Virgo       = 5,
    Libra       = 6,
    Scorpio     = 7,
    Sagittarius = 8,
    Capricorn   = 9,
    Aquarius    = 10,
    Pisces      = 11,
}

impl ZodiacalSign {

    pub fn to_text(&self) -> String {
        use ZodiacalSign::*;
        match self {
            Aries       => "♈".to_owned(),

            Taurus      => "♉".to_owned(),

            Gemini      => "♊".to_owned(),

            Cancer      => "♋".to_owned(),

            Leo         => "♌".to_owned(),

            Virgo       => "♍".to_owned(),

            Libra       => "♎".to_owned(),

            Scorpio     => "♏".to_owned(),

            Sagittarius => "♐".to_owned(),

            Capricorn   => "♑".to_owned(),

            Aquarius    => "♒".to_owned(),

            Pisces      => "♓".to_owned(),
        }
    }


    pub fn to_emoji(&self) -> String {
        use ZodiacalSign::*;
        match self {
            // TODO! Emoji is technically different from text
            // may need to redo some of these
            // 	
            // ♈️U+2648
            Aries       => "♈︎".to_owned(),

            // 	♉️  U+2649
            Taurus      => "♉︎".to_owned(),

            // 	♊️	U+264A
            Gemini      => "♊︎".to_owned(),

            // 	♋️	U+264B
            Cancer      => "♋︎".to_owned(),

            // 	♌️	U+264C
            Leo         => "♌︎".to_owned(),

            // 	♍️	U+264D
            Virgo       => "♍︎".to_owned(),

            // 	♎️	U+264E
            Libra       => "♎︎".to_owned(),

            // 	♏️	U+264F
            Scorpio     => "♏︎".to_owned(),

            // 	♐️	U+2650
            Sagittarius => "♐︎".to_owned(),

            // 	♑️	U+2651
            Capricorn   => "♑︎".to_owned(),

            // 	♒️	U+2652
            Aquarius    => "♒︎".to_owned(),

            // 	♓️	U+2653
            Pisces      => "♓︎".to_owned(),
        }
    }
}

