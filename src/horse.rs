use serde::{Deserialize, Serialize};

use crate::{form_table::FormTableHorse, FieldTableHorse};

#[derive(Debug, Serialize, Deserialize)]
pub struct Horse {
    pub number: i32,
    pub emergency: bool,
    pub scratched: bool,
    pub name: String,
    pub silks_img_src: String,
    pub last10: String,

    pub trainer: String,
    pub trainer_location: String,

    pub jockey: String,
    pub jockey_claim: Option<String>,

    pub barrier: String,
    pub weight: String,
    pub probable_weight: String,
    pub penalty: String,
    pub hcp_rating: String,

    pub age: String,
    pub colour: String,
    pub sex: String,
    pub foaled: String,
    pub sire: String,
    pub dam: String,
    pub dam_sire: String,
    pub breeder: String,
    pub owners: String,
    pub colours: String,

    pub record: String,
    pub prizemoney: String,
    pub first_up: String,
    pub second_up: String,
    pub track: String,
    pub dist: String,
    pub track_dist: String,
    pub firm: String,
    pub good: String,
    pub soft: String,
    pub heavy: String,
    pub synthetic: String,
}

impl Horse {
    pub fn from_entries(field: FieldTableHorse, form: FormTableHorse) -> Self {
        Horse {
            number: field.no,
            emergency: field.emergency,
            scratched: field.scratched,
            name: form.horse.name,
            silks_img_src: form.horse.silks_img_src,
            last10: field.last10,

            trainer: field.trainer,
            trainer_location: form.stats.trainer_location,
            jockey: field.jockey,
            jockey_claim: form.stats.jockey_claim,

            barrier: field.barrier,
            weight: form.stats.weight.unwrap_or(field.weight),
            probable_weight: field.probable_weight,
            penalty: field.penalty,
            hcp_rating: field.hcp_rating,

            age: form.horse.info.age,
            colour: form.horse.info.colour,
            sex: form.horse.info.sex,
            foaled: form.horse.info.foaled,
            sire: form.horse.info.sire,
            dam: form.horse.info.dam,
            dam_sire: form.horse.info.dam_sire,
            breeder: form.horse.info.breeder,
            owners: form.horse.info.owners,
            colours: form.horse.info.colours,

            record: form.stats.record,
            prizemoney: form.stats.prizemoney,
            first_up: form.stats.first_up,
            second_up: form.stats.second_up,
            track: form.stats.track,
            dist: form.stats.dist,
            track_dist: form.stats.track_dist,
            firm: form.stats.firm,
            good: form.stats.good,
            soft: form.stats.soft,
            heavy: form.stats.heavy,
            synthetic: form.stats.synthetic,
        }
    }

    pub fn vec_zip(
        field_horses: Vec<FieldTableHorse>,
        form_horses: Vec<FormTableHorse>,
    ) -> Vec<Self> {
        field_horses
            .into_iter()
            .zip(form_horses.into_iter())
            .map(|(f, fm)| Horse::from_entries(f, fm))
            .collect()
    }
}
