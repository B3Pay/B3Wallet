import { Stack } from "@chakra-ui/react"
import { WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../Loading"
import WalletBody from "./Body"
import WalletHeader from "./Header"

interface WalletProps {
  actor: B3Wallet
  version: string
  walletCanisterId: string
}

export enum Mode {
  Processed,
  Settings,
  Accounts
}

const Wallet: React.FC<WalletProps> = ({
  actor,
  version,
  walletCanisterId
}) => {
  const [mode, setMode] = useState<Mode>(Mode.Accounts)

  const [loading, setLoading] = useState(false)
  const [accounts, setAccounts] = useState<WalletAccountView[]>([])

  const fetchAccounts = useCallback(async () => {
    console.log("fetching accounts")
    setLoading(true)

    actor
      .get_account_views()
      .then(accounts => {
        setAccounts(accounts)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

  return (
    <Stack
      position="relative"
      spacing={6}
      width="100%"
      height="100%"
      justify="space-between"
    >
      {loading && <Loading title="Loading Wallet" />}
      <WalletHeader
        flex={1}
        mode={mode}
        actor={actor}
        walletCanisterId={walletCanisterId}
        fetchAccounts={fetchAccounts}
        toggleMode={Mode => setMode(Mode)}
      />
      <WalletBody
        flex={11}
        mode={mode}
        actor={actor}
        version={version}
        accounts={accounts}
        setAccounts={setAccounts}
        fetchAccounts={fetchAccounts}
      />
    </Stack>
  )
}

export default Wallet
