import { IcrcLedgerCanister } from "@dfinity/ledger"
import { Principal } from "@dfinity/principal"
import { localAgent } from "./actor"
import { LEDGER_CANISTER_ID_LOCAL } from "./env"

const transfer = async (account: string) => {
  const agent = await localAgent()

  const ledger = IcrcLedgerCanister.create({
    agent,
    canisterId: LEDGER_CANISTER_ID_LOCAL as unknown as Principal
  })

  return ledger.transfer({
    amount: 5_500_000_000n,
    to: {
      owner: Principal.fromText(account ?? "qaa6y-5yaaa-aaaaa-aaafa-cai"),
      subaccount: []
    }
  })
}

const account = process.argv[2]

transfer(account).then(() => console.log("Transfer complete"))
