import { IcrcLedgerCanister } from "@dfinity/ledger"
import { ICPToken, TokenAmount } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import { icAgent, localAgent } from "./actor"
import {
  CONSOLE_CANISTER_ID_LOCAL,
  CONSOLE_CANISTER_ID_MAINNET,
  LEDGER_CANISTER_ID_LOCAL,
  LEDGER_CANISTER_ID_MAINNET
} from "./env"
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
      mainnet ? LEDGER_CANISTER_ID_MAINNET : LEDGER_CANISTER_ID_LOCAL
    )
  })

  let owner: Principal

  if (account === undefined) {
    owner = system
      ? Principal.fromText(
          mainnet ? CONSOLE_CANISTER_ID_MAINNET : CONSOLE_CANISTER_ID_LOCAL
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
