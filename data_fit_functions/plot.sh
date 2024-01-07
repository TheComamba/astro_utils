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

ensure_data_exists "Z0.0001"
ensure_data_exists "Z0.0002"
