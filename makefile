update-expect:
	UPDATE_EXPECT=1 cargo test

test: update-expect
	cargo test
