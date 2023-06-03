# rusty_hashcracker
A simple hash bruteforcer written in Rust (🚀 Blazingly fast 🚀)
autodetects these hash types, and uses the correct hashing function
- MD5
- SHA1
- SHA2 -> (SHA224, SHA256, SHA384, SHA512)

Simple salt support - (might get reworked)
- Currently the salt is simply `hash(text + salt)`

The wordlist i used is the basic "rockyou.txt" but any wordlist should work.
If you're on Kali there should be several on your system by default.

![Peek 2023-06-03 14-19](https://github.com/Sara0x62/rusty_hashcracker/assets/83826811/83e59954-ee72-41d2-951b-20200e15b646)

![2023-06-03_14-21](https://github.com/Sara0x62/rusty_hashcracker/assets/83826811/076054d3-a512-48c2-808e-0460e48bb8ad)

![2023-06-02_23-09](https://github.com/Sara-0x62/rusty_hashcracker/assets/83826811/6d561ec3-02aa-4c2c-80dc-b9591b273a29)

