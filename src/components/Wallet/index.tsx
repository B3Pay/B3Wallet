import { Accordion, AccordionItem, Box, Stack, Text } from "@chakra-ui/react"
import { WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../Loading"
import Account from "./Account"
import CreateAccount from "./CreateAccount"
import WalletHeader from "./Header"
import Settings from "./Setting"

interface Loadings {
  global: boolean
  [key: string]: boolean
}
interface WalletProps {
  actor: B3Wallet
  version: string
  walletCanisterId: string
}

const Wallet: React.FC<WalletProps> = ({
  actor,
  version,
  walletCanisterId
}) => {
  const [settings, setSettings] = useState<boolean>(false)

  const [accounts, setAccounts] = useState<WalletAccountView[]>([])
  const [loading, setLoading] = useState<Loadings>({
    global: true
  })

  const fetchAccounts = useCallback(async () => {
    if (!actor) {
      console.log("no actor")
      return
    }

    console.log("fetching accounts")
    setLoading(prev => ({ ...prev, global: true }))

    actor
      .get_account_views()
      .then(accounts => {
        setAccounts(accounts)
        setLoading(prev => ({ ...prev, global: false }))
      })
      .catch(e => {
        console.log(e)
        setLoading(prev => ({ ...prev, global: false }))
      })
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

  const refetchAccount = useCallback(
    async (account_id: string) => {
      if (!actor) {
        console.log("no actor")
        return
      }

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
          console.log(e)
          setLoading(prev => ({ ...prev, [account_id]: false }))
        })
    },
    [actor]
  )

  return actor ? (
    <Stack position="relative" spacing={6} width="100%">
      {loading.global && <Loading title="Wallet Loading" />}
      <WalletHeader
        actor={actor}
        walletCanisterId={walletCanisterId}
        toggleSetting={() => setSettings(prev => !prev)}
      />
      {settings ? (
        <Settings
          actor={actor}
          version={version}
          fetchAccounts={fetchAccounts}
          setLoading={(global: boolean) =>
            setLoading(prev => ({ ...prev, global }))
          }
        />
      ) : (
        <Accordion allowMultiple>
          <Stack spacing={4}>
            <Text fontSize="xl" fontWeight="bold">
              Accounts
            </Text>
            <CreateAccount actor={actor} fetchAccounts={fetchAccounts} />
            <Box>
              {accounts.map((account, index) => (
                <AccordionItem paddingY={4} key={index}>
                  {({ isExpanded }) => (
                    <Account
                      key={index}
                      actor={actor}
                      isExpanded={isExpanded}
                      loading={loading[account.id]}
                      refresh={() => refetchAccount(account.id)}
                      {...account}
                    />
                  )}
                </AccordionItem>
              ))}
            </Box>
          </Stack>
        </Accordion>
      )}
    </Stack>
  ) : (
    <Loading title="Wallet Loading" />
  )
}

export default Wallet
