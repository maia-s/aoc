DAYS_SRC=$(wildcard src/bin/day-*.rs)
DAYS=$(DAYS_SRC:src/bin/%.rs=%)

test:
	@cargo test --release --no-fail-fast

run:
	$(foreach day, $(DAYS), cargo run --release --bin $(day);)

.PHONY: run test
