if [ ! -d "./candid" ]; then
    dfx nns import
fi

dfx nns install

# get --enable-bitcoin from args
if [[ $* == *--enable-bitcoin* ]]; then
    sh scripts/ckbtc.sh
fi

dfx identity use default