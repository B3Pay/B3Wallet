import { Principal } from "@dfinity/principal"
export const PROTOCOLS = {
  volt: "aclt4-uaaaa-aaaak-qb4zq-cai"
}
export const toHexString = (
  byteArray: Iterable<unknown> | ArrayLike<unknown>
) => {
  return Array.from(byteArray, byte =>
    ("0" + (byte & 0xff).toString(16)).slice(-2)
  ).join("")
}
export const fromHexString = (hex: string) => {
  if (hex.substr(0, 2) === "0x") hex = hex.substr(2)
  for (var bytes = [], c = 0; c < hex.length; c += 2)
    bytes.push(parseInt(hex.substr(c, 2), 16))
  return bytes
}
export const getSubaccountFromHex = (hex: any) => {
  const dec = fromHexString(hex)
  return Array(32 - dec.length)
    .fill(0)
    .concat(dec)
}

export const IcrcAccountFromAddress = (address: string) => {
  const decoded = address.split(":")
  const principalText = protocols.hasOwnProperty(decoded[0])
    ? PROTOCOLS[decoded[0]]
    : decoded[0]
  const subaccount =
    decoded.length > 1 ? [getSubaccountFromHex(decoded[1])] : []
  return { owner: Principal.fromText(principalText), subaccount: subaccount }
}

export const isValidIcrcAddress = (address: string) => {
  const decoded = address.split(":")
  try {
    if (decoded.length > 2) return false
    if (!PROTOCOLS.hasOwnProperty(decoded[0])) Principal.fromText(decoded[0])
    if (decoded.length > 1 && decoded[1] && !isHex(decoded[1])) return false
  } catch (e) {
    return false
  }
  return true
}

// Examples

console.log(IcrcAccountFromAddress("volt"))
console.log(IcrcAccountFromAddress("volt:0"))
console.log(IcrcAccountFromAddress("volt:"))
console.log(
  IcrcAccountFromAddress(
    "volt:e5c6a05f916d8408d4bc3f8a6e920cf9330ad3344d5505c534e6048e02"
  )
)
