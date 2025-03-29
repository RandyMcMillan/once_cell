run:cargo-b
	@$(MAKE) cargo-run-once-cell
	@$(MAKE) cargo-run-async-std-once-cell

cargo-b:
	@cargo -q b

cargo-run-once-cell:
	@cargo -q run --bin once_cell

cargo-run-async-std-once-cell:
	@cargo -q run --bin async_std_once_cell
