docs: book/src/*.md
	rm -rf $@
	mdbook build book
	mv book/book $@
