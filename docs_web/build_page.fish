#!/usr/bin/fish

function build_page
    rm -v -r ../docs
    mkdir ../docs
    flutter build web -v
    cp -r -v ./build/web ../docs
end