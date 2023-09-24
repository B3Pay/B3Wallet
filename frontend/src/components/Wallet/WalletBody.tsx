import {
  Accordion,
  AccordionItem,
  Stack,
  StackProps,
  Text
} from "@chakra-ui/react"
import { useCallback, useState } from "react"
import { Mode } from "."
import {
  WalletAccountView,
  WalletSettingsAndSigners
} from "../../../declarations/b3_wallet/b3_wallet.did"
import useToastMessage from "../../hooks/useToastMessage"
import { B3System, B3Wallet } from "../../service"
import Account from "./Account"
import CreateAccount from "./Account/CreateAccount"
import Logs from "./Logs"
import ProcessedList from "./ProcessedList"
import Settings from "./Setting"

interface Loadings {
  [key: string]: boolean
}

interface WalletBodyProps extends StackProps {
  mode: Mode
  actor: B3Wallet
  principal: string
  walletCanisterId: string
  settingsAndSigners: WalletSettingsAndSigners
  systemActor: B3System
  accounts: WalletAccountView[]
  setAccounts: React.Dispatch<React.SetStateAction<WalletAccountView[]>>
  refreshWallet: () => void
  fetchAccounts: () => void
}

const WalletBody: React.FC<WalletBodyProps> = ({
  mode,
  actor,
  accounts,
  principal,
  systemActor,
  setAccounts,
  fetchAccounts,
  refreshWallet,
  walletCanisterId,
  settingsAndSigners,
  ...rest
}) => {
  const [loading, setLoading] = useState<Loadings>({})
  const errorToast = useToastMessage()

  const refetchAccount = useCallback(
    async (account_id: string) => {
      console.log("refreshing account " + account_id)
      setLoading(prev => ({ ...prev, [account_id]: true }))
      actor
        .get_account_view(account_id)
        .then(account => {
          setAccounts(prev => {
            const index = prev.findIndex(a => a.id === account_id)

            if (index === -1) {
              return prev
            }

            prev[index] = account

            return [...prev]
          })

          setLoading(prev => ({ ...prev, [account_id]: false }))
        })
        .catch(e => {
          errorToast({
            title: "Error refreshing account.",
            description: e.message,
            status: "error",
            duration: 9000,
            isClosable: true
          })
          setLoading(prev => ({ ...prev, [account_id]: false }))
        })
    },
    [actor, setAccounts]
  )

  return (
    <Stack spacing={4} {...rest}>
      {mode === Mode.Settings ? (
        <Settings
          actor={actor}
          principal={principal}
          refreshWallet={refreshWallet}
          fetchAccounts={fetchAccounts}
          systemActor={systemActor}
          {...settingsAndSigners}
          setLoading={(global: boolean) =>
            setLoading(prev => ({ ...prev, global }))
          }
        />
      ) : mode === Mode.Processed ? (
        <ProcessedList
          actor={actor}
          setLoading={(global: boolean) =>
            setLoading(prev => ({ ...prev, global }))
          }
        />
      ) : mode === Mode.Logs ? (
        <Logs
          actor={actor}
          setLoading={(global: boolean) =>
            setLoading(prev => ({ ...prev, global }))
          }
        />
      ) : (
        <Stack spacing={4}>
          <Text fontSize="xl" fontWeight="bold">
            Accounts
          </Text>
          <CreateAccount actor={actor} fetchAccounts={fetchAccounts} />
          <Accordion allowMultiple>
            {accounts.map((account, index) => (
              <AccordionItem key={index} py={1} border="none">
                {({ isExpanded }) => (
                  <Account
                    key={index}
                    actor={actor}
                    loading={loading[account.id]}
                    isExpanded={isExpanded}
                    refetchAccount={() => refetchAccount(account.id)}
                    refetchAccounts={fetchAccounts}
                    {...account}
                  />
                )}
              </AccordionItem>
            ))}
          </Accordion>
        </Stack>
      )}
    </Stack>
  )
}

export default WalletBody
