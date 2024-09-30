install:
	mkdir -p ~/.local/bin/osrepo
	mkdir -p ~/.config/osr
	cp -r osrepo ~/.local/bin/
	sudo ln -s -f ~/.local/bin/osrepo/app.py /usr/bin/osr
	ln -s -f ~/.local/bin/osrepo/repo ~/.config/osr/
	sudo chmod -c 0755 /usr/bin/osr

clean:
	rm -rf ~/.local/bin/osrepo
	sudo rm -rf ~/usr/bin/osr
	rm -rf ~/.config/osr