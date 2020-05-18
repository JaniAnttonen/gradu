.PHONY: Thesis.pdf

Thesis.pdf: 
		pdflatex Thesis
		bibtex Thesis
