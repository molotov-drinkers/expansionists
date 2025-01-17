#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
  globe::territories::territory::{Continent, Size, SubContinent},
  i18n::base::{I18nContinent, I18nDefaultDictionary, I18nSubContinent}
};

use super::base::I18nSize;

fn get_territory_pt_br() -> I18nDefaultDictionary<'static> {
  let territory_pt_br: I18nDefaultDictionary = HashMap::from([
    ("horn", "O Chifre"),
    ("sahel", "Sahel"),
    ("africa_rainforest", "Floresta Tropical da Africa"),
    ("namid_desert", "Deserto do Namibe"),
    ("kalahari", "Calaári"),
    ("sahara", "Saara"),
    ("east_savanna", "Savanna do Leste"),
    ("african_south_central_plateau", "Planalto Centro-Sul Africano"),
    ("nile_river_region", "Região do Rio Nilo"),
    ("the_greatest_african_island", "A Maior Ilha Africana"),
    ("niger_river", "Rio Níger"),
    ("volta_lake", "Lago Volta"),
    ("africa_west_region", "Região Oeste da Africa"),
    ("amazon", "Amazônia"),
    ("andes", "Andes"),
    ("atlantic_forest", "Mata Atlântica"),
    ("caatinga", "Caatinga"),
    ("incas", "Incas"),
    ("latinos", "Latinos"),
    ("pampas", "Pampas"),
    ("patagonia", "Patagonia"),
    ("tropical_highlands", "Planalto Tropical"),
    ("californias", "Californias"),
    ("caribbean_islands", "Ilhas do Caribe"),
    ("north_america_desert", "Deserto Norte-Americano"),
    ("great_lakes", "Os Grandes Lagos"),
    ("artic_territories", "Territórios Árticos"),
    ("baffin_bay", "Baía de Baffin"),
    ("labrador_sea_neighbors", "Vizinhos do Mar do Labrador"),
    ("new_great_britain", "Nova Grã-Bretanha"),
    ("mississippi_way", "Caminho do Mississippi"),
    ("romance_speaking_territory", "Território de Língua Romântica"),
    ("southern_north", "O Sul do Norte"),
    ("parallel_49th", "Paralelo 49"),
    ("great_bear_lake", "Grande Lago do Urso"),
    ("slave_lake", "Lago do Escravo"),
    ("mount_columbia", "Monte Columbia"),
    ("thousand_lakes_region", "Região dos Mil Lagos"),
    ("hudson_bay_viewers", "Observadores da Baía de Hudson"),
    ("north_pacific_civilization", "Civilização do Pacífico Norte"),
    ("aztecas", "Astecas"),
    ("mayas", "Maias"),
    ("cocibolca_lake", "Mar Dulce"),
    ("arabian_peninsula", "Península Arábica"),
    ("suez_canal", "Canal de Suez"),
    ("east_dead_sea", "Mar Morto Oriental"),
    ("lut_desert", "Deserto de LutE"),
    ("monotheist_realms", "Reinos Monoteístas"),
    ("zagros_mountains", "Cordilheira de Zagros"),
    ("caspian_coast", "Costa Cáspia"),
    ("caucasus", "Cáucaso"),
    ("east_siberia", "Sibéria do Leste"),
    ("west_siberia", "Sibéria do Oeste"),
    ("lake_balkhash", "Lago Balcache"),
    ("amu_darya_river", "Rio Amu Dária"),
    ("aral_sea", "Mar de Aral"),
    ("zeya_dam", "Barragem de Zeya"),
    ("han_land", "Povo Han"),
    ("shibuya", "Shibuya"),
    ("daisetsuzan", "Daisetsuzan"),
    ("korean_peninsula", "Península Coreana"),
    ("cantonese_lands", "Terras Cantonesas"),
    ("great_wall", "A Muralha"),
    ("manchuria", "Manchúria"),
    ("mount_fuji", "Monte Fuji"),
    ("gobi_desert", "Deserto de Gobi"),
    ("loess_plateau", "Planalto Loess"),
    ("borneo_island", "Ilha de Bornéu"),
    ("gede_pangrango", "Gede-Pangrango"),
    ("banda_arc", "Arco Vulcânico de Banda"),
    ("das_visayas_sea", "Mar das Visayas"),
    ("tri_an_lake", "Lago Tri An"),
    ("asia_southeast_peninsula", "Península do Sudeste Asiático"),
    ("chao_phraya_river", "Rio Chao Phraya"),
    ("tonle_sap", "Lago Sap"),
    ("phou_bia", "Phou Bia"),
    ("red_river", "Rio Vermelho"),
    ("irrawaddy_river", "Rio Irauádi"),
    ("east_new_guinea", "Nova Guiné Oriental"),
    ("balimela_dam", "Represa Balimela"),
    ("central_sub_continent_highlands", "Planalto do Centrol do Sub-Continente"),
    ("ganges_delta_region", "Região do Delta do Ganges"),
    ("indo_river", "Rio Indo"),
    ("kaveri_river", "Rio Kaveri"),
    ("mount_pidurutalagala", "Monte Pidurutalagala"),
    ("thar_desert", "Deserto do Thar"),
    ("western_ghats", "Gates Ocidentais"),
    ("himalayas", "Himalaias"),
    ("k2_mountain", "Montanha K2"),
    ("registan_desert", "Deserto de Registan"),
    ("tian_shan_mountains", "Coordilheiras de Tian Shan"),
    ("lake_baikal", "Lago Baical"),
    ("nordics", "Nórdicos"),
    ("the_islands", "As Ilhas"),
    ("rhine_region", "Região do Réno"),
    ("balkan_peninsula", "Península Balcânica"),
    ("euro_romance_lands", "Terras de Línguas Românicas"),
    ("west_slavs", "Eslavos do Oeste"),
    ("baltics", "Bálticos"),
    ("big_plain", "A Grande Planície"),
    ("north_black_sea", "Norte do Mar Negro"),
    ("urau_mountains", "Montes Urais"),
    ("white_sea", "Mar Branco"),
    ("maoris", "Maoris"),
    ("aussie_desert", "Deserto Ozzie"),
    ("coral_sea_coast", "Costa do Mar de Coral"),
    ("daintree_rainforest", "Floresta Tropical Daintree"),
    ("kangaroos", "Cangurus"),
    ("most_isolated_city", "A Cidade Mais Isolada"),
    ("temperate_land", "Terra Temperada"),
    ("south_pacific_islands", "Ilhas do Pacífico Sul"),
    ("diomede_islands", "Ilhas Diômedes"),
    ("asia_europe_connection", "Conexão Asia-Europa"),
    ("west_antarctica", "Antarctica do Oeste"),
    ("somov_sea", "Mar de Somov"),
    ("east_antarctica", "Antarctica do Lest"),
    ("riiser_larsen_ice_shelf", "Plataforma de Gelo Riiser Larsen"),
    ("everybodys_south", "O Sul de Todos"),
    ("antartica_peninsula", "Peninsula da Antartica"),
    ("unclaimed_area", "Area Nunca Revindicada"),
  ]);

  territory_pt_br
}

fn get_continent_pt_br() -> I18nContinent<'static> {
  let continent_pt_br: I18nContinent = HashMap::from([
    (&Continent::Africa, "Africa"),
    (&Continent::Asia, "Ásia"),
    (&Continent::Europe, "Europe"),
    (&Continent::NorthAmerica, "América do Norte"),
    (&Continent::Oceania, "Oceania"),
    (&Continent::SouthAmerica, "América do Sul"),
    (&Continent::Antarctica, "Antártica"),
    (&Continent::Special, "Especial"),
  ]);
  continent_pt_br
}

fn get_sub_continent_pt_br() -> I18nSubContinent<'static> {
  let sub_continent_pt_br: I18nSubContinent = HashMap::from([
    (&SubContinent::MiddleEast, "Oriente Médio"),
    (&SubContinent::InteriorAsia, "Interior Asiático"),
    (&SubContinent::IndianSubcontinent, "Subcontinente Indiano"),
    (&SubContinent::SoutheastAsia, "Suldeste Asiático"),
    (&SubContinent::EastAsia, "Leste Asiático"),
    (&SubContinent::EuropeRelatedAsia, "Asia Relacionada à Europa"),
  ]);
  sub_continent_pt_br
}

fn get_size_en_default() -> I18nSize<'static> {
  let size_en_default: I18nSize = HashMap::from([
    (&Size::Tiny, "Pequenino"),
    (&Size::Small, "Pequeno"),
    (&Size::Medium, "Médio"),
    (&Size::Large, "Grande"),
    (&Size::Huge, "Gigante"),
    (&Size::Humongous, "Monstruoso"),
    (&Size::None, "Nenhum"),
  ]);

  size_en_default
}

fn get_full_dictionary_pt_br() -> I18nDefaultDictionary<'static> {
  HashMap::from([
    ("troop", "Tropa"),
    ("troops", "Tropas"),
    ("unoccupied_territory", "Território Desocupado"),
    ("ruler", "Regente"),
  ])
}