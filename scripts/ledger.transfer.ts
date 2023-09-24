import { IcrcLedgerCanister } from "@dfinity/ledger"
import { Principal } from "@dfinity/principal"
import { localAgent } from "./actor"
import { NNS_LEDGER } from "./env"

const transfer = async (account: string) => {
  console.log(
    "Transfering 5.5 ICP to",
    account ?? "qaa6y-5yaaa-aaaaa-aaafa-cai"
  )

  const agent = await localAgent()

  const ledger = IcrcLedgerCanister.create({
    agent,
    canisterId: Principal.fromText(NNS_LEDGER)
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

transfer(account).then(block_number => console.log("Transfered ", block_number))
