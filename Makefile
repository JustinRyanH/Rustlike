TO_DELETE := $(shell find .git/hooks -type l)

git: cleanhooks
	ln -s ../../hooks/pre-commit .git/hooks/pre-commit

cleanhooks:
	rm $(TO_DELETE)
