//! Type keybind converter, phoneme pattern validator, and phoneme sound inventory for Raven_Lang.
//!
//! Rules are derived from `Characters.md`:
//! - Vowel `Type` keybinds are reversed (e.g., `i*` -> `*i` for `/y/`).
//! - Consonant `Type` keybinds are matched greedily (longest match first).
//! - Unmapped Latin letters pass through directly as their IPA equivalents.
//! - Allowed word phoneme patterns: `CV`, `CVV`, `VC`, `VCV`, `VCVV`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhonemeKind {
    Consonant,
    Vowel,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeybindRule {
    pub keybind: &'static str,
    pub ipa: &'static str,
    pub kind: PhonemeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExampleWord {
    pub word: &'static str,
    pub lang: &'static str,
    pub sound_file: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhonemeEntry {
    pub ipa: &'static str,
    pub keybind: Option<&'static str>,
    pub sound_file: &'static str,
    pub examples: [ExampleWord; 5],
}

const fn ex(word: &'static str, lang: &'static str, sound_file: &'static str) -> ExampleWord {
    ExampleWord {
        word,
        lang,
        sound_file,
    }
}

/// Complete vowel inventory from `Characters.md` with phoneme audio & 5 example words each.
pub const VOWEL_INVENTORY: &[PhonemeEntry] = &[
    PhonemeEntry {
        ipa: "y",
        keybind: Some("*i"),
        sound_file: "phonetic_sounds/phonemes/v_y.wav",
        examples: [
            ex("yú (鱼)", "ZH", "phonetic_sounds/words/zh_yu.wav"),
            ex("nǚ (女)", "ZH", "phonetic_sounds/words/zh_nv.wav"),
            ex("lǜ (绿)", "ZH", "phonetic_sounds/words/zh_lv.wav"),
            ex("über", "DE", "phonetic_sounds/words/de_uber.wav"),
            ex("tu", "FR", "phonetic_sounds/words/fr_tu.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʏ",
        keybind: Some("*="),
        sound_file: "phonetic_sounds/phonemes/v_Y.wav",
        examples: [
            ex("fünf", "DE", "phonetic_sounds/words/de_fuenf.wav"),
            ex("Glück", "DE", "phonetic_sounds/words/de_glueck.wav"),
            ex("Hütte", "DE", "phonetic_sounds/words/de_huette.wav"),
            ex("küssen", "DE", "phonetic_sounds/words/de_kuessen.wav"),
            ex("dünn", "DE", "phonetic_sounds/words/de_duenn.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɘ",
        keybind: Some("_e"),
        sound_file: "phonetic_sounds/phonemes/v_schwa_close.wav",
        examples: [
            ex("kõrv", "ET", "phonetic_sounds/words/et_korv.wav"),
            ex("õun", "ET", "phonetic_sounds/words/et_oun.wav"),
            ex("sõna", "ET", "phonetic_sounds/words/et_sona.wav"),
            ex("õhtu", "ET", "phonetic_sounds/words/et_ohtu.wav"),
            ex("mõte", "ET", "phonetic_sounds/words/et_mote.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "i",
        keybind: Some("i"),
        sound_file: "phonetic_sounds/phonemes/v_i.wav",
        examples: [
            ex("see", "EN", "phonetic_sounds/words/en_see.wav"),
            ex("green", "EN", "phonetic_sounds/words/en_green.wav"),
            ex("machine", "EN", "phonetic_sounds/words/en_machine.wav"),
            ex("people", "EN", "phonetic_sounds/words/en_people.wav"),
            ex("key", "EN", "phonetic_sounds/words/en_key.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɪ",
        keybind: Some("="),
        sound_file: "phonetic_sounds/phonemes/v_I.wav",
        examples: [
            ex("sit", "EN", "phonetic_sounds/words/en_sit.wav"),
            ex("fish", "EN", "phonetic_sounds/words/en_fish.wav"),
            ex("win", "EN", "phonetic_sounds/words/en_win.wav"),
            ex("myth", "EN", "phonetic_sounds/words/en_myth.wav"),
            ex("ship", "EN", "phonetic_sounds/words/en_ship.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɛ",
        keybind: Some("|e"),
        sound_file: "phonetic_sounds/phonemes/v_E.wav",
        examples: [
            ex("bed", "EN", "phonetic_sounds/words/en_bed.wav"),
            ex("head", "EN", "phonetic_sounds/words/en_head.wav"),
            ex("said", "EN", "phonetic_sounds/words/en_said.wav"),
            ex("friend", "EN", "phonetic_sounds/words/en_friend.wav"),
            ex("many", "EN", "phonetic_sounds/words/en_many.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "æ",
        keybind: Some("||e"),
        sound_file: "phonetic_sounds/phonemes/v_ae.wav",
        examples: [
            ex("cat", "EN", "phonetic_sounds/words/en_cat.wav"),
            ex("hand", "EN", "phonetic_sounds/words/en_hand.wav"),
            ex("black", "EN", "phonetic_sounds/words/en_black.wav"),
            ex("apple", "EN", "phonetic_sounds/words/en_apple.wav"),
            ex("trap", "EN", "phonetic_sounds/words/en_trap.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʊ",
        keybind: Some("|u"),
        sound_file: "phonetic_sounds/phonemes/v_U.wav",
        examples: [
            ex("book", "EN", "phonetic_sounds/words/en_book.wav"),
            ex("good", "EN", "phonetic_sounds/words/en_good.wav"),
            ex("put", "EN", "phonetic_sounds/words/en_put.wav"),
            ex("could", "EN", "phonetic_sounds/words/en_could.wav"),
            ex("woman", "EN", "phonetic_sounds/words/en_woman.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "u",
        keybind: Some("u"),
        sound_file: "phonetic_sounds/phonemes/v_u.wav",
        examples: [
            ex("food", "EN", "phonetic_sounds/words/en_food.wav"),
            ex("blue", "EN", "phonetic_sounds/words/en_blue.wav"),
            ex("group", "EN", "phonetic_sounds/words/en_group.wav"),
            ex("move", "EN", "phonetic_sounds/words/en_move.wav"),
            ex("goose", "EN", "phonetic_sounds/words/en_goose.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʌ",
        keybind: Some("|_a"),
        sound_file: "phonetic_sounds/phonemes/v_V.wav",
        examples: [
            ex("cup", "EN", "phonetic_sounds/words/en_cup.wav"),
            ex("luck", "EN", "phonetic_sounds/words/en_luck.wav"),
            ex("blood", "EN", "phonetic_sounds/words/en_blood.wav"),
            ex("son", "EN", "phonetic_sounds/words/en_son.wav"),
            ex("enough", "EN", "phonetic_sounds/words/en_enough.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ə",
        keybind: Some("/|e"),
        sound_file: "phonetic_sounds/phonemes/v_schwa.wav",
        examples: [
            ex("about", "EN", "phonetic_sounds/words/en_about.wav"),
            ex("sofa", "EN", "phonetic_sounds/words/en_sofa.wav"),
            ex("support", "EN", "phonetic_sounds/words/en_support.wav"),
            ex("problem", "EN", "phonetic_sounds/words/en_problem.wav"),
            ex("banana", "EN", "phonetic_sounds/words/en_banana.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɑ",
        keybind: Some("_a"),
        sound_file: "phonetic_sounds/phonemes/v_A.wav",
        examples: [
            ex("father", "EN", "phonetic_sounds/words/en_father.wav"),
            ex("cot", "EN", "phonetic_sounds/words/en_cot.wav"),
            ex("rock", "EN", "phonetic_sounds/words/en_rock.wav"),
            ex("job", "EN", "phonetic_sounds/words/en_job.wav"),
            ex("not", "EN", "phonetic_sounds/words/en_not.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɔ",
        keybind: Some("|o"),
        sound_file: "phonetic_sounds/phonemes/v_O.wav",
        examples: [
            ex("thought", "EN", "phonetic_sounds/words/en_thought.wav"),
            ex("law", "EN", "phonetic_sounds/words/en_law.wav"),
            ex("caught", "EN", "phonetic_sounds/words/en_caught.wav"),
            ex("dawn", "EN", "phonetic_sounds/words/en_dawn.wav"),
            ex("talk", "EN", "phonetic_sounds/words/en_talk.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "e",
        keybind: Some("e"),
        sound_file: "phonetic_sounds/phonemes/v_e.wav",
        examples: [
            ex("enak", "ID", "phonetic_sounds/words/id_enak.wav"),
            ex("ekor", "ID", "phonetic_sounds/words/id_ekor.wav"),
            ex("beda", "ID", "phonetic_sounds/words/id_beda.wav"),
            ex("sate", "ID", "phonetic_sounds/words/id_sate.wav"),
            ex("lele", "ID", "phonetic_sounds/words/id_lele.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "a",
        keybind: Some("a"),
        sound_file: "phonetic_sounds/phonemes/v_a.wav",
        examples: [
            ex("anak", "ID", "phonetic_sounds/words/id_anak.wav"),
            ex("makan", "ID", "phonetic_sounds/words/id_makan.wav"),
            ex("mata", "ID", "phonetic_sounds/words/id_mata.wav"),
            ex("jalan", "ID", "phonetic_sounds/words/id_jalan.wav"),
            ex("panas", "ID", "phonetic_sounds/words/id_panas.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɯ",
        keybind: Some("*u"),
        sound_file: "phonetic_sounds/phonemes/v_M.wav",
        examples: [
            ex("umi (海)", "JA", "phonetic_sounds/words/ja_umi.wav"),
            ex("tsuki (月)", "JA", "phonetic_sounds/words/ja_tsuki.wav"),
            ex("fune (船)", "JA", "phonetic_sounds/words/ja_fune.wav"),
            ex("kutsu (靴)", "JA", "phonetic_sounds/words/ja_kutsu.wav"),
            ex("kuruma (車)", "JA", "phonetic_sounds/words/ja_kuruma.wav"),
        ],
    },
];

/// Complete consonant inventory from `Characters.md` with phoneme audio & 5 example words each.
pub const CONSONANT_INVENTORY: &[PhonemeEntry] = &[
    PhonemeEntry {
        ipa: "q",
        keybind: Some("K"),
        sound_file: "phonetic_sounds/phonemes/c_q.wav",
        examples: [
            ex("qalam (قلم)", "AR", "phonetic_sounds/words/ar_qalam.wav"),
            ex("qamar (قمر)", "AR", "phonetic_sounds/words/ar_qamar.wav"),
            ex("qalb (قلب)", "AR", "phonetic_sounds/words/ar_qalb.wav"),
            ex("quwwa (قوة)", "AR", "phonetic_sounds/words/ar_quwwa.wav"),
            ex("waqt (وقت)", "AR", "phonetic_sounds/words/ar_waqt.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɢ",
        keybind: Some("G"),
        sound_file: "phonetic_sounds/phonemes/c_G_cap.wav",
        examples: [
            ex("ghalib (غالب)", "UR", "phonetic_sounds/words/ur_ghalib.wav"),
            ex("ghazal (غزل)", "UR", "phonetic_sounds/words/ur_ghazal.wav"),
            ex("gham (غم)", "UR", "phonetic_sounds/words/ur_gham.wav"),
            ex("ghor (غور)", "UR", "phonetic_sounds/words/ur_ghor.wav"),
            ex("bagh (باغ)", "UR", "phonetic_sounds/words/ur_bagh.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɸ",
        keybind: Some("F"),
        sound_file: "phonetic_sounds/phonemes/c_F.wav",
        examples: [
            ex("fuji (富士)", "JA", "phonetic_sounds/words/ja_fuji.wav"),
            ex("fune (船)", "JA", "phonetic_sounds/words/ja_fune.wav"),
            ex("fuyu (冬)", "JA", "phonetic_sounds/words/ja_fuyu.wav"),
            ex("fuku (服)", "JA", "phonetic_sounds/words/ja_fuku.wav"),
            ex("futatsu (二つ)", "JA", "phonetic_sounds/words/ja_futatsu.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "β",
        keybind: Some("V"),
        sound_file: "phonetic_sounds/phonemes/c_beta.wav",
        examples: [
            ex("lobo", "ES", "phonetic_sounds/words/es_lobo.wav"),
            ex("saber", "ES", "phonetic_sounds/words/es_saber.wav"),
            ex("nube", "ES", "phonetic_sounds/words/es_nube.wav"),
            ex("beber", "ES", "phonetic_sounds/words/es_beber.wav"),
            ex("uva", "ES", "phonetic_sounds/words/es_uva.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "χ",
        keybind: Some("Kh"),
        sound_file: "phonetic_sounds/phonemes/c_chi.wav",
        examples: [
            ex("Bach", "DE", "phonetic_sounds/words/de_bach.wav"),
            ex("Buch", "DE", "phonetic_sounds/words/de_buch.wav"),
            ex("Nacht", "DE", "phonetic_sounds/words/de_nacht.wav"),
            ex("Dach", "DE", "phonetic_sounds/words/de_dach.wav"),
            ex("Loch", "DE", "phonetic_sounds/words/de_loch.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʁ",
        keybind: Some("Gh"),
        sound_file: "phonetic_sounds/phonemes/c_R_inv.wav",
        examples: [
            ex("rouge", "FR", "phonetic_sounds/words/fr_rouge.wav"),
            ex("rue", "FR", "phonetic_sounds/words/fr_rue.wav"),
            ex("Paris", "FR", "phonetic_sounds/words/fr_paris.wav"),
            ex("roi", "FR", "phonetic_sounds/words/fr_roi.wav"),
            ex("rose", "FR", "phonetic_sounds/words/fr_rose.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʋ",
        keybind: Some("wv"),
        sound_file: "phonetic_sounds/phonemes/c_v_app.wav",
        examples: [
            ex("water", "NL", "phonetic_sounds/words/nl_water.wav"),
            ex("waar", "NL", "phonetic_sounds/words/nl_waar.wav"),
            ex("wat", "NL", "phonetic_sounds/words/nl_wat.wav"),
            ex("wel", "NL", "phonetic_sounds/words/nl_wel.wav"),
            ex("winter", "NL", "phonetic_sounds/words/nl_winter.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "p",
        keybind: Some("p"),
        sound_file: "phonetic_sounds/phonemes/c_p.wav",
        examples: [
            ex("pen", "EN", "phonetic_sounds/words/en_pen.wav"),
            ex("paper", "EN", "phonetic_sounds/words/en_paper.wav"),
            ex("apple", "EN", "phonetic_sounds/words/en_apple.wav"),
            ex("happy", "EN", "phonetic_sounds/words/en_happy.wav"),
            ex("stop", "EN", "phonetic_sounds/words/en_stop.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "b",
        keybind: Some("b"),
        sound_file: "phonetic_sounds/phonemes/c_b.wav",
        examples: [
            ex("book", "EN", "phonetic_sounds/words/en_book.wav"),
            ex("ball", "EN", "phonetic_sounds/words/en_ball.wav"),
            ex("table", "EN", "phonetic_sounds/words/en_table.wav"),
            ex("rubber", "EN", "phonetic_sounds/words/en_rubber.wav"),
            ex("job", "EN", "phonetic_sounds/words/en_job.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "t",
        keybind: Some("t"),
        sound_file: "phonetic_sounds/phonemes/c_t.wav",
        examples: [
            ex("time", "EN", "phonetic_sounds/words/en_time.wav"),
            ex("tea", "EN", "phonetic_sounds/words/en_tea.wav"),
            ex("attack", "EN", "phonetic_sounds/words/en_attack.wav"),
            ex("water", "EN", "phonetic_sounds/words/en_water.wav"),
            ex("cat", "EN", "phonetic_sounds/words/en_cat.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "d",
        keybind: Some("d"),
        sound_file: "phonetic_sounds/phonemes/c_d.wav",
        examples: [
            ex("dog", "EN", "phonetic_sounds/words/en_dog.wav"),
            ex("day", "EN", "phonetic_sounds/words/en_day.wav"),
            ex("ladder", "EN", "phonetic_sounds/words/en_ladder.wav"),
            ex("idea", "EN", "phonetic_sounds/words/en_idea.wav"),
            ex("bad", "EN", "phonetic_sounds/words/en_bad.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "k",
        keybind: Some("k"),
        sound_file: "phonetic_sounds/phonemes/c_k.wav",
        examples: [
            ex("cat", "EN", "phonetic_sounds/words/en_cat.wav"),
            ex("key", "EN", "phonetic_sounds/words/en_key.wav"),
            ex("back", "EN", "phonetic_sounds/words/en_back.wav"),
            ex("kite", "EN", "phonetic_sounds/words/en_kite.wav"),
            ex("cool", "EN", "phonetic_sounds/words/en_cool.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "g",
        keybind: Some("g"),
        sound_file: "phonetic_sounds/phonemes/c_g.wav",
        examples: [
            ex("go", "EN", "phonetic_sounds/words/en_go.wav"),
            ex("get", "EN", "phonetic_sounds/words/en_get.wav"),
            ex("bigger", "EN", "phonetic_sounds/words/en_bigger.wav"),
            ex("again", "EN", "phonetic_sounds/words/en_again.wav"),
            ex("dog", "EN", "phonetic_sounds/words/en_dog.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "m",
        keybind: Some("m"),
        sound_file: "phonetic_sounds/phonemes/c_m.wav",
        examples: [
            ex("man", "EN", "phonetic_sounds/words/en_man.wav"),
            ex("moon", "EN", "phonetic_sounds/words/en_moon.wav"),
            ex("summer", "EN", "phonetic_sounds/words/en_summer.wav"),
            ex("time", "EN", "phonetic_sounds/words/en_time.wav"),
            ex("room", "EN", "phonetic_sounds/words/en_room.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "n",
        keybind: Some("n"),
        sound_file: "phonetic_sounds/phonemes/c_n.wav",
        examples: [
            ex("nose", "EN", "phonetic_sounds/words/en_nose.wav"),
            ex("name", "EN", "phonetic_sounds/words/en_name.wav"),
            ex("winner", "EN", "phonetic_sounds/words/en_winner.wav"),
            ex("ten", "EN", "phonetic_sounds/words/en_ten.wav"),
            ex("sun", "EN", "phonetic_sounds/words/en_sun.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ŋ",
        keybind: Some("ng"),
        sound_file: "phonetic_sounds/phonemes/c_ng.wav",
        examples: [
            ex("sing", "EN", "phonetic_sounds/words/en_sing.wav"),
            ex("long", "EN", "phonetic_sounds/words/en_long.wav"),
            ex("singer", "EN", "phonetic_sounds/words/en_singer.wav"),
            ex("ring", "EN", "phonetic_sounds/words/en_ring.wav"),
            ex("tongue", "EN", "phonetic_sounds/words/en_tongue.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "f",
        keybind: Some("f"),
        sound_file: "phonetic_sounds/phonemes/c_f.wav",
        examples: [
            ex("fish", "EN", "phonetic_sounds/words/en_fish.wav"),
            ex("food", "EN", "phonetic_sounds/words/en_food.wav"),
            ex("off", "EN", "phonetic_sounds/words/en_off.wav"),
            ex("leaf", "EN", "phonetic_sounds/words/en_leaf.wav"),
            ex("laugh", "EN", "phonetic_sounds/words/en_laugh.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "v",
        keybind: Some("v"),
        sound_file: "phonetic_sounds/phonemes/c_v.wav",
        examples: [
            ex("voice", "EN", "phonetic_sounds/words/en_voice.wav"),
            ex("very", "EN", "phonetic_sounds/words/en_very.wav"),
            ex("never", "EN", "phonetic_sounds/words/en_never.wav"),
            ex("move", "EN", "phonetic_sounds/words/en_move.wav"),
            ex("love", "EN", "phonetic_sounds/words/en_love.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "θ",
        keybind: Some("th"),
        sound_file: "phonetic_sounds/phonemes/c_th.wav",
        examples: [
            ex("think", "EN", "phonetic_sounds/words/en_think.wav"),
            ex("three", "EN", "phonetic_sounds/words/en_three.wav"),
            ex("bath", "EN", "phonetic_sounds/words/en_bath.wav"),
            ex("author", "EN", "phonetic_sounds/words/en_author.wav"),
            ex("teeth", "EN", "phonetic_sounds/words/en_teeth.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ð",
        keybind: Some("dh"),
        sound_file: "phonetic_sounds/phonemes/c_dh.wav",
        examples: [
            ex("this", "EN", "phonetic_sounds/words/en_this.wav"),
            ex("that", "EN", "phonetic_sounds/words/en_that.wav"),
            ex("mother", "EN", "phonetic_sounds/words/en_mother.wav"),
            ex("weather", "EN", "phonetic_sounds/words/en_weather.wav"),
            ex("breathe", "EN", "phonetic_sounds/words/en_breathe.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "s",
        keybind: Some("s"),
        sound_file: "phonetic_sounds/phonemes/c_s.wav",
        examples: [
            ex("sun", "EN", "phonetic_sounds/words/en_sun.wav"),
            ex("see", "EN", "phonetic_sounds/words/en_see.wav"),
            ex("class", "EN", "phonetic_sounds/words/en_class.wav"),
            ex("miss", "EN", "phonetic_sounds/words/en_miss.wav"),
            ex("basic", "EN", "phonetic_sounds/words/en_basic.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "z",
        keybind: Some("z"),
        sound_file: "phonetic_sounds/phonemes/c_z.wav",
        examples: [
            ex("zoo", "EN", "phonetic_sounds/words/en_zoo.wav"),
            ex("zebra", "EN", "phonetic_sounds/words/en_zebra.wav"),
            ex("rose", "EN", "phonetic_sounds/words/en_rose.wav"),
            ex("easy", "EN", "phonetic_sounds/words/en_easy.wav"),
            ex("buzz", "EN", "phonetic_sounds/words/en_buzz.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʃ",
        keybind: Some("sh"),
        sound_file: "phonetic_sounds/phonemes/c_sh.wav",
        examples: [
            ex("ship", "EN", "phonetic_sounds/words/en_ship.wav"),
            ex("she", "EN", "phonetic_sounds/words/en_she.wav"),
            ex("wash", "EN", "phonetic_sounds/words/en_wash.wav"),
            ex("nation", "EN", "phonetic_sounds/words/en_nation.wav"),
            ex("fish", "EN", "phonetic_sounds/words/en_fish.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ʒ",
        keybind: Some("zh"),
        sound_file: "phonetic_sounds/phonemes/c_zh.wav",
        examples: [
            ex("vision", "EN", "phonetic_sounds/words/en_vision.wav"),
            ex("measure", "EN", "phonetic_sounds/words/en_measure.wav"),
            ex("usual", "EN", "phonetic_sounds/words/en_usual.wav"),
            ex("beige", "EN", "phonetic_sounds/words/en_beige.wav"),
            ex("garage", "EN", "phonetic_sounds/words/en_garage.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "h",
        keybind: Some("h"),
        sound_file: "phonetic_sounds/phonemes/c_h.wav",
        examples: [
            ex("hat", "EN", "phonetic_sounds/words/en_hat.wav"),
            ex("house", "EN", "phonetic_sounds/words/en_house.wav"),
            ex("hello", "EN", "phonetic_sounds/words/en_hello.wav"),
            ex("behind", "EN", "phonetic_sounds/words/en_behind.wav"),
            ex("hope", "EN", "phonetic_sounds/words/en_hope.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɹ",
        keybind: Some("r@"),
        sound_file: "phonetic_sounds/phonemes/c_r_app.wav",
        examples: [
            ex("red", "EN", "phonetic_sounds/words/en_red.wav"),
            ex("right", "EN", "phonetic_sounds/words/en_right.wav"),
            ex("around", "EN", "phonetic_sounds/words/en_around.wav"),
            ex("sorry", "EN", "phonetic_sounds/words/en_sorry.wav"),
            ex("car", "EN", "phonetic_sounds/words/en_car.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "j",
        keybind: Some("y"),
        sound_file: "phonetic_sounds/phonemes/c_j.wav",
        examples: [
            ex("yes", "EN", "phonetic_sounds/words/en_yes.wav"),
            ex("yellow", "EN", "phonetic_sounds/words/en_yellow.wav"),
            ex("use", "EN", "phonetic_sounds/words/en_use.wav"),
            ex("music", "EN", "phonetic_sounds/words/en_music.wav"),
            ex("beyond", "EN", "phonetic_sounds/words/en_beyond.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "w",
        keybind: Some("w"),
        sound_file: "phonetic_sounds/phonemes/c_w.wav",
        examples: [
            ex("we", "EN", "phonetic_sounds/words/en_we.wav"),
            ex("water", "EN", "phonetic_sounds/words/en_water.wav"),
            ex("away", "EN", "phonetic_sounds/words/en_away.wav"),
            ex("what", "EN", "phonetic_sounds/words/en_what.wav"),
            ex("win", "EN", "phonetic_sounds/words/en_win.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "l",
        keybind: Some("l"),
        sound_file: "phonetic_sounds/phonemes/c_l.wav",
        examples: [
            ex("light", "EN", "phonetic_sounds/words/en_light.wav"),
            ex("love", "EN", "phonetic_sounds/words/en_love.wav"),
            ex("allow", "EN", "phonetic_sounds/words/en_allow.wav"),
            ex("yellow", "EN", "phonetic_sounds/words/en_yellow.wav"),
            ex("feel", "EN", "phonetic_sounds/words/en_feel.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "tʃ",
        keybind: Some("ch"),
        sound_file: "phonetic_sounds/phonemes/c_tsh.wav",
        examples: [
            ex("chair", "EN", "phonetic_sounds/words/en_chair.wav"),
            ex("child", "EN", "phonetic_sounds/words/en_child.wav"),
            ex("teacher", "EN", "phonetic_sounds/words/en_teacher.wav"),
            ex("nature", "EN", "phonetic_sounds/words/en_nature.wav"),
            ex("watch", "EN", "phonetic_sounds/words/en_watch.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "dʒ",
        keybind: Some("jh"),
        sound_file: "phonetic_sounds/phonemes/c_dzh.wav",
        examples: [
            ex("judge", "EN", "phonetic_sounds/words/en_judge.wav"),
            ex("giant", "EN", "phonetic_sounds/words/en_giant.wav"),
            ex("age", "EN", "phonetic_sounds/words/en_age.wav"),
            ex("magic", "EN", "phonetic_sounds/words/en_magic.wav"),
            ex("bridge", "EN", "phonetic_sounds/words/en_bridge.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɲ",
        keybind: Some("ny"),
        sound_file: "phonetic_sounds/phonemes/c_ny.wav",
        examples: [
            ex("nyanyi", "ID", "phonetic_sounds/words/id_nyanyi.wav"),
            ex("banyak", "ID", "phonetic_sounds/words/id_banyak.wav"),
            ex("punya", "ID", "phonetic_sounds/words/id_punya.wav"),
            ex("minyak", "ID", "phonetic_sounds/words/id_minyak.wav"),
            ex("senyum", "ID", "phonetic_sounds/words/id_senyum.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "r",
        keybind: Some("r"),
        sound_file: "phonetic_sounds/phonemes/c_r.wav",
        examples: [
            ex("rumah", "ID", "phonetic_sounds/words/id_rumah.wav"),
            ex("ramai", "ID", "phonetic_sounds/words/id_ramai.wav"),
            ex("baru", "ID", "phonetic_sounds/words/id_baru.wav"),
            ex("merah", "ID", "phonetic_sounds/words/id_merah.wav"),
            ex("berani", "ID", "phonetic_sounds/words/id_berani.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "x",
        keybind: Some("x"),
        sound_file: "phonetic_sounds/phonemes/c_x.wav",
        examples: [
            ex("khas", "ID", "phonetic_sounds/words/id_khas.wav"),
            ex("akhir", "ID", "phonetic_sounds/words/id_akhir.wav"),
            ex("khusus", "ID", "phonetic_sounds/words/id_khusus.wav"),
            ex("makhluk", "ID", "phonetic_sounds/words/id_makhluk.wav"),
            ex("ikhlas", "ID", "phonetic_sounds/words/id_ikhlas.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɾ",
        keybind: Some("r!"),
        sound_file: "phonetic_sounds/phonemes/c_flap.wav",
        examples: [
            ex("ramen (ラーメン)", "JA", "phonetic_sounds/words/ja_ramen.wav"),
            ex("ringo (りんご)", "JA", "phonetic_sounds/words/ja_ringo.wav"),
            ex("saru (猿)", "JA", "phonetic_sounds/words/ja_saru.wav"),
            ex("kuruma (車)", "JA", "phonetic_sounds/words/ja_kuruma.wav"),
            ex("sora (空)", "JA", "phonetic_sounds/words/ja_sora.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "ɕ",
        keybind: Some("C"),
        sound_file: "phonetic_sounds/phonemes/c_C.wav",
        examples: [
            ex("shio (塩)", "JA", "phonetic_sounds/words/ja_shio.wav"),
            ex("shiro (白)", "JA", "phonetic_sounds/words/ja_shiro.wav"),
            ex("shashin (写真)", "JA", "phonetic_sounds/words/ja_shashin.wav"),
            ex("ashita (明日)", "JA", "phonetic_sounds/words/ja_ashita.wav"),
            ex("densha (電車)", "JA", "phonetic_sounds/words/ja_densha.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "tɕ",
        keybind: Some("Ch"),
        sound_file: "phonetic_sounds/phonemes/c_tC.wav",
        examples: [
            ex("chichi (父)", "JA", "phonetic_sounds/words/ja_chichi.wav"),
            ex("chizu (地図)", "JA", "phonetic_sounds/words/ja_chizu.wav"),
            ex("chikai (近い)", "JA", "phonetic_sounds/words/ja_chikai.wav"),
            ex("machi (町)", "JA", "phonetic_sounds/words/ja_machi.wav"),
            ex("kuchi (口)", "JA", "phonetic_sounds/words/ja_kuchi.wav"),
        ],
    },
    PhonemeEntry {
        ipa: "dʑ",
        keybind: None,
        sound_file: "phonetic_sounds/phonemes/c_dZ.wav",
        examples: [
            ex("jikan (時間)", "JA", "phonetic_sounds/words/ja_jikan.wav"),
            ex("jibun (自分)", "JA", "phonetic_sounds/words/ja_jibun.wav"),
            ex("jisho (辞書)", "JA", "phonetic_sounds/words/ja_jisho.wav"),
            ex("daijoubu (大丈夫)", "JA", "phonetic_sounds/words/ja_daijoubu.wav"),
            ex("kaji (火事)", "JA", "phonetic_sounds/words/ja_kaji.wav"),
        ],
    },
];

/// Vowel keybind rules (Type column reversed per `@NOTE` in `Characters.md`).
pub const VOWEL_RULES: &[KeybindRule] = &[
    KeybindRule { keybind: "||e", ipa: "æ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "|_a", ipa: "ʌ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "/|e", ipa: "ə", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "*i", ipa: "y", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "*=", ipa: "ʏ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "_e", ipa: "ɘ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "|e", ipa: "ɛ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "|u", ipa: "ʊ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "_a", ipa: "ɑ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "|o", ipa: "ɔ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "*u", ipa: "ɯ", kind: PhonemeKind::Vowel },
    KeybindRule { keybind: "=", ipa: "ɪ", kind: PhonemeKind::Vowel },
];

/// Consonant keybind rules from `Characters.md`.
pub const CONSONANT_RULES: &[KeybindRule] = &[
    KeybindRule { keybind: "ng", ipa: "ŋ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "th", ipa: "θ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "dh", ipa: "ð", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "sh", ipa: "ʃ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "zh", ipa: "ʒ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "r@", ipa: "ɹ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "ch", ipa: "tʃ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "jh", ipa: "dʒ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "ny", ipa: "ɲ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "r!", ipa: "ɾ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "Ch", ipa: "tɕ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "Kh", ipa: "χ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "Gh", ipa: "ʁ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "wv", ipa: "ʋ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "y", ipa: "j", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "x", ipa: "x", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "C", ipa: "ɕ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "K", ipa: "q", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "G", ipa: "ɢ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "F", ipa: "ɸ", kind: PhonemeKind::Consonant },
    KeybindRule { keybind: "V", ipa: "β", kind: PhonemeKind::Consonant },
];

/// Allowed syllable/word phoneme patterns from `Characters.md`.
#[allow(dead_code)]
pub const ALLOWED_PATTERNS: &[&str] = &["CV", "CVV", "VC", "VCV", "VCVV"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertedUnit {
    pub ipa: String,
    pub kind: PhonemeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct TokenAnalysis {
    pub input: String,
    pub output: String,
    pub pattern: String,
    pub is_valid: bool,
}

fn classify_passthrough_char(ch: char) -> PhonemeKind {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ʏ' | 'ɘ' | 'ɪ' | 'ɛ' | 'æ' | 'ʊ' | 'ɑ' | 'ʌ'
        | 'ə' | 'ɔ' | 'ɯ' => PhonemeKind::Vowel,
        'p' | 'b' | 't' | 'd' | 'k' | 'g' | 'm' | 'n' | 'ŋ' | 'f' | 'v' | 'θ' | 'ð' | 's'
        | 'z' | 'ʃ' | 'ʒ' | 'h' | 'ɹ' | 'j' | 'w' | 'l' | 'ɲ' | 'r' | 'x' | 'ɾ' | 'ɕ' | 'q'
        | 'ɢ' | 'ɸ' | 'β' | 'χ' | 'ʁ' | 'ʋ' => PhonemeKind::Consonant,
        _ => PhonemeKind::Other,
    }
}

/// All rules sorted by keybind length descending for greedy longest-match.
fn all_rules_longest_first() -> Vec<&'static KeybindRule> {
    let mut rules: Vec<&'static KeybindRule> =
        VOWEL_RULES.iter().chain(CONSONANT_RULES.iter()).collect();
    rules.sort_by(|a, b| b.keybind.len().cmp(&a.keybind.len()));
    rules
}

/// Converts Type keybind text into IPA units using longest-match.
pub fn convert_to_units(input: &str) -> Vec<ConvertedUnit> {
    let rules = all_rules_longest_first();
    let mut units = Vec::new();
    let mut idx = 0;

    while idx < input.len() {
        let rest = &input[idx..];
        if let Some(rule) = rules.iter().find(|r| rest.starts_with(r.keybind)) {
            units.push(ConvertedUnit {
                ipa: rule.ipa.to_string(),
                kind: rule.kind,
            });
            idx += rule.keybind.len();
        } else if let Some(ch) = rest.chars().next() {
            units.push(ConvertedUnit {
                ipa: ch.to_string(),
                kind: classify_passthrough_char(ch),
            });
            idx += ch.len_utf8();
        } else {
            break;
        }
    }

    units
}

/// Converts Type keybind text directly into the output IPA string.
pub fn convert_type_to_ipa(input: &str) -> String {
    convert_to_units(input)
        .into_iter()
        .map(|u| u.ipa)
        .collect()
}

/// Checks if a phoneme pattern string matches `CV | CVV | VC | VCV | VCVV`.
#[allow(dead_code)]
pub fn is_valid_phoneme_pattern(pattern: &str) -> bool {
    ALLOWED_PATTERNS.contains(&pattern)
}

/// Analyzes whitespace-separated tokens for both IPA conversion and phoneme pattern validity.
#[allow(dead_code)]
pub fn analyze_text(input: &str) -> Vec<TokenAnalysis> {
    input
        .split_whitespace()
        .map(|token| {
            let units = convert_to_units(token);
            let mut output = String::new();
            let mut pattern = String::new();
            let mut has_other = false;

            for unit in units {
                output.push_str(&unit.ipa);
                match unit.kind {
                    PhonemeKind::Consonant => pattern.push('C'),
                    PhonemeKind::Vowel => pattern.push('V'),
                    PhonemeKind::Other => {
                        pattern.push('?');
                        has_other = true;
                    }
                }
            }

            let is_valid = !has_other && is_valid_phoneme_pattern(&pattern);
            TokenAnalysis {
                input: token.to_string(),
                output,
                pattern,
                is_valid,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_all_vowel_keybinds_reversed() {
        assert_eq!(convert_type_to_ipa("*i"), "y");
        assert_eq!(convert_type_to_ipa("*="), "ʏ");
        assert_eq!(convert_type_to_ipa("_e"), "ɘ");
        assert_eq!(convert_type_to_ipa("="), "ɪ");
        assert_eq!(convert_type_to_ipa("|e"), "ɛ");
        assert_eq!(convert_type_to_ipa("||e"), "æ");
        assert_eq!(convert_type_to_ipa("|u"), "ʊ");
        assert_eq!(convert_type_to_ipa("_a"), "ɑ");
        assert_eq!(convert_type_to_ipa("|_a"), "ʌ");
        assert_eq!(convert_type_to_ipa("/|e"), "ə");
        assert_eq!(convert_type_to_ipa("|o"), "ɔ");
        assert_eq!(convert_type_to_ipa("*u"), "ɯ");
    }

    #[test]
    fn converts_all_consonant_keybinds() {
        assert_eq!(convert_type_to_ipa("ng"), "ŋ");
        assert_eq!(convert_type_to_ipa("th"), "θ");
        assert_eq!(convert_type_to_ipa("dh"), "ð");
        assert_eq!(convert_type_to_ipa("sh"), "ʃ");
        assert_eq!(convert_type_to_ipa("zh"), "ʒ");
        assert_eq!(convert_type_to_ipa("r@"), "ɹ");
        assert_eq!(convert_type_to_ipa("y"), "j");
        assert_eq!(convert_type_to_ipa("ch"), "tʃ");
        assert_eq!(convert_type_to_ipa("jh"), "dʒ");
        assert_eq!(convert_type_to_ipa("ny"), "ɲ");
        assert_eq!(convert_type_to_ipa("x"), "x");
        assert_eq!(convert_type_to_ipa("r!"), "ɾ");
        assert_eq!(convert_type_to_ipa("C"), "ɕ");
        assert_eq!(convert_type_to_ipa("Ch"), "tɕ");
        assert_eq!(convert_type_to_ipa("K"), "q");
        assert_eq!(convert_type_to_ipa("G"), "ɢ");
        assert_eq!(convert_type_to_ipa("F"), "ɸ");
        assert_eq!(convert_type_to_ipa("V"), "β");
        assert_eq!(convert_type_to_ipa("Kh"), "χ");
        assert_eq!(convert_type_to_ipa("Gh"), "ʁ");
        assert_eq!(convert_type_to_ipa("wv"), "ʋ");
    }

    #[test]
    fn longest_match_disambiguates_prefixes() {
        assert_eq!(convert_type_to_ipa("Cha"), "tɕa");
        assert_eq!(convert_type_to_ipa("Ca"), "ɕa");
        assert_eq!(convert_type_to_ipa("Kha"), "χa");
        assert_eq!(convert_type_to_ipa("Ka"), "qa");
        assert_eq!(convert_type_to_ipa("Gha"), "ʁa");
        assert_eq!(convert_type_to_ipa("Ga"), "ɢa");
        assert_eq!(convert_type_to_ipa("r@a"), "ɹa");
        assert_eq!(convert_type_to_ipa("r!a"), "ɾa");
        assert_eq!(convert_type_to_ipa("ra"), "ra");
    }

    #[test]
    fn validates_phoneme_patterns() {
        let tokens = analyze_text("pa p*ia ak aka ak*i|u ch*i|e");
        assert_eq!(tokens.len(), 6);

        assert_eq!(tokens[0].pattern, "CV");
        assert!(tokens[0].is_valid);

        assert_eq!(tokens[1].pattern, "CVV");
        assert!(tokens[1].is_valid);

        assert_eq!(tokens[2].pattern, "VC");
        assert!(tokens[2].is_valid);

        assert_eq!(tokens[3].pattern, "VCV");
        assert!(tokens[3].is_valid);

        assert_eq!(tokens[4].pattern, "VCVV");
        assert!(tokens[4].is_valid);

        assert_eq!(tokens[5].output, "tʃyɛ");
        assert_eq!(tokens[5].pattern, "CVV");
        assert!(tokens[5].is_valid);

        let invalid = analyze_text("a CCV CVC");
        assert!(!invalid[0].is_valid);
        assert!(!invalid[1].is_valid);
        assert!(!invalid[2].is_valid);
    }

    #[test]
    fn all_phoneme_and_example_sound_files_exist() {
        for entry in VOWEL_INVENTORY.iter().chain(CONSONANT_INVENTORY.iter()) {
            assert!(
                std::path::Path::new(entry.sound_file).exists(),
                "Missing phoneme sound: {}",
                entry.sound_file
            );
            assert_eq!(entry.examples.len(), 5);
            for ex in &entry.examples {
                assert!(
                    std::path::Path::new(ex.sound_file).exists(),
                    "Missing example word sound: {} for /{}/",
                    ex.sound_file,
                    entry.ipa
                );
            }
        }
    }
}
