SPHINXOPTS    ?=
SPHINXBUILD   ?= poetry run sphinx-build   # Changed this line
SPHINXAUTO    ?= poetry run sphinx-autobuild
SOURCEDIR     = source
BUILDDIR      = build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile install clean serve

install:
	poetry install --no-root

clean:
	rm -rf $(BUILDDIR)
	@$(SPHINXBUILD) -M clean "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

serve:
	$(SPHINXAUTO) "$(SOURCEDIR)" "$(BUILDDIR)/html" --watch $(SOURCEDIR) --open-browser

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
