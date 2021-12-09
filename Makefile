.PHONY: test_build
test_build:
	cargo build
	cp target/debug/client test/entity/client
	cp target/debug/listener test/entity/listener
	mkdir -p test/entity/roles
	cp target/debug/shell test/entity/roles/base_binary
	cp target/debug/server test/central/server

.PHONY: deploy_to_vms
deploy_to_vms:
	cargo build --release --target=x86_64-unknown-linux-musl
	mkdir -p ../host1/roles
	mkdir -p ../host2/roles
	mkdir -p ../host3/roles
	cp target/x86_64-unknown-linux-musl/release/client ../host1/
	cp target/x86_64-unknown-linux-musl/release/listener ../host1/
	cp target/x86_64-unknown-linux-musl/release/client ../host2/
	cp target/x86_64-unknown-linux-musl/release/server ../host3/
	cp target/x86_64-unknown-linux-musl/release/shell ../host1/roles/base_binary
	cp target/x86_64-unknown-linux-musl/release/shell ../host2/roles/base_binary

.PHONY: test_db_initialize
test_db_initialize:
	rm -f test/entity/sample.db
	rm -f test/central/sample.db
	cat common/migrations/2021-09-28-080036_create_delegations/up.sql | sqlite3 sample.db
	cp sample.db test/entity/
	cp sample.db test/central/
	rm sample.db

.PHONY: build_client
build_client:
	rm -rf build
	mkdir -p build/roles
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/client build/
	cp target/x86_64-unknown-linux-musl/release/shell build/roles/base_binary

.PHONY: build_listener
build_listener:
	rm -rf build
	mkdir -p build/roles
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/listener build/
	cp target/x86_64-unknown-linux-musl/release/shell build/roles/base_binary

.PHONY: build_server
build_server:
	rm -rf build
	mkdir -p build
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/server build/

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
