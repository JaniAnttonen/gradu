pdf: 
	bibtex Thesis
	pdflatex -synctex=1 -interaction=nonstopmode --shell-escape Thesis

diagrams:
	./build_diagrams.sh
