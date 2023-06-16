if [ ! -d "./candid" ]; then
    dfx nns import
fi

dfx nns install

# get --enable-bitcoin from args
if [[ $* == *--enable-bitcoin* ]]; then
    sh scripts/ckbtc.sh
    sh scripts/kyt.sh
    sh scripts/minter.sh
    sh scripts/index.sh
fi
