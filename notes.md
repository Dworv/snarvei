## Method to aquire keystrokes
It required admin permissions, so in the future it would be ideal to use 
alternate methods such as DE extensions. However, for now we use `input.rs`.
Libinput does a lot of the work for us.

## Method to export keystrokes
Not yet decided, however looking into `uinput`. 
https://www.kernel.org/doc/html/v4.12/input/uinput.html Also looking into 
`tfc`, seems promising.

`tfc` doesn't seem to have all the keys we need, may need to fork it or something.

## GUI
Will probably use GTK-rs, as this is intended for Linux desktops