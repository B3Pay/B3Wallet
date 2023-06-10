import { computeAddress, getCreateAddress } from "ethers"

let publicKey =
  "0x02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1"

let from = computeAddress(publicKey)
console.log(from)

let expectedAddress = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66"

console.log(from === expectedAddress)

let nonce = 0
let expectedResult = "0x0407316cB70D5a7D4642B592e9CB37Fa70c56CD1"
let result = getCreateAddress({ from, nonce })

console.log(result)
console.log(result === expectedResult)

nonce = 1
expectedResult = "0xa871C4B1DC678bE80AF6b5cc8AA4910Ad62b11Cb"
result = getCreateAddress({ from, nonce })

console.log(result)
console.log(result === expectedResult)
