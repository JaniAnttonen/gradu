install-tectonic:
	sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
	cargo install tectonic

pdf: 
	tectonic Thesis.tex 

diagrams:
	./build_diagrams.sh
