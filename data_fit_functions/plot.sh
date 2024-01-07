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

ensure_data_exists "Z0.0001"

plot_small_star() {
	echo "'Z0.0001/Z0.0001Y0.249OUTA1.77_F7_M${1}.DAT' using 3:${2} title 'M${1}' with lines,\\" >> gnuplot.gp
}

plot_star() {
	echo "'Z0.0001/Z0.0001Y0.249OUTA1.74_F7_M${1}.DAT' using 3:${2} title 'M${1}' with lines,\\" >> gnuplot.gp
}

plot_stars() {
	plot_small_star ".10" $1
	plot_small_star ".20" $1
	plot_small_star ".30" $1
	plot_small_star ".40" $1
	plot_small_star ".50" $1
	plot_small_star ".60" $1
	plot_small_star ".70" $1
	plot_star ".80" $1
	plot_star ".90" $1
	plot_star "1.00" $1
	plot_star "2.00" $1
	plot_star "3.00" $1
	plot_star "4.00" $1
	plot_star "5.00" $1
	plot_star "6.00" $1
	plot_star "7.00" $1
	plot_star "8.00" $1
	plot_star "9.00" $1
	plot_star "10.0" $1
	plot_star "20.0" $1
	plot_star "30.0" $1
	plot_star "40.0" $1
	plot_star "50.0" $1
	plot_star "60.0" $1
	plot_star "70.0" $1
	plot_star "80.0" $1
	plot_star "90.0" $1
	plot_star "100.0" $1
	plot_star "200.0" $1
	plot_star "300.0" $1
}

echo """
set terminal png size 1900,1000 enhanced font "Helvetica" 10
set logscale x
set xlabel 'Age [yr]'
""" > gnuplot.gp

echo """
set output 'Mass.png'
set ylabel 'Mass [M_Sun]'
plot \\""" >> gnuplot.gp
plot_stars 2

echo """
set output 'Absolute_Magnitude.png'
set ylabel 'Absolute Magnitude (?)'
plot \\""" >> gnuplot.gp
plot_stars 4

echo """
set output 'Effective_Temperature.png'
set ylabel 'Logarithmic Effective Temperature [log(K) (?)]'
plot \\""" >> gnuplot.gp
plot_stars 5

echo """
set output 'Radius.png'
set ylabel 'Radius [???]'
plot \\""" >> gnuplot.gp
plot_stars 6

gnuplot -p gnuplot.gp