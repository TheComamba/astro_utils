#!/bin/bash

set -e

for file in *.rs; do
    sed -E -i 's/^const ([A-Z_1-9]+): RealData = RealData \{/fn \1() -> RealData { RealData {/' "$file"
    correct_import='use uom::si::{f64::{Length, Mass, ThermodynamicTemperature, Time}, length::light_year, thermodynamic_temperature::kelvin};'
    sed -i "/simple_si_units::/c\\$correct_import" "$file"
    sed -i "/units::{/,/},/d" "$file"
done
