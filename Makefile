install-tectonic:
	sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
	cargo install tectonic

pdf: 
	tectonic -X build

clean-pdf:
	tectonic -X build && sudo rm -rf _minted-Thesis

diagrams:
	./build_diagrams.sh
