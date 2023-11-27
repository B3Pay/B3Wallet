import { AccountIdentifier } from "@dfinity/ledger-icp"
import { Principal } from "@dfinity/principal"
import { initIdentity } from "./utils"

export const accountIdentifier = (mainnet: boolean, principal?: Principal) => {
  const identity = initIdentity(mainnet)

  return AccountIdentifier.fromPrincipal({
    principal: principal ?? (identity.getPrincipal() as any),
    subAccount: undefined
  })
}
