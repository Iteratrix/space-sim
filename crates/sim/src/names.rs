//! Name generation for persons and minds.

use crate::state::Game;
use rand::Rng;

const GIVEN: &[&str] = &[
    "Okonkwo",
    "Varga",
    "Haddad",
    "Reyes",
    "Tanaka",
    "Adeyemi",
    "Lindqvist",
    "Marchetti",
    "Okafor",
    "Petrova",
    "Nakamura",
    "Osei",
    "Kowalski",
    "Almeida",
    "Chandra",
    "Fischer",
    "Mbeki",
    "Sørensen",
    "Yilmaz",
    "Quiroga",
    "Halloran",
    "Ibarra",
    "Jansen",
    "Kaur",
    "Lefebvre",
    "Mahmoud",
    "Nwosu",
    "Oyelaran",
    "Pham",
    "Rasmussen",
    "Saito",
    "Torres",
    "Umeh",
    "Vance",
    "Whitcombe",
    "Xiao",
    "Yamada",
    "Zaidi",
    "Abara",
    "Brennan",
    "Castellanos",
    "Dlamini",
    "Eriksen",
    "Ferreira",
    "Gallo",
    "Hoshino",
    "Iqbal",
    "Joshi",
    "Kimathi",
    "Lorenz",
    "Moreau",
    "Nilsen",
    "Ortiz",
    "Pacheco",
    "Rahimi",
    "Sato",
    "Tremblay",
    "Ueda",
    "Villanueva",
    "Wozniak",
    "Yoon",
    "Zhang",
    "Achterberg",
    "Bakshi",
    "Cordero",
    "Draganova",
    "Esposito",
    "Fontaine",
    "Gyasi",
    "Hartmann",
    "Ishikawa",
    "Jelinek",
    "Kessler",
    "Lombardi",
    "Mensah",
    "Novak",
    "Oduya",
    "Prakash",
    "Rinaldi",
];

const FIRST: &[&str] = &[
    "Ada", "Bao", "Ceri", "Dov", "Efe", "Fen", "Gil", "Hana", "Idris", "Jun", "Kai", "Lior",
    "Mira", "Nils", "Oba", "Pia", "Quin", "Rui", "Sana", "Tomas", "Uma", "Vela", "Wren", "Xu",
    "Yara", "Zev", "Amal", "Bram", "Cato", "Dana", "Eli", "Femi", "Gao", "Hiro", "Ines", "Joss",
    "Kemi", "Lena", "Mateo", "Nour", "Orin", "Priya", "Rafa", "Sol", "Tess", "Uri", "Vik", "Wale",
    "Yuki", "Zara",
];

const MIND_NAMES: &[&str] = &[
    "Tollan",
    "Ferrugem",
    "Kaimana",
    "Iskander",
    "Meridian",
    "Patient Argument",
    "Half a Degree",
    "Sennit",
    "Verdigris",
    "Long Count",
    "Quiet Sun",
    "Nadir",
];

/// A fresh person name not already used in the game.
pub fn person_name(game: &Game, rng: &mut impl Rng) -> String {
    for _ in 0..400 {
        let family = GIVEN[rng.random_range(0..GIVEN.len())];
        let first = FIRST[rng.random_range(0..FIRST.len())];
        let name = format!("{first} {family}");
        let taken = game.people.iter().any(|p| p.name == name);
        if !taken {
            return name;
        }
    }
    let n = game.people.len();
    format!(
        "{} {}-{n}",
        FIRST[rng.random_range(0..FIRST.len())],
        GIVEN[rng.random_range(0..GIVEN.len())]
    )
}

/// Names the oldest unnamed living mind, in the order the pitches gave.
pub fn name_oldest_mind(game: &mut Game) {
    let used: Vec<String> = game.minds.iter().filter_map(|m| m.name.clone()).collect();
    let Some(name) = MIND_NAMES.iter().find(|n| !used.iter().any(|u| u == *n)) else {
        return;
    };
    let Some(mind) = game
        .minds
        .iter_mut()
        .filter(|m| m.alive && m.name.is_none())
        .max_by_key(|m| m.counts_unblanked)
    else {
        return;
    };
    mind.name = Some((*name).to_owned());
}
