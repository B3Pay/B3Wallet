import { IcrcLedgerCanister } from "@dfinity/ledger"
import { ICPToken, TokenAmount } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import { icAgent, localAgent } from "./actor"
import { accountIdentifier } from "./ledger.utils"
import { initIdentity } from "./utils"

const getBalance = async (
  mainnet: boolean,
  system: boolean,
  account?: string
) => {
  const agent = await (mainnet ? icAgent : localAgent)()

  const ledger = IcrcLedgerCanister.create({
    agent,
    canisterId: Principal.fromText(
      mainnet
        ? process.env.LEDGER_CANISTER_ID_MAINNET!
        : process.env.LEDGER_CANISTER_ID_LOCAL!
    )
  })

  let owner: any

  if (account === undefined) {
    owner = system
      ? Principal.fromText(
          mainnet
            ? process.env.SYSTEM_CANISTER_ID_MAINNET!
            : process.env.SYSTEM_CANISTER_ID_LOCAL!
        )
      : initIdentity(mainnet).getPrincipal()
  } else {
    owner = Principal.fromText(account)
  }

  const e8sBalance = await ledger.balance({
    owner,
    certified: false
  })

  const E8S_PER_ICP = 100_000_000n

  const formatE8sICP = (balance: bigint) => `${balance / E8S_PER_ICP} ICP`

  const token = TokenAmount.fromE8s({ amount: e8sBalance, token: ICPToken })

  console.log(formatE8sICP(token.toE8s()), "|", e8sBalance)

  const identifier = accountIdentifier(mainnet, owner)

  console.log(identifier.toHex())
}

const mainnet =
  process.argv.find(arg => arg.indexOf("--mainnet") > -1) !== undefined
const system =
  process.argv.find(arg => arg.indexOf("--system") > -1) !== undefined

const account = process.argv.find(arg => arg.indexOf("--account=") > -1)

getBalance(mainnet, system, account?.replace("--account=", ""))
