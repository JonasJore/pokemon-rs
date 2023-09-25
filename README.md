# pokemon-rs

![crates version](https://img.shields.io/crates/v/pokemon-rs.svg?style=flat-square)
![repos size](https://img.shields.io/github/repo-size/jonasjore/pokemon-rs)
![docs build](https://img.shields.io/docsrs/pokemon-rs)
![downloads](https://img.shields.io/crates/d/pokemon-rs)
![recent downloads](https://img.shields.io/crates/dr/pokemon-rs)
![downloads latest version](https://img.shields.io/crates/dv/pokemon-rs)
## Description

Rust library for getting Pokémon names

PRs for adding support for more translated Pokémon generations welcome
  
## Usage

```rust
use pokemon_rs;

pokemon_rs::get_all(None);
// => ['Bulbasaur', 'Ivysaur', 'Venusaur', ...]

pokemon_rs::get_by_id(33, None);
// => Nidorino

pokemon_rs::get_id_by_name("Pikachu", None);
// => 25

pokemon_rs::random(None);
// => Raichu

pokemon_rs::get_generation("Kanto", Some("en"));
// => ['Bulbasaur', 'Ivysaur', 'Venusaur', ...'Mewtwo', 'Mew']

// There is support for another language than default (None mapped to en) that can be given, like `Some("jp")`):
pokemon_rs::get_all(Some("jp"));
// => ['フシギダネ', 'フシギソウ', 'フシギバナ', ...]

pokemon_rs::get_by_id(33, Some("jp"));
// => ニドリーノ

pokemon_rs::get_id_by_name("フシギダネ", Some("jp"));
// => 1

pokemon_rs::random(Some("jp"));
// => プリン

pokemon_rs::get_generation("Kanto", Some("jp"));
// => ['フシギダネ', 'フシギソウ', 'フシギバナ', ...'ミュウツー', 'ミュウ']

pokemon_rs::get_region(4);
// => Sinnoh
```

## Supported Pokémon generations and languages

✅ = Supported  
❌ = Not Supported  
⌛ = Partly Supported

|  Generation | Region | EN(default) | CH | DE | FR | JP | RU |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Kanto | ✅ | ✅ | ✅  | ✅ | ✅ | ✅ |
| 2 | Johto | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 3 | Hoenn | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 4 | Sinnoh | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| 5 | Unova | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| 6 | Kalos | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| 7 | Alola | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ |
| 8 | Galar | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ |
| 9 | Paldea | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ |

## Supported languages

---
`en`: English (default language if None is given as `locale`)  
`jp`: Japanese  
`fr`: French  
`de`: German  
`ru`: Russian  
`ch`: Chinese  
