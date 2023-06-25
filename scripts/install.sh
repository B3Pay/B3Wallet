if [ ! -d "./candid" ]; then
    dfx nns import
fi

dfx nns install

sh scripts/ckbtc.sh
sh scripts/kyt.sh
sh scripts/minter.sh
sh scripts/index.sh
