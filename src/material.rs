

pub struct Incidence {
    pub normal: V3,
    pub light_dir: V3,
    pub eye_dir: V3,
    pub light_color: V3
}

pub trait IsoMaterial {
    fn refl_color(incidence: Incidence) -> V3;
}
