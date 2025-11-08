rmsubgit:
	@for dir in $$(find . -maxdepth 1 -type d | grep "^\./" | grep -v "^\./\.git$$"); do \
		if [ -f "$$dir/.gitignore" ]; then \
			echo "Removing $$dir/.gitignore"; \
			rm "$$dir/.gitignore"; \
		fi; \
		if [ -d "$$dir/.git" ]; then \
			echo "Removing $$dir/.git"; \
			rm -rf "$$dir/.git"; \
		fi; \
	done