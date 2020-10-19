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
