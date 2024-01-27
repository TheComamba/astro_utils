#!/bin/bash

req="https://gea.esac.esa.int/tap-server/tap/sync"
req+="?REQUEST=doQuery"
req+="&LANG=ADQL"
req+="&FORMAT=json"
req+="&QUERY=SELECT+designation,ecl_lon,ecl_lat,phot_g_mean_mag,logg_gspphot,distance_gspphot,parallax,teff_gspphot"
req+="+FROM+gaiadr3.gaia_source"
req+="+WHERE+phot_g_mean_mag+<+2.5"
curl $req -o result.json
