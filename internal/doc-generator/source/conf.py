# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'iceoryx2'
copyright = '2024, ekxide.io'
author = 'ekxide developers'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'myst_parser',          # Markdown support
    'sphinx_rtd_theme',     # Read The Docs theme
    'breathe',              # Doxygen XML support
    'exhale'                # Automatic API documentation generation
]

templates_path = ['_templates']
exclude_patterns = []

source_suffix = {
    '.rst': 'restructuredtext',
    '.md': 'markdown',
}

# -- Exhale configuration ---------------------------------------------------
# Setup the breathe extension
breathe_projects = {
    "iceoryx2": "../build/doxygen/xml"
}
breathe_default_project = "iceoryx2"

exhale_args = {
    # These arguments are required
    "containmentFolder":     "./api/cxx",
    "rootFileName":          "library_root.rst",
    "rootFileTitle":         "iceoryx2",
    "doxygenStripFromPath":  "..",
    "createTreeView":        True,
    "exhaleExecutesDoxygen": False,          # Add this line - don't run doxygen
}

primary_domain = 'cpp'
highlight_language = 'cpp'

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_theme_options = {
    'prev_next_buttons_location': 'bottom',
    'style_external_links': False,
    'collapse_navigation': True,   
    'sticky_navigation': True,      
    'navigation_depth': 4,          
    'includehidden': True,          
    'titles_only': False,
    'logo_only': False
}
html_static_path = ['_static']
