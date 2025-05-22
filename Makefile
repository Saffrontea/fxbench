RUNS := 30
BENCH := hash_bench
RESULT_DIR := results

.PHONY: all clean bench

all: bench

bench:
	@mkdir -p $(RESULT_DIR)
	@for i in $(shell seq -w 1 $(RUNS)); do \
		echo "==> Run $$i"; \
		cargo bench --bench $(BENCH); \
		mkdir -p $(RESULT_DIR)/run_$$i; \
		cp -r target/criterion/* $(RESULT_DIR)/run_$$i/; \
	done

clean:
	rm -rf $(RESULT_DIR)/run*/
	rm -rf target/criterion
	cargo clean

