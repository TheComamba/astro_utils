use super::*;
use crate::stars::real_data::RealData;

pub fn get_many_stars() -> Vec<RealData> {
    let mut all_stars: Vec<RealData> = Vec::new();
    // all_stars.append(&mut Corvus::get_stars());
    andromeda::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    antlia::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    apus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aquarius::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aquila::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ara::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aries::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    auriga::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    bootes::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    caelum::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    camelopardalis::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cancer::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canes_venatici::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canis_major::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canis_minor::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    capricornus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    carina::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cassiopeia::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    centaurus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cepheus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cetus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    chamaeleon::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    circinus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    columba::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    coma_berenices::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corona_australis::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corona_borealis::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corvus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    crater::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    crux::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cygnus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    delphinus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    dorado::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    draco::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    equuleus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    eridanus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    fornax::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    gemini::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    grus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hercules::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    horologium::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hydra::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hydrus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    indus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lacerta::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    leo::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    leo_minor::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lepus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    libra::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lupus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lynx::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lyra::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    mensa::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    microscopium::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    monoceros::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    musca::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    norma::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    octans::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ophiuchus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    orion::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pavo::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pegasus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    perseus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    phoenix::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pictor::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pisces::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    piscis_austrinus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    puppis::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pyxis::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    reticulum::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sagitta::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sagittarius::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    scorpius::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sculptor::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    scutum::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    serpens::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sextans::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    taurus::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    telescopium::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    triangulum::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    triangulum_australe::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    tucana::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ursa_major::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ursa_minor::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    vela::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    virgo::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    volans::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    vulpecula::STARS()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    all_stars
}
