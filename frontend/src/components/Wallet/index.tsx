import { Card } from "@chakra-ui/react"
import {
  WalletAccountView,
  WalletSettingsAndSigners
} from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3System, B3Wallet } from "service/actor"
import Loading from "../Loading"
import InitialSetup from "./InitialSetup"
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
  const [settingAndSigners, setSttingAndSigners] =
    useState<WalletSettingsAndSigners>()

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

  const fetchSettingsAndSigners = useCallback(async () => {
    console.log("fetching settings")
    setLoading(true)

    actor
      .setting_and_signer()
      .then(setting => {
        console.log(setting)
        setSttingAndSigners(setting)
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

  const refreshWallet = async () => {
    await fetchAccounts()
    await fetchSettingsAndSigners()
  }

  useEffect(() => {
    fetchSettingsAndSigners()
    fetchAccounts()
  }, [])

  return (
    <Card
      position="relative"
      padding={2}
      width="100%"
      height="100%"
      justify="space-between"
    >
      {loading && <Loading title="Loading Wallet" />}
      {accounts.length === 0 ? (
        <InitialSetup
          actor={actor}
          {...settingAndSigners}
          fetchAccounts={fetchAccounts}
          fetchSettingsAndSigners={fetchSettingsAndSigners}
        />
      ) : (
        <>
          <WalletHeader
            flex={1}
            mode={mode}
            actor={actor}
            refreshWallet={refreshWallet}
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
            refreshWallet={refreshWallet}
            fetchAccounts={fetchAccounts}
            walletCanisterId={walletCanisterId}
            settingsAndSigners={settingAndSigners}
          />
        </>
      )}
    </Card>
  )
}

export default Wallet
