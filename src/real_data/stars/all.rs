use super::*;
use crate::stars::real_data::RealData;

pub fn get_many_stars() -> Vec<RealData> {
    let mut all_stars: Vec<RealData> = Vec::new();
    // all_stars.append(&mut Corvus::get_stars());
    andromeda::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    antlia::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    apus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aquarius::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aquila::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ara::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    aries::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    auriga::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    bootes::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    caelum::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    camelopardalis::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cancer::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canes_venatici::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canis_major::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    canis_minor::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    capricornus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    carina::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cassiopeia::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    centaurus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cepheus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cetus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    chamaeleon::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    circinus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    columba::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    coma_berenices::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corona_australis::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corona_borealis::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    corvus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    crater::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    crux::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    cygnus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    delphinus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    dorado::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    draco::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    equuleus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    eridanus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    fornax::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    gemini::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    grus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hercules::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    horologium::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hydra::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    hydrus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    indus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lacerta::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    leo::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    leo_minor::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lepus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    libra::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lupus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lynx::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    lyra::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    mensa::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    microscopium::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    monoceros::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    musca::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    norma::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    octans::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ophiuchus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    orion::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pavo::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pegasus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    perseus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    phoenix::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pictor::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pisces::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    piscis_austrinus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    puppis::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    pyxis::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    reticulum::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sagitta::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sagittarius::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    scorpius::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sculptor::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    scutum::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    serpens::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    sextans::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    taurus::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    telescopium::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    triangulum::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    triangulum_australe::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    tucana::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ursa_major::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    ursa_minor::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    vela::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    virgo::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    volans::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    vulpecula::stars()
        .into_iter()
        .for_each(|star| all_stars.push(star));
    all_stars
}
