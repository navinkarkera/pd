# PD: A dead simple secure password vault

A dead simple password vault cli application. This is my first Rust project so the code might not be very idiomatic.
Feel free to provide feedback and suggestions by opening up issues.

**Password vault is secured using [zbox](https://github.com/zboxfs/zbox)**

## Disclaimer

I am not responsible for any loss in data or leak caused by using it. Always back up your files and use at your own risk!

Features
====

- Each site is stored as a separate file in zbox file system
- Add or update site details using `add` command
- List all the sites
- Get details for a single site using `get`
- Setup master password and initialize zbox using `init`

License
=======
`pd` is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE)
file for details.
