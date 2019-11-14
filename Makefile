EXE=rustdoku

INSTALLDIR=/usr/games/

build: main.rs
	rustc -O main.rs -o $(EXE)
  
install: build
	@echo You must be root to install
	if [ -d $(INSTALLDIR) ]; then sudo cp -f $(EXE) $(INSTALLDIR); fi
