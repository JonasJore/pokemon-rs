pub enum Generation {
    Kanto,
}


/// TODO: denne sjekker ikke for alle generasjoner ennå...
fn map_str_to_generation(generation_name: &str ) -> Generation {
    match generation_name {
        "Kanto" => Generation::Kanto,
        _ => panic!("")
    }
}