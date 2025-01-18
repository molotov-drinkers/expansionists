use std::collections::HashMap;

use crate::globe::territories::territory::{Continent, SubContinent, Size};

use super::{en_international::InternationalEnglish, es_us::AmericanSpanish, pt_br::BrazilianPortuguese};

pub type I18nDefaultDictionary<'a> = HashMap<&'a str, &'a str>;
pub type I18nContinent<'a> = HashMap<&'a Continent, &'a str>;
pub type I18nSubContinent<'a> = HashMap<&'a SubContinent, &'a str>;
pub type I18nSize<'a> = HashMap<&'a Size, &'a str>;

pub trait ILanguage {
  fn get_general_dictionary(&self) -> I18nDefaultDictionary<'static> {
    unimplemented !()
  }
  fn get_territory_dictionary(&self) -> I18nDefaultDictionary<'static> {
    unimplemented !()
  }
  fn get_continents(&self) -> I18nContinent<'static> {
    unimplemented !()
  }
  fn get_sub_continents(&self) -> I18nSubContinent<'static> {
    unimplemented !()
  }
  fn get_sizes(&self) -> I18nSize<'static> {
    unimplemented !()
  }
}

#[derive(Debug, Clone)]
pub enum AvailableLanguage {
  // English
  InternationalEnglish,

  // Portuguese
  BrazilianPortuguese,
  
  // Spanish
  AmericanSpanish,
  // German
  // French
  // Italian
  // Japanese
  // Korean
  // Russian
  // SimplifiedChinese
  // Turkish
  // Arabic
  // Polish
  // Czech
}

impl AvailableLanguage {
  pub fn get_translations(&self) -> Box<dyn ILanguage> {
    match self {
      AvailableLanguage::InternationalEnglish => Box::new(InternationalEnglish {}),
      AvailableLanguage::BrazilianPortuguese => Box::new(BrazilianPortuguese {}),
      AvailableLanguage::AmericanSpanish => Box::new(AmericanSpanish {}),
    }
  }
}