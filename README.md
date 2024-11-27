## Source
[Source 1](https://kmsorsms.github.io/embassy_preempt/docs/%E5%BC%80%E5%8F%91%E8%AE%B0%E5%BD%95.html#%E7%A5%9E%E5%A5%87%E7%9A%84ffi)
[Source 2](https://kmsorsms.github.io/embassy_preempt/docs/%E6%8A%80%E6%9C%AF%E6%8A%A5%E5%91%8A.html#aapcs%E4%B8%8Effi)
## Reproduce
- [Raspberry Pi running in QEMU](https://github.com/matteocarnelos/dockerpi).
```shell
rustc --crate-type=staticlib callee.rs -O
gcc caller.c -c -mabi=aapcs -O2
gcc --static caller.o libcallee.a -mabi=aapcs -O2
objdump -d a.out > result.S
```

## Cleanup
```shell
rm a.out caller.o libcallee.a result.S
```
