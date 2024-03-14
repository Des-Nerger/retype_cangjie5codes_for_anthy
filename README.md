# retype_cangjie5codes_for_anthy
```sh
$ sudo apt install libxdo-dev
$ cargo build --release
$ sleep 15 && target/release/retype_cangjie5codes_for_anthy </path/to/libcangjie-1.2/data/table.txt
```
* Quickly (in 15 secs):
  * open a new terminal window,
  * start `$ cat >>~/.anthy/private_words_default` in it, and
  * switch over the input method to e.g. `fcitx5-anthy` by pressing `Ctrl+Space`.
* Wait a little more than an hour, before it finishes retyping all the Cangjie dictionary, then input `Ctrl+D` to both close the input stream and quit `cat`.
* Do some fixes to the result: `$ sed -i 's/ $/ 鱫/; s/ヴ/う゛/g' ~/.anthy/private_words_default`.
* The performance of recent versions of the `kasumi` GUI tool, when resaving a big dictionary, is quite awful, even impractical to do on HDD (and I don't recommend anyone to wear out their SSDs, either). So move the anthy directory onto a ramdisk first and symlink it: `$ mv ~/.anthy /dev/shm && ln -sv {/dev/shm/,~/}.anthy`.
* Run `$ kasumi` itself, press `Save`, and when it finishes, press `Close`.
* Get the anthy directory back in its place: `$ gio trash ~/.anthy && mv /dev/shm/.anthy ~`.
