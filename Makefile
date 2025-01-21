dev:
	zola serve --drafts

build:
	zola build --drafts

check/links:
	@wget --directory-prefix=/tmp/check-links \
		 --spider \
		 --output-file=- \
		 --execute=robots=off \
		 --reject livereload.js \
		 -rp http://127.0.0.1:1111 > /dev/null

crawl:
	wget --directory-prefix=/tmp/check-links \
		 --spider \
		 --output-file=- \
		 --execute=robots=off \
		 --reject livereload.js \
		 -rp http://127.0.0.1:1111 \
		| less

# If the first argument is "new"...
ifeq (new,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "new"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

.PHONY: new
new:
	mkdir content/$(RUN_ARGS)
	cp content/template content/$(RUN_ARGS)/index.md
	echo content/$(RUN_ARGS)/index.md
