dev:
	zola serve

build:
	zola build

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
