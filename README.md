# pokemon_rs

## Description
This crate is for getting Pokémon names, currently only first generation pokemon
is available (pull-requests for adding more pokemon welcome) for [languages](#supported-languages)

This is a rust-library inspired by [Pokémon](https://github.com/sindresorhus/pokemon)

usage:

```rust
use pokemon_rs;

pokemon_rs::get_all(None);
// => ['Bulbasaur', ...]

pokemon_rs::get_by_id(33, None);
// => Nidorino

pokemon_rs::get_id_by_name("Pikachu", None);
// => 25

pokemon_rs::random(None);
// => Raichu
```

## Supported languages
---
`en`: English (default language if None is given as `locale`)  
`jp`: Japanese  
`fr`: French  
`de`: German  
`ru`: Russian  
`ch`: Chinese  
