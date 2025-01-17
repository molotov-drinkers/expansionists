use std::collections::HashMap;

use crate::globe::territories::territory::{Continent, SubContinent, Size};

pub type I18nDefaultDictionary<'a> = HashMap<&'a str, &'a str>;
pub type I18nContinent<'a> = HashMap<&'a Continent, &'a str>;
pub type I18nSubContinent<'a> = HashMap<&'a SubContinent, &'a str>;
pub type I18nSize<'a> = HashMap<&'a Size, &'a str>;