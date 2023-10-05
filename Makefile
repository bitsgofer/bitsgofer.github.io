UNAME_S := $(shell uname -s)
ifeq (${UNAME_S},Darwin)        # OSX
	PANDOC_BIN=$(shell which pandoc)
else ifeq (${UNAME_S},Linux)    # Linux
	PANDOC_BIN=$(shell which pandoc)
	MINIFY_BIN=$(shell which minify)
endif
# https://pandoc.org/installing.html
PANDOC=${PANDOC_BIN}
MINIFY=${MINIFY_BIN}

# https://github.com/tdewolff/minify/tree/master/cmd/minify
${MINIFY}:
	go install github.com/tdewolff/minify/v2/cmd/minify@v2.12.9

MARKDOWN_FILES := $(shell find ./content -type f -name '*.md')
HTML_FILES := $(patsubst ./content/%/README.md, ./_html/%/index.html, $(MARKDOWN_FILES))

THEME_CSS_FILES := $(shell find ./themes/indie_studio -type f -name '*.css' | sort)
THEME_JS_FILES := $(shell find ./themes/indie_studio -type f -name '*.js')

THEME_SVG_FILES := $(shell find ./themes/indie_studio -type f -name '*.svg')
SVG_FILES := $(patsubst ./themes/indie_studio/images/%.svg, ./_html/images/%.svg, $(THEME_SVG_FILES))

THEME=themes/indie_studio

HTML_FOLDER=./_html
_html/%/index.html: content/%/README.md
	mkdir -p $(dir ${@})
	${PANDOC} \
		--standalone \
		--output ${@} \
		--metadata=current-date=$(shell date +"%Y-%b-%d") \
		--metadata=last-modified-date=$(shell date -d "$(stat -c %y ${<})" +"%Y-%b-%d") \
		--template=${THEME}/templates/page.html \
		--highlight-style=${THEME}/pandoc/highlight-theme/solarized.theme \
		${<}

_html/images/%.svg: themes/indie_studio/images/%.svg
	mkdir -p $(dir ${@})
	cp ${<} ${@}

.cache/css/theme.css: ${THEME_CSS_FILES}
	mkdir -p $(dir ${@})
	cat ${^} > ${@}

_html/css/theme.css: .cache/css/theme.css
	mkdir -p $(dir ${@})
	${MINIFY} --type=css --output ${@} ${<}

.cache/js/theme.js: ${THEME_JS_FILES}
	mkdir -p $(dir ${@})
	cat ${^} > ${@}

_html/js/theme.js: .cache/js/theme.js
	mkdir -p $(dir ${@})
	${MINIFY} --type=js --output ${@} ${<}

render: ${HTML_FILES}
render: _html/css/theme.css
render: _html/js/theme.js
render: ${SVG_FILES}
render:
.PHONY: render

clean:
	rm -r \
		.cache \
		${HTML_FOLDER}/*
.PHONY: clean
