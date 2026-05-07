PAPERS = $(wildcard publications/*/main.tex)
PDFS   = $(patsubst publications/%/main.tex, publications/%/main.pdf, $(PAPERS))
DIST   = docs

.PHONY: all clean dist

all: $(PDFS)

publications/%/main.pdf: publications/%/main.tex
	cd $(dir $<) && pdflatex -interaction=nonstopmode main.tex && pdflatex -interaction=nonstopmode main.tex

dist: all
	@mkdir -p $(DIST)
	@for pdf in $(PDFS); do \
		name=$$(basename $$(dirname $$pdf)); \
		cp $$pdf $(DIST)/$$name.pdf 2>/dev/null && echo "  copied $$name.pdf" || true; \
	done

clean:
	@for dir in $(dir $(PAPERS)); do \
		rm -f $$dir*.aux $$dir*.log $$dir*.out $$dir*.toc $$dir*.bbl $$dir*.blg; \
	done
