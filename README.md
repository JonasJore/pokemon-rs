# pokemon-rs

## Description

This rust library is for getting Pokémon names
is available (pull-requests for adding more pokemon welcome) for [languages](#supported-languages)  
  
Inspired by [Pokémon](https://github.com/sindresorhus/pokemon)  
  
Usage:

```rust
use pokemon_rs;

pokemon_rs::get_all(None);
// => ['Bulbasaur', 'Ivysaur', 'Vensaur', ...]

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

## Supported languages

---
`en`: English (default language if None is given as `locale`)  
`jp`: Japanese  
`fr`: French  
`de`: German  
`ru`: Russian  
`ch`: Chinese  
