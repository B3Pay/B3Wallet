if [[ $* == *--enable-bitcoin* ]]; then
    dfx start --enable-bitcoin --background --clean
else
    dfx start --background --clean
fi

if [ ! -d "./candid" ]; then
    dfx nns import
fi

dfx nns install

# get --enable-bitcoin from args
if [[ $* == *--enable-bitcoin* ]]; then
    sh scripts/ckbtc.sh
fi
