export MAGICK_VERSION=7.1

sudo apt-get update
sudo apt-get -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev

curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz
cd ImageMagick-${MAGICK_VERSION}*
./configure --with-magick-plus-plus=no --with-perl=no
make
sudo make install
cd ..
rm -r ImageMagick-${MAGICK_VERSION}*
