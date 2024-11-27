## Source
[Source 1](https://kmsorsms.github.io/embassy_preempt/docs/%E5%BC%80%E5%8F%91%E8%AE%B0%E5%BD%95.html#%E7%A5%9E%E5%A5%87%E7%9A%84ffi)  
[Source 2](https://kmsorsms.github.io/embassy_preempt/docs/%E6%8A%80%E6%9C%AF%E6%8A%A5%E5%91%8A.html#aapcs%E4%B8%8Effi)

## Reproduce
- Environment: [Raspberry Pi 2 running in qemu by docker](https://github.com/matteocarnelos/dockerpi).

```shell
# the following is identical to `test.sh`
rustc --crate-type=staticlib callee.rs -O
gcc caller.c -c -mabi=aapcs -O2
gcc --static caller.o libcallee.a -mabi=aapcs -O2
objdump -d a.out > result.S
```

Running the output program will lead to a `Segmentation fault` when calling `func_from_rust1`.

## Analysis
First let's take a look at [Procedure Call Standard for Arm Architecture (AAPCS)](https://github.com/ARM-software/abi-aa/releases/download/2024Q3/aapcs32.pdf)
> 6.4 Result Return  
> The manner in which a result is returned from a function is determined by the type of that result.  
> For the base standard:
> - A Half-precision Floating Point Type is returned in the least significant 16 bits of r0.
> - A Fundamental Data Type that is smaller than 4 bytes is zero- or sign-extended to a word and
    returned in r0.
> - A word-sized Fundamental Data Type (e.g., int, float) is returned in r0.
> - A double-word sized Fundamental Data Type (e.g., long long, double and 64-bit containerized
    vectors) is returned in r0 and r1.
> - A 128-bit containerized vector is returned in r0-r3.
> - A Composite Type not larger than 4 bytes is returned in r0. The format is as if the result had been
    stored in memory at a word-aligned address and then loaded into r0 with an LDR instruction. Any
    bits in r0 that lie outside the bounds of the result have unspecified values.
> - A Composite Type larger than 4 bytes, or whose size cannot be determined statically by both caller
    and callee, is stored in memory at an address passed as an extra argument when the function was
    called (Parameter Passing (base PCS), Rule A.4). The memory to be used for the result may be
    modified at any point during the function call.


Then let's dive into the rust code.

```rust
#[repr(align(8))]
#[repr(C)]
pub enum TEST1 { ... }

#[repr(align(4))]
#[repr(C)]
pub enum TEST2 { ... }

#[no_mangle]
pub extern "aapcs" fn func_from_rust1() -> TEST1 { return TEST1::C; }

#[no_mangle]
pub extern "aapcs" fn func_from_rust2() -> TEST2 { return TEST2::C; }

#[no_mangle]
pub extern "aapcs" fn func_from_rust3() -> u64 { return 0xfffffffff; } // 2 ^ 36 - 1
```

`func_from_rust1` and `func_from_rust2` both returns an enum, aligned to 8 and 4 bytes respectively.   
And `func_from_rust1` return a `u64`.

```asm
00010534 <func_from_rust1>:
   10534:	e3a01002 	mov	r1, #2
   10538:	e5801000 	str	r1, [r0]
   1053c:	e12fff1e 	bx	lr

00010540 <func_from_rust2>:
   10540:	e3a00002 	mov	r0, #2
   10544:	e12fff1e 	bx	lr

00010548 <func_from_rust3>:
   10548:	e3e00000 	mvn	r0, #0
   1054c:	e3a0100f 	mov	r1, #15
   10550:	e12fff1e 	bx	lr
```

In the assembly, 
- `TEST2` is 4 bytes so it is stored in a register(`r0`), which is as expected. (`function_from_rust2`)      
- `u64` is `A double-word sized Fundamental Data Type`, so it is stored in `r0` and `r1`.  (`function_from_rust3`)      
- `TEST1` is stored at an address specified by `r0` (`function_from_rust1`)

It might be strange at first that `TEST1` and `u64` is stored in different places.  It seems that the rust compiler treat `TEST1` as `A Composite Type larger than 4 bytes`. And according to the last rule in `AAPCS`, it should be stored in memory.

But in C, `enum` with a size of 8 bytes is returned just as the `uint64_t`, which conflicts with rust. That is probably why the call to `func_from_rust1` will lead to a `Segmentation fault`.

As it is stated in [The Rust Reference](https://rustwiki.org/en/reference/type-layout.html#reprc-field-less-enums): 
> Warning:  
> There are crucial differences between an `enum` in the C language and Rust's [field-less enums](https://rustwiki.org/en/reference/items/enumerations.html#field-less-enum) with this representation. An `enum` in C is mostly a `typedef` plus some named constants; in other words, an object of an `enum` type can hold any integer value. For example, this is often used for bitflags in `C`. In contrast, Rust’s [field-less enums](https://rustwiki.org/en/reference/items/enumerations.html#field-less-enum) can only legally hold the discriminant values, everything else is [undefined behavior](https://rustwiki.org/en/reference/behavior-considered-undefined.html). Therefore, using a field-less enum in FFI to model a C `enum` is often wrong.
