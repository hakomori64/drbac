.PHONY: test_build
test_build:
	cargo build
	cp target/debug/client test/entity/client
	cp target/debug/listener test/entity/listener
	mkdir -p test/entity/roles
	cp target/debug/shell test/entity/roles/base_binary
	cp target/debug/server test/central/server

.PHONY: test_db_initialize
test_db_initialize:
	rm -f test/entity/sample.db
	rm -f test/central/sample.db
	cat common/migrations/2021-09-28-080036_create_delegations/up.sql | sqlite3 sample.db
	cp sample.db test/entity/
	cp sample.db test/central/
	rm sample.db

.PHONY: client
client:
	cargo build
	cp target/debug/client test/entity/client
	mkdir -p test/entity/roles
	cp target/debug/shell test/entity/roles/base_binary
	test/entity/client

.PHONY: server
server:
	cargo build
	cp target/debug/server test/central/server
	test/central/server

.PHONY: listener
listener:
	cargo build
	cp target/debug/listener test/entity/listener
	mkdir -p test/entity/roles
	cp target/debug/shell test/entity/roles/base_binary
	sudo test/entity/listener