#!/bin/bash

set -e

ensure_prerequisite() {
	if ! command -v $1 &> /dev/null
	then
		echo "Installing $1."
		sudo apt-get install -y $1
	fi
}

ensure_data_exists() {
	if [ ! -d $1 ]
	then
		if [ ! -f $1.tar.gz ]
		then
			echo "File $1.tar.gz does not exist. Downloading it."
			curl -o $1.tar.gz https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/no_phase/$1.tar.gz
		fi

		tar -xzf $1.tar.gz
		rm $1.tar.gz
	fi
}

ensure_prerequisite curl
ensure_prerequisite tar
ensure_prerequisite gnuplot

ensure_data_exists "Z0.01"

filename="Z0.01/all.dat"
if [ ! -f $filename ]
then
	append_small_star() {
		cat "Z0.01/Z0.01Y0.267OUTA1.77_F7_M${1}.DAT" >> $2
	}

	append_star() {
		cat "Z0.01/Z0.01Y0.267OUTA1.74_F7_M${1}.DAT" >> $2
	}

	append_small_star ".10" $filename
	append_small_star ".20" $filename
	append_small_star ".30" $filename
	append_small_star ".40" $filename
	append_small_star ".50" $filename
	append_small_star ".60" $filename
	append_small_star ".70" $filename
	append_star ".80" $filename
	append_star ".90" $filename
	append_star "1.00" $filename
	append_star "2.00" $filename
	append_star "3.00" $filename
	append_star "4.00" $filename
	append_star "5.00" $filename
	append_star "6.00" $filename
	append_star "7.00" $filename
	append_star "8.00" $filename
	append_star "9.00" $filename
	append_star "10.0" $filename
	append_star "20.0" $filename
	append_star "30.0" $filename
	append_star "40.0" $filename
	append_star "50.0" $filename
	append_star "60.0" $filename
	append_star "70.0" $filename
	append_star "80.0" $filename
	append_star "90.0" $filename
	append_star "100.0" $filename
	append_star "200.0" $filename
	append_star "300.0" $filename
fi

echo """
set terminal png size 1900,1000 enhanced font "Helvetica" 10
set logscale x
set xlabel 'Age [yr]'
""" > gnuplot.gp

echo """
set xlabel 'Mass [M_Sun]'
set xrange [0:310]
set logscale x
set ylabel 'Time [yr]'
set yrange [0:1e10]
set logscale y
#set view map
#set pm3d at b map
#set dgrid3d 50,50
#set hidden3d
""" >> gnuplot.gp

echo """
set zlabel 'Absolute Magnitude (?)'

abs_mag(m,t) = 1.0

set output 'Absolute_Magnitude.png'
splot 'Z0.01/all.dat' using 2:3:4 notitle, \
	abs_mag(x,y) notitle

set output 'Absolute_Magnitude_minus_fit.png'
splot 'Z0.01/all.dat' using 2:3:(column(4)-abs_mag(column(2),column(3))) notitle
""" >> gnuplot.gp

echo """
set output 'Effective_Temperature.png'
set zlabel 'Logarithmic Effective Temperature [log(K) (?)]'

temp(m,t) = 1.0

splot 'Z0.01/all.dat' using 2:3:5 notitle, \
	temp(x,y) notitle

set output 'Effective_Temperature_minus_fit.png'
splot 'Z0.01/all.dat' using 2:3:(column(5)-temp(column(2),column(3))) notitle
""" >> gnuplot.gp

echo """
set output 'Radius.png'
set zlabel 'Radius [???]'

radius(m,t) = 1.0

splot 'Z0.01/all.dat' using 2:3:6 notitle, \
	radius(x,y) notitle

set output 'Radius_minus_fit.png'
splot 'Z0.01/all.dat' using 2:3:(column(6)-radius(column(2),column(3))) notitle
""" >> gnuplot.gp

gnuplot -p gnuplot.gp
