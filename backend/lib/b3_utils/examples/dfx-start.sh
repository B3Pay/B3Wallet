if [[ $* == *--use-old-metering* ]]; then
    dfx start --use-old-metering --clean --background
else
    dfx start --clean --background
fi

dfx deploy vetkd_system_api --specified-id wfdtj-lyaaa-aaaap-abakq-cai