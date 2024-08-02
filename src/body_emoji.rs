use crate::*;

pub fn body_emoji(body: &Body) -> String {
    use Body::*;
    match body {
        // U+2609
        Sun                => "☉".to_owned() ,          

        // U+263D
        Moon               => "☽".to_owned() ,          

        // U+263F
        Mercury            => "☿".to_owned() ,          

        // U+2640
        Venus              => "♀".to_owned() ,          

        // U+2642
        Mars               => "♂".to_owned() ,          

        // U+2643
        Jupiter            => "♃".to_owned() ,          

        // U+2644
        Saturn             => "♄".to_owned() ,          

        // U+2645
        Uranus             => "♅".to_owned() ,          

        // U+2646
        Neptune            => "♆".to_owned() ,          

        // ⯓ U+2BD3  and  ♇ U+2647
        Pluto              => "♇".to_owned() ,          

        // 	U+260A
        MeanNode           => "☊".to_owned() ,          

        TrueNode           => "".to_owned() ,          

        // 	U+26B8
        MeanApog           => "⚸".to_owned() ,          

        // 	U+2BDE  TODO: Check this one
        OscuApog           => "⯞".to_owned() ,          

        // 	U+1F728	
        Earth              => "🜨︎".to_owned() ,          

        //  U+26B7
        Chiron              => "⚷".to_owned(),          

        // 	U+2BDB	
        Pholus              => "⯛".to_owned(),          

        //  U+26B3
        Ceres              => "⚳".to_owned() ,          

        //  U+26B4	
        Pallas             => "⚴".to_owned() ,          

        //  U+26B5
        Juno               => "⚵".to_owned() ,          

        //  U+26B6
        Vesta              => "⚶".to_owned() ,          


//        IntpApog           => "".to_owned() ,          
//        IntpPerg           => "".to_owned() ,          


        // 	U+2BE0
        Cupido            => "⯠".to_owned(),  

        // 	U+2BE1
        Hades             => "⯡".to_owned(),  

        // 	U+2BE2
        Zeus              => "⯢".to_owned(),  

        // 	U+2BE3
        Kronos            => "⯣".to_owned(),  

        // 	U+2BE4
        Apollon           => "⯤".to_owned(),  

        // 	U+2BE5
        Admetos           => "⯥".to_owned(),  

        // 	U+2BE6
        Vulkanus          => "⯦".to_owned(),  

        // 	U+2BE7
        Poseidon          => "⯧".to_owned(),  

//        Isis              => "".to_owned(),  
//        Nibiru            => "".to_owned(),  
//        Harrington        => "".to_owned(),  
//        NeptuneLeverrier  => "".to_owned(),  
//        NeptuneAdams      => "".to_owned(),  
//        PlutoLowell       => "".to_owned(),  
//        PlutoPickering    => "".to_owned(),  

        // %, ⯙	U+0025, U+2BD9
        Astraea           => "%,".to_owned(),  

//        Hebe              => "".to_owned(),  
//        Iris              => "".to_owned(),  
//        Flora             => "".to_owned(),  
//        Metis             => "".to_owned(),  

        // 	U+2BDA
        Hygiea            => "⯚".to_owned(),  

//        Urania            => "".to_owned(),  
//        IsisAstroid       => "".to_owned(),  
//        Hilda             => "".to_owned(),  
//        Philosophia       => "".to_owned(),  
//        Sophia            => "".to_owned(),  
//        Aletheia          => "".to_owned(),  
//        Sapientia         => "".to_owned(),  
//        Thule             => "".to_owned(),  
//        Ursula            => "".to_owned(),  
//        Eros              => "".to_owned(),  
//        CupidoAstroid     => "".to_owned(),  
//        Hidalgo           => "".to_owned(),  
//        Lilith            => "".to_owned(),  
//        Amor              => "".to_owned(),  
//        Kama              => "".to_owned(),  
//        Aphrodite         => "".to_owned(),  
//        Apollo            => "".to_owned(),  
//        Damocles          => "".to_owned(),  
//        Cruithne          => "".to_owned(),  
//        PoseidonAstroid   => "".to_owned(),  
//        Vulcano           => "".to_owned(),  
//        ZeusAstroid       => "".to_owned(),  

        //  U+2BDC
        Nessus            => "⯜".to_owned(),  

        _ => "todo".to_owned()
    }
}




//☾	U+263E
//        EclNut             => "".to_owned() ,
//Pluto	Pluto (bident symbol) Pluto (bident symbol)	⯓	U+2BD3	Pluto's orb and a bident
//
//Pluto (PL monogram)	♇	U+2647	PL monogram for Pluto and Percival Lowell
//
//PlutoPluto	⯔	U+2BD4	Symbol used mainly in France, Spain, Italy and Germany.[32]
//
//Pluto	⯕	U+2BD5	Symbol invented by German astrologer Hermann Lefeldt in 1946. Used by some followers of the Hamburg School of Astrology.[32] Also proposed for Pluto's moon Charon.[21]
//
//PlutoPluto	⯖	U+2BD6	Pluto's orbit crossing that of Neptune. Symbol mostly used in German-speaking 
