import { Card } from "@chakra-ui/react"
import {
  WalletAccountView,
  WalletSettingsAndSigners
} from "declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "hooks/useToastMessage"
import { useRouter } from "next/router"
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
  Accounts,
  Initial
}

const Wallet: React.FC<WalletProps> = ({
  actor,
  systemActor,
  walletCanisterId
}) => {
  const [mode, setMode] = useState<Mode>(Mode.Initial)

  const [loading, setLoading] = useState(false)
  const [accounts, setAccounts] = useState<WalletAccountView[]>([])
  const [settingAndSigners, setSttingAndSigners] =
    useState<WalletSettingsAndSigners>()

  const errorToast = useToastMessage()
  const router = useRouter()

  const fetchAccounts = useCallback(() => {
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

  const fetchSettings = useCallback(() => {
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
    router.push(router.asPath)
  }

  useEffect(() => {
    fetchSettings()
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
        <InitialSetup actor={actor} {...settingAndSigners} />
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
            setting={settingAndSigners}
            accounts={accounts}
            systemActor={systemActor}
            setAccounts={setAccounts}
            refreshWallet={refreshWallet}
            fetchAccounts={fetchAccounts}
            walletCanisterId={walletCanisterId}
          />
        </>
      )}
    </Card>
  )
}

export default Wallet
