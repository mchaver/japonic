use util;

pub const ROMAJI_TO_HIRAGANA_TABLE: &'static [(&str, &str)] = &[
    ("a","あ"),("ba","ば"),("be","べ"),("bi","び"),("bo","ぼ"),("bu","ぶ"),("bya","びゃ"),("byo","びょう"),("byu","びゅ"),("cha","ちゃ"),("che","ちぇ"),("chi","ち"),("cho","ちょ"),("chu","ちゅ"),("da","だ"),("de","で"),("di","でぃ"),("do","ど"),("du","どぅ"),("dya","ぢゃ"),("dye","ぢぇ"),("dyi","ぢぃ"),("dyo","ぢょ"),("dyu","ぢゅ"),("e","え"),("fa","ふぁ"),("fe","ふぇ"),("fi","ふぃ"),("fo","ふぉ"),("fu","ふ"),("ga","が"),("ge","げ"),("gi","ぎ"),("go","ご"),("gu","ぐ"),("gwa","ぐゎ"),("gwe","ぐぇ"),("gwi","ぐぃ"),("gwo","ぐぉ"),("gya","ぎゃ"),("gyo","ぎょ"),("gyu","ぎゅ"),("ha","は"),("he","へ"),("hi","ひ"),("ho","ほ"),("hu","ふ"),("hya","ひゃ"),("hyo","ひょ"),("hyu","ひゅ"),("i","い"),("ja","じゃ"),("je","じぇ"),("ji","じ"),("jo","じょ"),("ju","じゅ"),("ka","か"),("ke","け"),("ki","き"),("ko","こ"),("ku","く"),("kwa","くゎ"),("kwe","くぇ"),("kwi","くぃ"),("kwo","くぉ"),("kya","きゃ"),("kyo","きょ"),("kyu","きゅ"),("ma","ま"),("me","め"),("mi","み"),("mo","も"),("mu","む"),("mya","みゃ"),("myo","みょ"),("myu","みゅ"),("na","な"),("ne","ね"),("ni","に"),("nn","ん"),("no","の"),("nu","ぬ"),("nya","にゃ"),("nyo","にょ"),("nyu","にゅ"),("o","お"),("pa","ぱ"),("pe","ぺ"),("pi","ぴ"),("po","ぽ"),("pu","ぷ"),("pya","ぴゃ"),("pyo","ぴょお"),("pyu","ぴゅ"),("qnn","っん"),("qwa","っわ"),("qwe","っゑ"),("qwi","っゐ"),("qwo","っを"),("qya","っや"),("qyo","っよ"),("qyu","っゆ"),("ra","ら"),("re","れ"),("ri","り"),("ro","ろ"),("ru","る"),("rya","りゃ"),("ryo","りょ"),("ryu","りゅ"),("sa","さ"),("se","せ"),("sha","しゃ"),("she","しぇ"),("shi","し"),("sho","しょ"),("shu","しゅ"),("si","すぃ"),("so","そ"),("su","す"),("ta","た"),("te","て"),("ti","てぃ"),("to","と"),("tsa","つぁ"),("tse","つぇ"),("tsi","つぃ"),("tso","つぉ"),("tsu","つ"),("tu","とぅ"),("u","う"),("wa","わ"),("we","ゑ"),("wi","ゐ"),("wo","を"),("wu","をぉ"),("xtsu","っ"),("xtu","っ"),("ya","や"),("ye","えぇ"),("yi","いぃ"),("yo","よ"),("yu","ゆ"),("za","ざ"),("ze","ぜ"),("zi","ずぃ"),("zo","ぞ"),("zu","ず")
];

pub const HIRAGANA_TO_ROMAJI_TABLE: &'static [(&str, &str)] = &[
    ("あ","a"),("い","i"),("いぃ","yi"),("う","u"),("え","e"),("えぇ","ye"),("お","o"),("か","ka"),("が","ga"),("き","ki"),("きゃ","kya"),("きゅ","kyu"),("きょ","kyo"),("ぎ","gi"),("ぎゃ","gya"),("ぎゅ","gyu"),("ぎょ","gyo"),("く","ku"),("くぃ","kwi"),("くぇ","kwe"),("くぉ","kwo"),("くゎ","kwa"),("ぐ","gu"),("ぐぃ","gwi"),("ぐぇ","gwe"),("ぐぉ","gwo"),("ぐゎ","gwa"),("け","ke"),("げ","ge"),("こ","ko"),("ご","go"),("さ","sa"),("ざ","za"),("し","shi"),("しぇ","she"),("しゃ","sha"),("しゅ","shu"),("しょ","sho"),("じ","ji"),("じぇ","je"),("じゃ","ja"),("じゅ","ju"),("じょ","jo"),("す","su"),("すぃ","si"),("ず","zu"),("ずぃ","zi"),("せ","se"),("ぜ","ze"),("そ","so"),("ぞ","zo"),("た","ta"),("だ","da"),("ち","chi"),("ちぇ","che"),("ちゃ","cha"),("ちゅ","chu"),("ちょ","cho"),("ぢぃ","dyi"),("ぢぇ","dye"),("ぢゃ","dya"),("ぢゅ","dyu"),("ぢょ","dyo"),("っ","xtsu"),("っ","xtu"),("っや","qya"),("っゆ","qyu"),("っよ","qyo"),("っわ","qwa"),("っゐ","qwi"),("っゑ","qwe"),("っを","qwo"),("っん","qnn"),("つ","tsu"),("つぁ","tsa"),("つぃ","tsi"),("つぇ","tse"),("つぉ","tso"),("て","te"),("てぃ","ti"),("で","de"),("でぃ","di"),("と","to"),("とぅ","tu"),("ど","do"),("どぅ","du"),("な","na"),("に","ni"),("にゃ","nya"),("にゅ","nyu"),("にょ","nyo"),("ぬ","nu"),("ね","ne"),("の","no"),("は","ha"),("ば","ba"),("ぱ","pa"),("ひ","hi"),("ひゃ","hya"),("ひゅ","hyu"),("ひょ","hyo"),("び","bi"),("びゃ","bya"),("びゅ","byu"),("びょう","byo"),("ぴ","pi"),("ぴゃ","pya"),("ぴゅ","pyu"),("ぴょお","pyo"),("ふ","fu"),("ふ","hu"),("ふぁ","fa"),("ふぃ","fi"),("ふぇ","fe"),("ふぉ","fo"),("ぶ","bu"),("ぷ","pu"),("へ","he"),("べ","be"),("ぺ","pe"),("ほ","ho"),("ぼ","bo"),("ぽ","po"),("ま","ma"),("み","mi"),("みゃ","mya"),("みゅ","myu"),("みょ","myo"),("む","mu"),("め","me"),("も","mo"),("や","ya"),("ゆ","yu"),("よ","yo"),("ら","ra"),("り","ri"),("りゃ","rya"),("りゅ","ryu"),("りょ","ryo"),("る","ru"),("れ","re"),("ろ","ro"),("わ","wa"),("ゐ","wi"),("ゑ","we"),("を","wo"),("をぉ","wu"),("ん","nn")
];

pub fn romaji_to_hiragana(romaji: &str) -> String {
    let romaji_len = romaji.len();

    let mut romaji_index = 0;
    let mut window = 1; // 1 to 3
    let mut hiragana = "".to_string();

    // convert 1 to 3 ascii chars to a single hirgana
    // i is starting point, window is length to be looked up
    while romaji_index < romaji_len && romaji_index + window < romaji_len + 1 {
        let romaji_sub = &romaji[romaji_index .. romaji_index + window];

        // check for gemminate consonant, if it exists, consume the char
        // add a "っ" and skip the lookup
        if window == 1 && romaji_index + 1 < romaji_len && util::is_consonant(romaji_sub) {
            let romaji_peek_ahead = &romaji[romaji_index + 1 .. romaji_index + 2];

            if romaji_sub == romaji_peek_ahead {
                hiragana = format!("{}{}", hiragana, "っ");
                romaji_index += 1;
                window = 1;
                continue;
            }
        }
        
        match util::lookup(romaji_sub, ROMAJI_TO_HIRAGANA_TABLE) {
            Some(hiragana_index) => {
                let single_hiragana = ROMAJI_TO_HIRAGANA_TABLE[hiragana_index].1.to_string();
                hiragana = format!("{}{}", hiragana, single_hiragana);
                romaji_index += window;
                window = 1;
            },
            None => {
                // romaji_sub was not found in the table, increase the window size if it was
                // not found. If the window is too large then move the index over and reset
                // the window size to 1
                if window < 3 {
                    window += 1;
                } else {
                    romaji_index += 1;
                    window = 1;
                }
            },
        }
    }
    
    hiragana.to_string()
}

pub fn hiragana_to_romaji(hiragana: &str) -> String {
    let hiragana_chars = hiragana.chars();
    let hiragana_len = hiragana_chars.count();

    let mut hiragana_index = 0;
    let mut window = 
        if hiragana_len > 1 {
            2
        } else {
            1
        };
    let mut romaji = "".to_string();

    while hiragana_index < hiragana_len && hiragana_index + window < hiragana_len + 1 {
        let hiragana_sub_vec = &hiragana.chars().collect::<Vec<_>>()[hiragana_index .. hiragana_index + window];
        let hiragana_sub_string: String = hiragana_sub_vec.into_iter().collect();
        let hiragana_sub: &str = &hiragana_sub_string[..];

        match util::lookup(hiragana_sub, HIRAGANA_TO_ROMAJI_TABLE) {
            Some(romaji_index) => {
                let single_romaji = HIRAGANA_TO_ROMAJI_TABLE[romaji_index].1.to_string();
                romaji = format!("{}{}", romaji, single_romaji);
                hiragana_index += window;
                window =
                    if hiragana_len - hiragana_index > 1 {
                        2
                    } else {
                        1
                    };
            },
            None => {
                // hiragana_sub was not found in the table, increase the window size if it was
                // not found. If the window is too large then move the index over and reset
                // the window size relative to the length of string.
                if window > 1 {
                    window -= 1;
                } else {
                    hiragana_index += 1;
                    window =
                        if hiragana_len - hiragana_index > 1 {
                            2
                        } else {
                            1
                        };
                }
            },
        }
    }

    romaji.to_string()
}
/*
import Text.Show.Unicode (uprint)
import Data.List (sort)
import Data.Tuple (swap)
uprint $ sort $ swap <$> x
*/


pub enum VerbType {
          // headword ending, nonpast negative, gerund
    Ia1,  // in, ran, ti
    Ia2,  // in, (r)an, ti
    Ib1,  // in, ran, chi
    Ib2,  // in, ran, tchi
    Ic1,  // ri-in, ran, t-ti
    Ic2,  // ri-in, ri-ran, t-ti
    IIa1, // jun, gan, ji
    IIa2, // jun, dan, ti
    IIa3, // jun, dan, chi
    IIb1, // chun, kan, chi
    IIb2, // chun, tan, tchi
    IIc,  // sun, san, chi
    IId,  // bun, ban, di
    IIe,  // mun, man, di
    IIf,  // nun, nan, ji
    III,  // n, ran, ti
    IV    // irregular
}

/*
PRED predicative
ATT attributive - attributive is a word or phrase within a noun phrase that modifies the head noun
ru = attributive

基本語幹 	連用語幹 	派生語幹 	音便語幹
那覇方言の第1種動詞の語幹
書く 	ka 	k 	c 	c 	c 	kacuN(書く)、cicuN(聞く)、sacuN(咲く)、ʔaQcuN(歩く)
漕ぐ 	kuu 	g 	z 	z 	z 	kuuzuN(漕ぐ)、ʔwiizuN(泳ぐ)、ʔoozuN(扇ぐ)
立つ 	ta 	t 	c 	c 	Qc 	tacuN(立つ)、ʔucuN(打つ)、kacuN(勝つ)
育つ 	sura 	t 	c 	c 	c 	suracuN(育つ)、tamucuN(保つ)、kucuN(朽ちる)
殺す 	kuru 	s 	s 	s 	c 	kurusuN(殺す)、meesuN(燃やす)、haNsuN(外す)
為る 	‐ 	s 	s 	s 	s 	suN(為る)、siQkwasuN(敷く)、hiQkoosuN(比較する)
呼ぶ 	ju 	b 	b 	b 	r/d 	jubuN(呼ぶ)、tubuN(飛ぶ)、musubuN(結ぶ)
読む 	ju 	m 	m 	m 	r/d 	jumuN(読む)、numuN(飲む)、ʔamuN(編む)
眠る 	niN 	r 	z 	z 	t 	niNzuN(眠る)、kaNzuN(被る)、ʔaNzuN(あぶる)
見る 	NN 	r 	z 	z 	c 	NNzuN(見る)、kuNzuN(括る)
死ぬ 	shi     n       n       n       j       shinuN 死ぬん

那覇方言の第2種動詞の語幹
取る 	tu 	r 	○/j 	○ 	Qt 	tuiN(取る)
刈る 	ka 	r 	○/j 	○ 	t 	kaiN(刈る)、nubuiN(登る)、ʔaraiN(洗う)
蹴る 	ki 	r 	○/j 	○ 	Qc 	kiiN(蹴る)、ʔiiN(入る)、hiiN(放る)、ciiN(切る)
煮る 	ni 	r 	○/j 	○ 	c/(Qt) 	niiN(煮る)、ciiN(着る)、ʔiiN(言う)、iiN(座る)

那覇方言の第3種動詞の語幹
有る 	ʔa 	r 	○/j/i 	○ 	t 	ʔaN(有る)、uN(居る)、jaN(である)
*/

pub enum VerbTypes {
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    II1,
    II2,
    II3,
    II4,
    III,
    IV
}

pub enum VerbStem {
    Base,       // 基本語幹 base
    // 基本語幹+a  : N（否定）, riiN（可能・受身）, suN(使役)
    // 基本語幹+ee : 条件形, 命令形
    // 基本語幹+i  : 命令形
    // 基本語幹+u  : na(な。禁止), ka(まで), kazirii(まで・かぎり)

    Connective, // 連用語幹 connective
    // 連用語幹+i : ga(〜しに), ciroo(〜しそう), uusuN(〜できる), busaN(〜したい)
    // 連用語幹+(j)abiiN/ibiiN :

    // 連用形 をり

    Derivative, // 派生語幹 derivative stem
    // 派生語幹+uN/iN/N          : 終止形(現在形)
    // 派生語幹+uru/iru/ru       : baa(〜時)、hazi(〜はず), ru(ぞ)
    // 派生語幹+ura/ira/ra       : 疑問の助詞ga(か)
    // 派生語幹+uraa/iraa/raa    : 「〜なら」という条件を表す。
    // 派生語幹+u/i/○            : si（の）、siga（〜のだが）、sa（よ）、gutu（理由）、ga（疑問）、mi・i（たずね）
    // 派生語幹+utaN/itaN/taN    : 〜していた = 過去進行形
    // 派生語幹+uti/iti/ti       : 〜していたか = 過去進行中止形
    // 派生語幹+uteeN/iteeN/teeN : 〜していただろう = 過去進行推量形

    Euphonic    // 音便語幹 euphonic change stem
    // 音便語幹+i   : 〜して
    // 音便語幹+aN  : 〜した
    // 音便語幹+eeN : (今までに)きっと〜している,〜したに違いない,〜してある
    // 音便語幹+ooN : 〜している
}

pub enum VerbConjugation {
    NonPast,         // in/un/n 辞書形
    NonPastNegative, // ~an 否定形
    Past,            // ~an
    PastNegative,    // ~antan

    // ClauseEnding,    // i, does/ and 連用形
    // Connective,      // (y)a
    

    YesNoInterrogative, // ~mi
    WhInterrogative, // ~ga

    Honorific,
    Potential, // able to ~juusun
    Desiderative, // desire, want to
    Imperative,
    Prohibitive, // prohibitive

    Volitional,
    Causative, // ~sun
    Passive, // riiN rijuN
    Continuative, // ti form
    AttributiveNonPast, // N -> ru
    AttributivePast, // N -> ru
    Progressive, // 
    
    Gerund,           // 音便語幹+i   : 〜して, ti ティ形
    NonPastPolite,   // biin
    NonPastNegativePolite, // biran
    InterrogativePolite, // biimi
    InterrogativePoliteII, // biiga
    PastPolite, // bitan
    PastNegativePoilte, // birantan
    InterrogativePastPolite, // bitii
    InterrogativePastPoliteII // bitiiga
}


pub fn conjugate_iv_verbs(verb: &str, conjugation: VerbConjugation) -> &str {
    if verb == "chuun" {
        match conjugation {
            VerbConjugation::NonPastNegative => "kuun",
            VerbConjugation::PastNegative => "kuuntan",
            VerbConjugation::Past => "chan",
            VerbConjugation::NonPastPolite => "chaabiin",
            VerbConjugation::Honorific => "menseen",
            VerbConjugation::Potential => "chiyuusun",
            VerbConjugation::Desiderative => "chibusan",
            VerbConjugation::Volitional => "kuu",
            VerbConjugation::YesNoInterrogative => "chuumi",
            VerbConjugation::WhInterrogative => "chuuga",
            VerbConjugation::Imperative => "kuu",
            VerbConjugation::Prohibitive => "kuuna",
            VerbConjugation::Causative => "kuurasun",
            VerbConjugation::AttributiveNonPast => "chuuru",
            VerbConjugation::AttributivePast => "charu",
            VerbConjugation::Progressive => "choon",
            VerbConjugation::Continuative => "chchi",
            _ => ""
        }
    } else if verb == "yan" { // to be (copula)
        match conjugation {
            VerbConjugation::NonPastNegative => "aran",
            VerbConjugation::PastNegative => "arantan",
            VerbConjugation::Past => "yatan",
            VerbConjugation::NonPastPolite => "yaibiin", // yayabiin
            // attributive yaru
            // attriute past yataru
            _ => ""
        }
    } else if verb == "sun" { // to do
        match conjugation {
            VerbConjugation::NonPastNegative => "san",
            VerbConjugation::PastNegative => "santan",
            VerbConjugation::Past => "san",
            VerbConjugation::NonPastPolite => "sabiin",
            _ => ""
        }
    } else if verb == "an" { // to be (inanimate)
        match conjugation {
            VerbConjugation::NonPastNegative => "neeran", // neen
            VerbConjugation::PastNegative => "neerantan",
            VerbConjugation::Past => "neeran",
            VerbConjugation::NonPastPolite => "ayabiin",
            _ => ""
        }
    } else if verb == "'yun" { // to say
        match conjugation {
            VerbConjugation::NonPastNegative => "'yan",
            VerbConjugation::PastNegative => "'yantan",
            VerbConjugation::Past => "'yan",
            VerbConjugation::NonPastPolite => "'yabiin",

            VerbConjugation::YesNoInterrogative => "qyurumi",
            VerbConjugation::WhInterrogative => "qyuruga",
            VerbConjugation::Imperative => "ii",
            VerbConjugation::Prohibitive => "qyuna",
            VerbConjugation::Causative => "qyashimiyun", // qyashimiin
            VerbConjugation::AttributiveNonPast => "qyuru",
            VerbConjugation::AttributivePast => "icharu",
            VerbConjugation::Progressive => "qnjoon",
            // ii-busan desire, ii-busatan
            // ichi te form
            // seems: qyuru-haji qyan-haji
            _ => ""
        }
    } else if verb == "umuin" { // to think
        match conjugation {
            VerbConjugation::NonPastNegative => "umaan", // umuran
            VerbConjugation::PastNegative => "umaantan", // umurantan
            VerbConjugation::Past => "umaan",
            VerbConjugation::NonPastPolite => "umuyabiin", // umuibiin
            _ => ""
        }
    } else if verb == "ichun" { // to go
        match conjugation {
            VerbConjugation::NonPastNegative => "ikan",
            VerbConjugation::PastNegative => "ikantan",
            VerbConjugation::Past => "ikan",
            VerbConjugation::NonPastPolite => "ichabiin",

            VerbConjugation::YesNoInterrogative => "ichumi",
            VerbConjugation::WhInterrogative => "ichuga",
            VerbConjugation::Imperative => "iki",
            VerbConjugation::Prohibitive => "ikuna",
            VerbConjugation::Causative => "ikasun",
            VerbConjugation::AttributiveNonPast => "ichuru",
            VerbConjugation::AttributivePast => "'njaru",
            VerbConjugation::Progressive => "qnjoon",
            // prgressive 'qnjoon
            // lets' ika
            // seems ichuru-haji ikan-haji
            // 'nji te form
            _ => ""
        }
    } else {
        ""
    }
}

pub fn conjugate_verbs(verb: &str, vt: VerbTypes, conjugation: VerbConjugation) -> String {
    match conjugation {
        VerbConjugation::NonPastNegative => format!("{}{}", get_verb_stem(verb,VerbStem::Base,vt), "aN"),
        VerbConjugation::PastNegative => format!("{}{}", get_verb_stem(verb,VerbStem::Base,vt), "aNtaN"),
        VerbConjugation::Past => format!("{}{}", get_verb_stem(verb,VerbStem::Euphonic,vt), "aN"),

        VerbConjugation::NonPastPolite => match vt {
            VerbTypes::III => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "jabiiN"), // or ibiiN
            VerbTypes::II1 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "jabiiN"),
            VerbTypes::II2 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "jabiiN"),
            VerbTypes::II3 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "jabiiN"),
            VerbTypes::II4 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "jabiiN"),
            _ => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "abiiN")
        },

        VerbConjugation::YesNoInterrogative => match vt{
            VerbTypes::II1 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "imi"),
            VerbTypes::II2 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "imi"),
            VerbTypes::II3 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "imi"),
            VerbTypes::II4 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "imi"),
            _ => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "umi"),
        }
        VerbConjugation::WhInterrogative => match vt{
            VerbTypes::II1 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "iga"),
            VerbTypes::II2 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "iga"),
            VerbTypes::II3 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "iga"),
            VerbTypes::II4 => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "iga"),
            _ => format!("{}{}", get_verb_stem(verb,VerbStem::Derivative,vt), "uga"),
        }
        
        VerbConjugation::Honorific => match vt {
            VerbTypes::II1 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "miseeN"),
            VerbTypes::II2 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "miseeN"),
            VerbTypes::II3 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "miseeN"),
            VerbTypes::II4 => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "miseeN"),
            _ => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "imiseeN"),
        },
        VerbConjugation::Potential => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "ijuusuN"),
        VerbConjugation::Desiderative => format!("{}{}", get_verb_stem(verb,VerbStem::Connective,vt), "ibusaN"),

        VerbConjugation::Imperative => format!("{}{}", get_verb_stem(verb,VerbStem::Base,vt), "ee"),
        VerbConjugation::Prohibitive => match vt {
            VerbTypes::II1 => replace_last(&get_verb_stem(verb,VerbStem::Base,vt).to_owned(), "nna"),
            VerbTypes::II2 => replace_last(&get_verb_stem(verb,VerbStem::Base,vt), "nna"),
            VerbTypes::II3 => replace_last(&get_verb_stem(verb,VerbStem::Base,vt), "nna"),
            VerbTypes::II4 => replace_last(&get_verb_stem(verb,VerbStem::Base,vt), "nna"),
            _ => format!("{}{}", get_verb_stem(verb,VerbStem::Base,vt), "una"),
        }

        VerbConjugation::Gerund => format!("{}{}", get_verb_stem(verb,VerbStem::Euphonic,vt), "i"),
        _ => "".to_string()
    }
}

fn get_verb_stem(word: &str, verb_stem: VerbStem, verb_type: VerbTypes) -> String {
    let stem = match verb_type {
        VerbTypes::III => truncate_chars(word, (word.chars().count() - 1)),
        _              => truncate_chars(word, (word.chars().count() - 2))

    };
    
    match (verb_type, verb_stem) {
        (_, VerbStem::Derivative) => stem.to_string(),

        (VerbTypes::I1, VerbStem::Base)       => replace_last(stem, "k"),
        (VerbTypes::I1, VerbStem::Connective) => replace_last(stem, "c"),
        (VerbTypes::I1, VerbStem::Euphonic)   => replace_last(stem, "c"),

        (VerbTypes::I2, VerbStem::Base)       => replace_last(stem, "g"),
        (VerbTypes::I2, VerbStem::Connective) => replace_last(stem, "z"),
        (VerbTypes::I2, VerbStem::Euphonic)   => replace_last(stem, "z"),

        (VerbTypes::I3, VerbStem::Base)       => replace_last(stem, "t"),
        (VerbTypes::I3, VerbStem::Connective) => replace_last(stem, "c"),
        (VerbTypes::I3, VerbStem::Euphonic)   => replace_last(stem, "Qc"),

        (VerbTypes::I4, VerbStem::Base)       => replace_last(stem, "t"),
        (VerbTypes::I4, VerbStem::Connective) => replace_last(stem, "c"),
        (VerbTypes::I4, VerbStem::Euphonic)   => replace_last(stem, "c"),

        (VerbTypes::I5, VerbStem::Base)       => replace_last(stem, "s"),
        (VerbTypes::I5, VerbStem::Connective) => replace_last(stem, "s"),
        (VerbTypes::I5, VerbStem::Euphonic)   => replace_last(stem, "c"),

        (VerbTypes::I6, VerbStem::Base)       => replace_last(stem, "s"),
        (VerbTypes::I6, VerbStem::Connective) => replace_last(stem, "s"),
        (VerbTypes::I6, VerbStem::Euphonic)   => replace_last(stem, "s"),

        (VerbTypes::I7, VerbStem::Base)       => replace_last(stem, "b"),
        (VerbTypes::I7, VerbStem::Connective) => replace_last(stem, "b"),
        (VerbTypes::I7, VerbStem::Euphonic)   => replace_last(stem, "r"),

        (VerbTypes::I8, VerbStem::Base)       => replace_last(stem, "m"),
        (VerbTypes::I8, VerbStem::Connective) => replace_last(stem, "m"),
        (VerbTypes::I8, VerbStem::Euphonic)   => replace_last(stem, "r"),

        (VerbTypes::I9, VerbStem::Base)       => replace_last(stem, "r"),
        (VerbTypes::I9, VerbStem::Connective) => replace_last(stem, "z"),
        (VerbTypes::I9, VerbStem::Euphonic)   => replace_last(stem, "t"),

        (VerbTypes::I10, VerbStem::Base)       => replace_last(stem, "r"),
        (VerbTypes::I10, VerbStem::Connective) => replace_last(stem, "z"),
        (VerbTypes::I10, VerbStem::Euphonic)   => replace_last(stem, "c"),

        (VerbTypes::II1, VerbStem::Base)       => format!("{}{}", stem, "r"),
        (VerbTypes::II1, VerbStem::Connective) => stem.to_string(),
        // (VerbTypes::II1, VerbStem::Connective) => format!("{}{}", stem, "y"),
        (VerbTypes::II1, VerbStem::Euphonic)   => format!("{}{}", stem, "Qt"),

        (VerbTypes::II2, VerbStem::Base)       => format!("{}{}", stem, "r"),
        (VerbTypes::II2, VerbStem::Connective) => stem.to_string(),
        // (VerbTypes::II2, VerbStem::Connective) => format!("{}{}", stem, "y"),
        (VerbTypes::II2, VerbStem::Euphonic)   => format!("{}{}", stem, "t"),

        (VerbTypes::II3, VerbStem::Base)       => format!("{}{}", stem, "r"),
        (VerbTypes::II3, VerbStem::Connective) => stem.to_string(),
        // (VerbTypes::II3, VerbStem::Connective) => format!("{}{}", stem, "j"),
        (VerbTypes::II3, VerbStem::Euphonic)   => format!("{}{}", stem, "Qc"),
        
        (VerbTypes::II4, VerbStem::Base)       => format!("{}{}", stem, "r"),
        (VerbTypes::II4, VerbStem::Connective) => stem.to_string(),
        // (VerbTypes::II4, VerbStem::Connective) => format!("{}{}", stem, "z"),
        (VerbTypes::II4, VerbStem::Euphonic)   => format!("{}{}", stem, "c"),
        // (VerbTypes::II4, VerbStem::Euphonic)   => replace_last(stem, "Qt"),

        (VerbTypes::III, VerbStem::Base)       => format!("{}{}", stem, "r"),
        (VerbTypes::III, VerbStem::Connective) => stem.to_string(), // ○/j/i
        (VerbTypes::III, VerbStem::Euphonic)   => format!("{}{}", stem, "t"),

        _ => "".to_string()
    }
}

pub fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn replace_last(x: &str, y: &str) -> String {
    format!("{}{}", truncate_chars(x, x.chars().count()-1), y)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_verb_stem() {
        assert_eq!(get_verb_stem("kacuN",VerbStem::Base,VerbTypes::I1), "kak".to_string());
        assert_eq!(get_verb_stem("kacuN",VerbStem::Connective,VerbTypes::I1), "kac".to_string());
        assert_eq!(get_verb_stem("kacuN",VerbStem::Euphonic,VerbTypes::I1), "kac".to_string());

        assert_eq!(get_verb_stem("cicuN",VerbStem::Base,VerbTypes::I1), "cik".to_string());
        assert_eq!(get_verb_stem("cicuN",VerbStem::Connective,VerbTypes::I1), "cic".to_string());
        assert_eq!(get_verb_stem("cicuN",VerbStem::Euphonic,VerbTypes::I1), "cic".to_string());

        assert_eq!(get_verb_stem("uN",VerbStem::Base,VerbTypes::III), "ur".to_string());
        assert_eq!(get_verb_stem("uN",VerbStem::Connective,VerbTypes::III), "u".to_string());
        assert_eq!(get_verb_stem("uN",VerbStem::Euphonic,VerbTypes::III), "ut".to_string());

        assert_eq!(get_verb_stem("jaN",VerbStem::Base,VerbTypes::III), "jar".to_string());
        assert_eq!(get_verb_stem("jaN",VerbStem::Connective,VerbTypes::III), "ja".to_string());
        assert_eq!(get_verb_stem("jaN",VerbStem::Euphonic,VerbTypes::III), "jat".to_string());

        assert_eq!(get_verb_stem("jumuN",VerbStem::Base,VerbTypes::I8), "jum".to_string());
        assert_eq!(get_verb_stem("jumuN",VerbStem::Connective,VerbTypes::I8), "jum".to_string());
        assert_eq!(get_verb_stem("jumuN",VerbStem::Euphonic,VerbTypes::I8), "jur".to_string());
    }

    #[test]
    fn test_iv() {

    }

    #[test]
    fn test_conjugate_verb() {
        assert_eq!(conjugate_verbs("jaN",VerbTypes::III, VerbConjugation::NonPastNegative), "jaraN".to_string());
        // conjugations
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::NonPastNegative), "kakaN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::PastNegative), "kakaNtaN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::Past), "kacaN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::NonPastPolite), "kacabiiN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::Desiderative), "kacibusaN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::Honorific), "kacimiseeN".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::Imperative), "kakee".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::Prohibitive), "kakuna".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::YesNoInterrogative), "kacumi".to_string());
        assert_eq!(conjugate_verbs("kacuN",VerbTypes::I1, VerbConjugation::WhInterrogative), "kacuga".to_string());
        
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::NonPastNegative), "mutaN".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::PastNegative), "mutaNtaN".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::Past), "muQcaN".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::NonPastPolite), "mucabiiN".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::Desiderative), "mucibusaN".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::Imperative), "mutee".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::Prohibitive), "mutuna".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::YesNoInterrogative), "mucumi".to_string());
        assert_eq!(conjugate_verbs("mucuN",VerbTypes::I3, VerbConjugation::WhInterrogative), "mucuga".to_string());
        
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::NonPastNegative), "jumaN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::PastNegative), "jumaNtaN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::Past), "juraN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::NonPastPolite), "jumabiiN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::Desiderative), "jumibusaN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::Honorific), "jumimiseeN".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::Imperative), "jumee".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::Prohibitive), "jumuna".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::YesNoInterrogative), "jumumi".to_string());
        assert_eq!(conjugate_verbs("jumuN",VerbTypes::I8, VerbConjugation::WhInterrogative), "jumuga".to_string());
        
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::NonPastNegative), "turaN".to_string());
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::PastNegative), "turaNtaN".to_string());
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Past), "tutaN".to_string());
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Desiderative), "tuibusaN".to_string()); // tuibusaN
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Honorific), "tumiseeN".to_string()); // tuibusaN
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Imperative), "turee".to_string()); // tuibusaN
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Prohibitive), "tunna".to_string()); // tuibusaN
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::YesNoInterrogative), "tuimi".to_string());
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::WhInterrogative), "tuiga".to_string());
    }

    #[test]
    fn test_gerund() {
        assert_eq!(conjugate_verbs("kacuN",    VerbTypes::I1 , VerbConjugation::Gerund), "kaci".to_string());
        assert_eq!(conjugate_verbs("kuuzuN",   VerbTypes::I2 , VerbConjugation::Gerund), "kuuzi".to_string());
        assert_eq!(conjugate_verbs("tacuN",    VerbTypes::I3 , VerbConjugation::Gerund), "taQci".to_string());
        assert_eq!(conjugate_verbs("suracuN",  VerbTypes::I4 , VerbConjugation::Gerund), "suraci".to_string());
        assert_eq!(conjugate_verbs("kurusuN",  VerbTypes::I5 , VerbConjugation::Gerund), "kuruci".to_string());
        assert_eq!(conjugate_verbs("hiQkoosuN",VerbTypes::I6 , VerbConjugation::Gerund), "hiQkoosi".to_string());
        assert_eq!(conjugate_verbs("jubuN",    VerbTypes::I7 , VerbConjugation::Gerund), "juri".to_string());
        assert_eq!(conjugate_verbs("jumuN",    VerbTypes::I8 , VerbConjugation::Gerund), "juri".to_string());
        assert_eq!(conjugate_verbs("niNzuN",   VerbTypes::I9 , VerbConjugation::Gerund), "niNti".to_string());
        assert_eq!(conjugate_verbs("NNzuN",    VerbTypes::I10, VerbConjugation::Gerund), "NNci".to_string());
        // assert_eq!(conjugate_verbs("NNzuN",VerbTypes::I10, VerbConjugation::Gerund), "tuti".to_string());
        
        assert_eq!(conjugate_verbs("tuiN",VerbTypes::II2, VerbConjugation::Gerund), "tuti".to_string());
        assert_eq!(conjugate_verbs("kiiN",VerbTypes::II3, VerbConjugation::Gerund), "kiQci".to_string());
        assert_eq!(conjugate_verbs("niiN",VerbTypes::II4, VerbConjugation::Gerund), "nici".to_string());
    }

    #[test]
    fn test_hiragana_to_romaji() {
        // I1
        assert_eq!(hiragana_to_romaji("かちゅん"), "kachunn".to_string());
        assert_eq!(hiragana_to_romaji("ちちゅん"), "chichunn".to_string());
        assert_eq!(hiragana_to_romaji("さちゅん"), "sachunn".to_string());
        assert_eq!(hiragana_to_romaji("あっちゅん"), "axtuchunn".to_string());
        // I2
        assert_eq!(hiragana_to_romaji("くうじゅん"), "kuujunn".to_string());
        assert_eq!(hiragana_to_romaji("っゐいじゅん"), "qwiijunn".to_string());
        assert_eq!(hiragana_to_romaji("おおじゅん"), "oojunn".to_string());
        // I3
        assert_eq!(hiragana_to_romaji("たちゅん"), "tachunn".to_string());
        assert_eq!(hiragana_to_romaji("うちゅん"), "uchunn".to_string());
        assert_eq!(hiragana_to_romaji("かちゅん"), "kachunn".to_string());
        // I4
        assert_eq!(hiragana_to_romaji("すらちゅん"), "surachunn".to_string());
        assert_eq!(hiragana_to_romaji("たむちゅん"), "tamuchunn".to_string());
        assert_eq!(hiragana_to_romaji("くちゅん"), "kuchunn".to_string());
        // I5
        assert_eq!(hiragana_to_romaji("くるすん"), "kurusunn".to_string());
        assert_eq!(hiragana_to_romaji("めえすん"), "meesunn".to_string());
        assert_eq!(hiragana_to_romaji("はんすん"), "hannsunn".to_string());
        // I6
        assert_eq!(hiragana_to_romaji("すん"), "sunn".to_string());
        assert_eq!(hiragana_to_romaji("すぃっくゎすん"), "sixtukwasunn".to_string());
        assert_eq!(hiragana_to_romaji("ひっこおすん"), "hixtukoosunn".to_string());
        // I7
        assert_eq!(hiragana_to_romaji("ゆぶん"), "yubunn".to_string());
        assert_eq!(hiragana_to_romaji("とぅぶん"), "tubunn".to_string());
        assert_eq!(hiragana_to_romaji("むすぶん"), "musubunn".to_string());
        // I8
        assert_eq!(hiragana_to_romaji("ゆむん"), "yumunn".to_string());
        assert_eq!(hiragana_to_romaji("ぬむん"), "numunn".to_string());
        assert_eq!(hiragana_to_romaji("あむん"), "amunn".to_string());
        // I9
        assert_eq!(hiragana_to_romaji("にんじゅん"), "ninnjunn".to_string());
        assert_eq!(hiragana_to_romaji("かんじゅん"), "kannjunn".to_string());
        assert_eq!(hiragana_to_romaji("あんじゅん"), "annjunn".to_string());
        // I10
        assert_eq!(hiragana_to_romaji("んんじゅん"), "nnnnjunn".to_string());
        assert_eq!(hiragana_to_romaji("くんじゅん"), "kunnjunn".to_string());
        // I11
        assert_eq!(hiragana_to_romaji("しぬん"), "shinunn".to_string());

    }
}

/*
書く 	ka 	k 	c 	c 	c 	kacuN(書く)、cicuN(聞く)、sacuN(咲く)、ʔaQcuN(歩く)
漕ぐ 	kuu 	g 	z 	z 	z 	kuuzuN(漕ぐ)、ʔwiizuN(泳ぐ)、ʔoozuN(扇ぐ)
立つ 	ta 	t 	c 	c 	Qc 	tacuN(立つ)、ʔucuN(打つ)、kacuN(勝つ)
育つ 	sura 	t 	c 	c 	c 	suracuN(育つ)、tamucuN(保つ)、kucuN(朽ちる)
殺す 	kuru 	s 	s 	s 	c 	kurusuN(殺す)、meesuN(燃やす)、haNsuN(外す)
為る 	‐ 	s 	s 	s 	s 	suN(為る)、siQkwasuN(敷く)、hiQkoosuN(比較する)
呼ぶ 	ju 	b 	b 	b 	r/d 	jubuN(呼ぶ)、tubuN(飛ぶ)、musubuN(結ぶ)
読む 	ju 	m 	m 	m 	r/d 	jumuN(読む)、numuN(飲む)、ʔamuN(編む)
眠る 	niN 	r 	z 	z 	t 	niNzuN(眠る)、kaNzuN(被る)、ʔaNzuN(あぶる)
見る 	NN 	r 	z 	z 	c 	NNzuN(見る)、kuNzuN(括る)
死ぬ 	shi     n       n       n       j       shinuN 死ぬん


Ia1 (tuin,take),(wakain,understand)
Ia2 warain, laugh
Ib1 chiin, don (clothes)
Ib2 chiin, cut it
Ic1 yumariin, can read
Ic2 iriin, put in
IIa1 'wiijun, swim
IIa2 kanjun, don (hats)
IIa3 kunjun, tie up/down
IIb1 kachun, write
IIb2 tachun, stand up
IIc nasun, give birth to
IId yubun, call
IIe yumun, read
IIf shinun, die
III wun, be (animate)
IV
menseen, come, go, be (exalting)
chuun, come
sun, do
an, be (inanimate)
yan, be (copula)
'yun, say
umuin, think
ichun, go
*/


/* 
晴れる はりいん　はりゆん II2
照る　てぃいん　てぃゆん　II2
鳴る ないん　なゆん　II2
渇く かーきいん　かーきゆん　II2

落ちる うてぃいん うてぃゆん　II2

死ぬ　しぬん
*/

/*
biraN
bira
biraa
biree

biin
biiru
biiru
biira
biishi
biishee
biishiga
biikutu

biitaN
biitaru
biitaru
biitara
biitashi
biitashee
biitakutu
biitaga
biitii

bitan
bitaru
bitaru
bitara
bitashi
bitashee
bitashiga
bitakutu
bitaga
bitii
*/


/*

ある
あん、あいびーん

たくさんある
まんどーん、まんどーいびーん

する
すん、さびーん

取る
とぅゆん、とぅやびーん

買う
こーゆん、こーやびーん

笑う
わらゆん、わらいびーん

書く
かちゅん、かちゃびーん

読む
ゆむん、ゆまびーん

生る（？なる、うまる）
なすん、なさびーん

進む
（自動詞） =maN, =di 
ししむん、ししまびーん

煮る
（自動詞） =raN、=ci 
にゆん、にやびーん

建物が建つ
たてぃむん ぬ たちゅん｜たちゃびーん

手が痛む
てぃ ぬ やむん｜やまびーん

出来がよくないです
ふでぃき やん｜やいびーん

当たる
あたゆん、あたやびーん

帰る
けーゆん、けーいびーん

飛ぶ
とぅぶん、とぅばびーん

倒す
とーすん、とーさびーん

飲む
ぬむん、ぬまびーん

戻る
むどぅゆん、むどぅやびーん

待つ
まちゅん、まちゃびーん

知らせる
しらすん、しらさびーん

立つ
たちゅん、たちゃびーん

寝る、眠る
にんじゅん、にんじゃびーん

泳ぐ
「うぃーじゅん、「うぃーじゃびーん

入れる
いりゆん、いりやびーん

着る
ちゆん、ちやびーん

切る
ちゆん、ちやびーん

居る
をぅん、をぅやびーん

行く
いちゅん、いちゃびーん

来る
ちゅーん、ちゃーびーん

見る
んーじゅん、んーじゃびーん

思う
うむゆん、うむやびーん

死ぬ
しぬん、しなびーん

落ちる
うてぃゆん、？

弾く
ふぃちゅん、？ふぃちゃびーん

言う
「ゆん、「やびーん

飼う
ちかなゆん、？

伸びる
ぬぶん、？ぬばびーん

成る
なゆん、？

遊ぶ
あしぶん、？あしばびーん

入る
いゆん、？

知る
しゆん、？

乗る
ぬゆん、？

歩く
あっちゅん、？あっちゃびーん

聞く
ちちゅん、？ちちゃびーん

似る
にゆん、？

呼ぶ
ゆぶん、？ゆばびーん

急ぐ
いすじゅん、？いすじゃびーん

習う
ならゆん、？

植える
「うぇーゆん、？

酔う
うぇゆん、？

置く
うちゅん、？うちゃびーん

使う
ちかゆん、？

歌う
うたゆん、？

奪う
ぼーゆん、？

触る
さーゆん、？

返る
けーゆん、？

立てる
（他動詞） =raN、=ti 
たてぃゆん、？

泣く
（自動詞） =kaN、 =ci 
なちゅん、？なちゃびーん


IV
めんせえん to go, to come, to be
ちゅうん to come　来ゅーん　
むっちちゅん　持っち来ゅーん
すん to do
あん be inanimate
やん　be copula
っゆん　言う
うむいん 思う
いちゅん　行く
*/


/*
ます形　　ビーン形
ビーン 自動詩文　他動詩文
ビラン　否定
ビーミ　肯否質問文
ビーガ　疑問
ビタン　過去形
ビランタン　否定　過去
ビティー　肯否　過去
ビタガ 疑問　過去
 */

// 辞書形 = 派生語幹 + うん/いん/ん = 非過去

    // 未然形
    // 基本語幹+a
    // N（否定） NonPastNegative
    // riiN（可能・受身） Passive
    // suN(使役) Causative
    // a・wa(仮定条件)

    // 条件形I
    // 基本語幹+ee
    // 単独で条件を表す。未然形を使った仮定条件と違って既定条件を表し、文脈によって「〜なら」とも「〜ので」とも訳せる

    // 命令形1
    // 基本語幹+i。
    // 単独で命令を表す。ImperativeI

    // 命令形2
    // 基本語幹+ee。
    // 同じく命令を表す。ImperativeII

    // 連体形1
    // 基本語幹+u。
    // na(な。禁止) Prohibitive
    // ka(まで) Until
    // kazirii(まで・かぎり)などが付く。

    // 連用形
    // 連用語幹+i。
    // ga(〜しに)
    // ciroo(〜しそう) Volitional
    // uusuN(〜できる) Potential
    // busaN(〜したい)などが付く。 Desire

    // 丁寧形
    // 連用語幹+(j)abiiN/ibiiN。第1種動詞では連用語幹+abiiN、第2種動詞では頭語幹+jabiiN、第3種動詞では頭語幹+jabiiNまたは頭語幹+ibiiNとなる[17]。日本語の「〜ます」にあたる。

    // 終止形(現在形)
    // 派生語幹+uN/iN/N。第1種動詞にはuNが、第2種動詞にはiNが、第3種動詞にはNが付く[17]。派生語幹から成る活用形は連体形以下も同様にu/i/○の交替がある。なお、第2種動詞の終止形語尾をiNではなくjuNとしている辞書等もある。iNはjuNの慣用形だが[18]、20世紀後半にはjuNは高齢層で稀に聞かれるだけとなり、ほぼiNに統一された[17]。かつての首里方言ではjuN・iN並存だった。

    // 終止形(現在形)は、未来や、特定の時間に限定されない行為を表すほかに、現在のことを表す点が日本語と異なる。すなわち、日本語で「新聞を読んでいる」と言うところを、現在形でsiNbuN junuNと言う[19]。沖縄方言の現在形は元々「をり」を含んでいる形であり、西日本方言で進行アスペクトを表す「連用形＋よる(をり)」との関係が考えられる。
    // 連体形2
    // 派生語幹+uru/iru/ru。後に名詞が続くほか、baa(〜時)、hazi(〜はず)などの語が付く。また、強調の助詞ru(ぞ)に呼応して係り結びをつくる。

    // 未然形2(ga係り結び形)
    // 派生語幹+ura/ira/ra。疑問の助詞ga(か)に呼応して係り結びをつくる。

    // 条件形2
    // 派生語幹+uraa/iraa/raa。「〜なら」という条件を表す。

    // 準体形
    // 派生語幹+u/i/○。si（の）、siga（〜のだが）、sa（よ）、gutu（理由）、ga（疑問）、mi・i（たずね）などが付く。

    // 過去進行形
    // 派生語幹+utaN/itaN/taN。過去における動作の進行を表す。「〜していた」。

    // 過去進行中止形
    // 派生語幹+uti/iti/ti。疑問の助詞iが付いて、過去における動作進行に対する疑問を表す。「〜していたか」。

    // 過去進行推量形
    // 派生語幹+uteeN/iteeN/teeN。過去における確実な動作進行の推量を表す。「〜していただろう」。

    // 接続形
    // 音便語幹+i。「〜して」の意味を表す。

    // 過去形
    // 音便語幹+aN。過去を表す。「〜した」。

    // 完了形(確証過去形)
    // 音便語幹+eeN。ある動作・行為の結果が現在残っていて、過去に確かにそれが行なわれたことを表す[15][20]。「(今までに)きっと〜している」「〜したに違いない」「〜してある」などと訳される[19]。

    // 継続形
    // 音便語幹+ooN。動作の結果あるいは継続進行を表す[20]。「〜している」などと訳される。

/*

かちゃびーん　書きます
ちちゃびーん　聞きます
さちゃびーん　咲きます
っあっちゃびーん　歩きます

くーじゃびーん　漕ぎます　（こぐ）
っゐーじゃびーん　泳ぎます
っおーじゃびーん　扇ぎます　（あおぐ）

っうちゃびーん　立ちます
うちゃびーん　打つ
かちゃびーん　勝つ

すらちゃびーん 育つ
たむちゃびーん　保つ　たもつ
くちゃびーん　朽ちる　くちる

くるさびーん　殺します
めーさびーん　燃やす
はんさびーん　外す　はずす

さびーん　します
すぃっくゎさびーん　敷きます
ひっこーさびーん　比較します

ゆばびーん　呼びます
とぅばびーん　飛びます
むすばびーん　結びます

ゆまびーん　読む
ぬまびーん　飲む
っあまびーん　編む

にんじゃびーん　眠ります
かんじゃびーん　被る
っあんじゃびーん　あぶる

んんじゃびーん　見る
くんじゃびーん　括る　くくる

うきやびーん　起きます
かまびーん　食べます
ゆまびーん　読みます
さびーん　します
しなびーん　死にます
とぅばびーん　飛びます
はなさびーん　話します
かちゃびーん　書きます
たちゃびーん　立ちます
っゐーじゃびーん　泳ぎます
にんじゃびーん　寝ます
とぅいびーん　　とります

いちゃびーん　行きます
ちゃーびーん　来ます
いやびーん　言います

うきやびらん　起きません
かまびらん　食べません
ゆまびらん　読みません
さびらん　しません
 */
