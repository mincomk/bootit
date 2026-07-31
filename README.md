# bootit
A lightweight next boot selector

## What is Bootit?
Bootit is basically a CLI tool that makes your PC reboot to specific OS for one time.
```bash
sudo bootit scan # list IDs of bootable entries

sudo bootit alias add windows 1
sudo bootit alias add archlinux 2
```
Then, reboot to it with
```bash
sudo it windows
```
or
```bash
sudo it 2
```
or whatever. It does not change boot order.

If you want to use it without sudo, use the command below.

```bash
sudo bootit allow-non-root
```

From then on, you can use it without sudo.

```
it windows # No sudo required!
```

## Installation
### Cargo
```bash
cargo install bootit
```

## Troubleshooting

### `sudo: bootit: command not found` after `cargo install`

`cargo install` puts `bootit` in `~/.cargo/bin`, which is on your user PATH
but not on sudo's `secure_path` (the list of directories sudo searches when
you run `sudo <command>`). As a result, the first command fails and the
second one cannot even find the binary:

```bash
$ bootit scan
Error: This program must be run as root (try: sudo bootit ...)

$ sudo bootit scan
sudo: bootit: command not found
```

Run it with the full path instead:

```bash
sudo "$(which bootit)" scan
```

or link it into a system directory once so plain `sudo bootit` works:

```bash
sudo ln -s "$(which bootit)" /usr/local/bin/bootit
```

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests on the GitHub repository
