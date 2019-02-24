docs: book/src/*.md
	mdbook build book
	mv book/book $@
