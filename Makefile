VENV=.venv

ifeq ($(OS),Windows_NT)
	VENV_BIN=$(VENV)/Scripts
else
	VENV_BIN=$(VENV)/bin
endif

.venv:
	python3 -m venv $(VENV)
	$(MAKE) requirements

.PHONY: requirements
requirements: .venv
	$(VENV_BIN)/python -m pip install --upgrade uv \
	&& $(VENV_BIN)/uv pip install -r py-psykan/requirements-dev.txt

.PHONY: build
build: .venv
	$(VENV_BIN)/maturin develop -m py-psykan/Cargo.toml
