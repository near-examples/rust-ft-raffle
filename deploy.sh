#!/bin/bash
./build.sh
near dev-deploy res/prize.wasm
source neardev/dev-account.env
cat neardev/dev-account.env > .env

echo "Initializing prize contract '$CONTRACT_NAME'"
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "100000000", "metadata": { "spec": "ft-1.0.0", "name": "Prize Token", "symbol": "Prize", "decimals": 8 }}' --accountId $CONTRACT_NAME

echo "Creating alice account"
near create-account alice.$CONTRACT_NAME --masterAccount $CONTRACT_NAME --initialBalance "1"
near call $CONTRACT_NAME storage_deposit '' --accountId alice.$CONTRACT_NAME --amount 0.00125
near call $CONTRACT_NAME ft_transfer '{"receiver_id": "'alice.$CONTRACT_NAME'", "amount": "10"}' --accountId $CONTRACT_NAME --amount 0.000000000000000000000001

echo "Creating bob account"
near create-account bob.$CONTRACT_NAME --masterAccount $CONTRACT_NAME --initialBalance "1"
near call $CONTRACT_NAME storage_deposit '' --accountId bob.$CONTRACT_NAME --amount 0.00125
near call $CONTRACT_NAME ft_transfer '{"receiver_id": "'bob.$CONTRACT_NAME'", "amount": "10"}' --accountId $CONTRACT_NAME --amount 0.000000000000000000000001

echo "Balance of alice"
near view $CONTRACT_NAME ft_balance_of '{"account_id": "'alice.$CONTRACT_NAME'"}'
echo "Balance of bob"
near view $CONTRACT_NAME ft_balance_of '{"account_id": "'bob.$CONTRACT_NAME'"}'


RAFFLE_CONTRACT_NAME=raffle.$CONTRACT_NAME
echo "Creating '$RAFFLE_CONTRACT_NAME' account"

near create-account $RAFFLE_CONTRACT_NAME  --masterAccount $CONTRACT_NAME --initialBalance 50
near call $CONTRACT_NAME storage_deposit '' --accountId $RAFFLE_CONTRACT_NAME --amount 0.00125

echo "Deploying raffle contract '$RAFFLE_CONTRACT_NAME'"
near deploy $RAFFLE_CONTRACT_NAME res/raffle.wasm new '{"fungible_token_account_id": "'$CONTRACT_NAME'", "tokens_per_ticket": 5, "number_of_predefined":3}'

prize=$CONTRACT_NAME
raffle=$RAFFLE_CONTRACT_NAME

echo "prize='$CONTRACT_NAME'"
echo "raffle='$RAFFLE_CONTRACT_NAME'"