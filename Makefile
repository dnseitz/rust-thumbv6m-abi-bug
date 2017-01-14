

all: debug

debug:
	@xargo build --target=thumbv6m-none-eabi
	@arm-none-eabi-ld -n --gc-sections -T link.ld target/thumbv6m-none-eabi/debug/liblmul_test.a

