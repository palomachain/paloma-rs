# Limit Order Bot CosmWasm Smart Contract for Paloma

This is a CosmWasm smart contract to manage limit-orders on Ethereum limit-order-bot smart contract in Vyper.

Users can deposit Eth into Vyper smart contract on Ethereum.

The smart contract will add liquidity to expected price between current price.

This deposit requires Eth only.

When Eth price gets the expected price, the liquidity is withdrawn to the user.

So the users can get desired amount of USDC and additional some USDC and Eth from liquidity fee.

## ExecuteMsg

### GetDeposit

Get deposit information from Vyper contract.

| Key            | Type    | Description                         |
|----------------|---------|-------------------------------------|
| token_id       | u128    | Uniswap V3 NFLP token_id            |
| sqrt_price_x96 | Uint256 | sqrtpricex96 value of the liquidity |
| deadline       | u64     | deadline of limit-order             |

### PutWithdraw

Run withdraw transaction for orders that reaches to the expected price to Vyper contract via pigeons.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |


### PutCancel

Run cancel transaction for orders that reaches to the deadline to Vyper contract via pigeons.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

### GetWithdraw

Get withdraw / cancel information from Vyper contract.

| Key       | Type      | Description                         |
|-----------|-----------|-------------------------------------|
| token_ids | Vec<u128> | withdrawn or canceled token id list |

## QueryMsg

### DepositList

Get deposited token_id list.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key           | Type      | Description                   |
|---------------|-----------|-------------------------------|
| list          | Vec<u128> | Uniswap V3 NFLP token_id list |

### WithdrawableList

Get token_id list that reach to expected price.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key           | Type      | Description                   |
|---------------|-----------|-------------------------------|
| list          | Vec<u128> | Uniswap V3 NFLP token_id list |


### CancelableList

Get token_id list that reach to deadline.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key           | Type      | Description                   |
|---------------|-----------|-------------------------------|
| list          | Vec<u128> | Uniswap V3 NFLP token_id list |

