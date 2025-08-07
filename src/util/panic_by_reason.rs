pub mod panic_handling {
    use crate::github::constants::{REPO_ISSUES, REPO_LINK};

    pub enum PanicReason {
        UnsupportedLanguage,
        UnsupportedLanguageContribute,
        UnsupportedPokemon,
        UnsupportedPokemonSprite,
        TranslatedList,
    }

    pub fn get_panic_by_reason(panic_reason: PanicReason) -> ! {
        match panic_reason {
            PanicReason::UnsupportedLanguage => panic!(
                "Invalid or unsupported locale. PRs welcome at {}",
                REPO_LINK
            ),
            PanicReason::UnsupportedPokemon => panic!(
                "The pokémon given does not seem to have been added to the list yet, PRs welcome at {}",
                REPO_LINK
            ),
            PanicReason::UnsupportedLanguageContribute => panic!(
                "Language currently not supported. Want support for your language? Pull requests welcome at {}. Or you can just post a feature request as an issue here: {}",
                REPO_LINK,
                REPO_ISSUES
            ),
            PanicReason::TranslatedList => panic!(
                "Translated list could not be returned, please report issues here: {}",
                REPO_ISSUES
            ),
            PanicReason::UnsupportedPokemonSprite => panic!(
                "The pokémon given does not seem to have been added as a sprite yet, PRs welcome at {}",
                REPO_LINK
            ),
        }
    }
}
