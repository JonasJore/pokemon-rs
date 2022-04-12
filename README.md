# pokemon-rs

## Description
This crate is for getting Pokémon names, currently only first generation pokemon
is available (pull-requests for adding more pokemon welcome) for [languages](#supported-languages)

usage:

```rust
use pokemon;

pokemon::get_all(None);
// => ['Bulbasaur', ...]

pokemon::get_by_id(33, None);
// => Nidorino

pokemon::get_id_by_name("Pikachu", None);
// => 25

pokemon::random(None);
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
