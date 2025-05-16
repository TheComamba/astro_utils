#!/bin/bash

set -e

for file in *.rs; do
    sed -E -i 's/^const ([A-Z_1-9]+): RealData = RealData \{/fn \1() -> RealData { RealData {/' "$file"
    correct_import='use uom::si::{f64::{Length, Mass, ThermodynamicTemperature, Time}, length::light_year, thermodynamic_temperature::kelvin};'
    sed -i "/simple_si_units::/c\\$correct_import" "$file"
    sed -Ei ':a;N;$!ba;s/Length\s*\{\s*m:\s*([0-9.]+)\s*\*\s*SOLAR_RADIUS\.m,\s*\}/Length::new::<solar_radii>(\1)/g' "$file"
    sed -Ei ':a;N;$!ba;s/Mass\s*\{\s*kg:\s*([0-9.]+)\s*\*\s*SOLAR_MASS\.kg,\s*\}/Mass::new::<solar_mass>(\1)/g' "$file"
    sed -Ei ':a;N;$!ba;s/Temperature\s*\{\s*K:\s*([0-9.]+)\s*\}/ThermodynamicTemperature::new::<kelvin>(\1)/g' "$file"
    sed -Ei ':a;N;$!ba;s/Time\s*\{\s*s:\s*([0-9.]+)\s*\*\s*BILLION_YEARS\.s,\s*\}/Time::new::<gigayear>(\1)/g' "$file"
    sed -Ei ':a;N;$!ba;s/Length\s*\{\s*m:\s*([0-9.]+)\s*\*\s*LIGHT_YEAR\.m,\s*\}/Length::new::<light_year>(\1)/g' "$file"
    outdated_expressions=(
        'LIGHT_YEAR'
        'SOLAR_RADIUS'
        'SOLAR_MASS'
        'BILLION_YEARS'
    )
    for expr in "${outdated_expressions[@]}"; do
        sed -i "s/$expr//g" "$file"
    done
    sed -i "s/, ,//g" "$file"
    sed -i "s/length::{, }/length::{solar_radii }/g" "$file"
    sed -i "s/mass::,/mass::{solar_mass, },/g" "$file"
    sed -i "s/time::,/time::{gigayear, },/g" "$file"
done
