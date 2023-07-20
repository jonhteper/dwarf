dir=$(pwd)
pkgname="dwarf"

sudo install -Dm755 "$dir/dwarf" "/usr/bin/$pkgname"

sudo install -Dm644 LICENSE "/usr/share/licenses/$pkgname/LICENSE"

sudo install -Dm644 icon.png "/usr/share/$pkgname/icon.png"

sudo install -Dm644 "$pkgname.desktop" "/usr/share/applications/$pkgname.desktop"

