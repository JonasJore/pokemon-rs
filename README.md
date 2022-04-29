# pokemon-rs

![crates version](https://img.shields.io/crates/v/pokemon-rs.svg?style=flat-square)
![docs build](https://img.shields.io/docsrs/pokemon-rs)

## Description

Rust library for getting Pokémon names

PRs for adding support for more translated Pokémon generations welcome
  
Inspired by [Pokémon](https://github.com/sindresorhus/pokemon)  
  
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

// There is support for another language than default (None mapped to en) that can be given, like `Some("jp")`):
pokemon_rs::get_all(Some("jp"));
// => ['フシギダネ', 'フシギソウ', 'フシギバナ', ...]

pokemon_rs::get_by_id(33, Some("jp"));
// => ニドリーノ

pokemon_rs::get_id_by_name("フシギダネ", Some("jp"))
// => 1

pokemon_rs::random(Some("jp"));
// => プリン
```

## Supported Pokémon generations

|  Generation  |  Region  |  Locale  | Supported |
| --- | --- | --- | --- |
| 1 | Kanto | en(default) | ✅ |
| 2 | Johto | en(default) | ✅ |
| 3 | Hoenn | en(default) | ✅ |
| 4 | Sinnoh | en(default) | ❌ |
| 5 | Unova | en(default) | ❌ |
| 6 | Kalos | en(default) | ❌ |
| 7 | Alola  | en(default) | ❌ |
| 8 | Galar  | en(default) | ❌ |
| 9 | TBA  | en(default) | ❌ |
| 1 | Kanto | ch | ✅ |
| 2 | Johto  | ch | ✅ |
| 3 | Hoenn  | ch | ❌ |
| 4 | Sinnoh | ch | ❌ |
| 5 | Unova | ch | ❌ |
| 6 | Kalos | ch | ❌ |
| 7 | Alola  | ch | ❌ |
| 8 | Galar  | ch | ❌ |
| 9 | TBA | ch | ❌ |
| 1 | Kanto | de | ✅ |
| 2 | Johto | de | ✅ |
| 3 | Hoenn | de | ❌ |
| 4 | Sinnoh | de | ❌ |
| 5 | Unova | de | ❌ |
| 6 | Kalos | de | ❌ |
| 7 | Alola | de | ❌ |
| 8 | Galar | de | ❌ |
| 9 | TBA | de | ❌ |
| 1 | Kanto | fr | ✅ |
| 2 | Johto | fr | ✅ |
| 3 | Hoenn | fr | ❌ |
| 4 | Sinnoh | fr | ❌ |
| 5 | Unova | fr | ❌ |
| 6 | Kalos | fr | ❌ |
| 7 | Alola | fr | ❌ |
| 8 | Galar | fr | ❌ |
| 9 | TBA | fr | ❌ |
| 1 | Kanto | jp | ✅ |
| 2 | Johto | jp | ✅ |
| 3 | Hoenn | jp | ❌ |
| 4 | Sinnoh | jp | ❌ |
| 5 | Unova | jp | ❌ |
| 6 | Kalos | jp | ❌ |
| 7 | Alola | jp | ❌ |
| 8 | Galar | jp | ❌ |
| 9 | TBA | jp | ❌ |
| 1 | Kanto | ru | ✅ |
| 2 | Johto | ru | ✅ |
| 3 | Hoenn | ru | ❌ |
| 4 | Sinnoh | ru | ❌ |
| 5 | Unova | ru | ❌ |
| 6 | Kalos | ru | ❌ |
| 7 | Alola | ru | ❌ |
| 8 | Galar | ru | ❌ |
| 9 | TBA | ru | ❌ |

## Supported languages

---
`en`: English (default language if None is given as `locale`)  
`jp`: Japanese  
`fr`: French  
`de`: German  
`ru`: Russian  
`ch`: Chinese  
