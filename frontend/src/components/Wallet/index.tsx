import { Card } from "@chakra-ui/react"
import { WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3System, B3Wallet } from "service/actor"
import Loading from "../Loading"
import WalletBody from "./WalletBody"
import WalletHeader from "./WalletHeader"

interface WalletProps {
  actor: B3Wallet
  systemActor: B3System
  walletCanisterId: string
}

export enum Mode {
  Processed,
  Settings,
  Accounts
}

const Wallet: React.FC<WalletProps> = ({
  actor,
  systemActor,
  walletCanisterId
}) => {
  const [mode, setMode] = useState<Mode>(Mode.Accounts)

  const [loading, setLoading] = useState(false)
  const [accounts, setAccounts] = useState<WalletAccountView[]>([])
  const errorToast = useToastMessage()

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
        errorToast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoading(false)
      })
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

  return (
    <Card
      position="relative"
      padding={2}
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
        accounts={accounts}
        systemActor={systemActor}
        setAccounts={setAccounts}
        fetchAccounts={fetchAccounts}
      />
    </Card>
  )
}

export default Wallet
