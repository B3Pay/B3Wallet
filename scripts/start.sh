if [[ $* == *--enable-bitcoin* ]]; then
    dfx start --enable-bitcoin --clean
else
    dfx start --clean
fi