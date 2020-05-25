.PHONY: Thesis.pdf

Thesis.pdf: 
		pdflatex Thesis
		bibtex Thesis

diagrams:
	./build_diagrams.sh
