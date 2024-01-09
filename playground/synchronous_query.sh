#!/bin/bash

req="https://gea.esac.esa.int/tap-server/tap/sync"
req+="?REQUEST=doQuery"
req+="&LANG=ADQL"
req+="&FORMAT=json"
req+="&QUERY=SELECT+ecl_lon,ecl_lat,phot_g_mean_mag,logg_gspphot,distance_gspphot"
req+="+FROM+gaiadr3.gaia_source"
req+="+WHERE+phot_g_mean_mag+<+6.5"
curl $req -o result.json
