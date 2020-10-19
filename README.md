# thecat
The cat manages windows for me.  Good cat.

# Setup Instructions
Since the cat talks to X11 there are a few C/C++ dependencies you need to have set up on your
machine in order to install it (with
`cargo install --branch dev --git https://github.com/dunmatt/thecat` ):

### Arch Based Distros
    sudo pacman -Syu pango libxcb

### Debian Based Distros
    sudo apt-get install -y libpango* libxcb-randr0*

Depending on what (if any) desktop manager you use you may need to create a `.desktop` file before it will allow you to launch the cat.  The file path may depend on your set up, but mine is as follows:

/usr/share/xsessions/thecat.desktop:

    \[Desktop Entry]
    Name=The Cat
    Comment=A simple, and happy, window manager with good reflexes.
    TryExec=ls
    Exec=/home/matt/.cargo/bin/thecat
    Type=Application
