import {
  Accordion,
  AccordionItem,
  Button,
  Container,
  Heading,
  Input,
  Stack,
  Text
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import Account from "components/Account"
import CreateAccount from "components/CreateAccount"
import { Footer } from "components/Footer"
import Loading from "components/Loading"
import RestoreAccount from "components/RestoreAccount"
import Status from "components/Status"
import { WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import useAuthClient from "hooks/useAuthClient"
import Head from "next/head"
import { useCallback, useEffect, useState } from "react"
import { B3User, makeB3UserActor } from "service/actor"

interface Loadings {
  global: boolean
  [key: string]: boolean
}

function HomePage() {
  const { isAuthenticated, authClient, login, logout, systemActor } =
    useAuthClient()

  const [loading, setLoading] = useState<Loadings>({
    global: false
  })
  const [error, setError] = useState<string>()
  const [version, setVersion] = useState<string>("")

  const [accounts, setAccounts] = useState<WalletAccountView[]>([])
  const [actor, setActor] = useState<B3User>()
  const [canisterId, setCanisterId] = useState<string>("")

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      if (!canisterId || !authClient) {
        console.log("no canisterId or authClient")
        return
      }
      const userActor = makeB3UserActor(canisterId, authClient.getIdentity())

      userActor
        .version()
        .then(async version => {
          setLoading(prev => ({ ...prev, global: true }))

          setVersion(version)
          setActor(userActor)

          setLoading(prev => ({ ...prev, global: false }))
        })
        .catch(e => {
          console.log(e)
          setLoading(prev => ({ ...prev, global: false }))
        })
    },
    [authClient]
  )

  const fetchCanisterId = useCallback(async () => {
    if (!systemActor || !authClient) {
      return
    }
    setLoading(prev => ({ ...prev, global: true }))

    systemActor
      .get_canister()
      .then(({ canister_id }) => {
        const canisterId = canister_id.toString()

        setCanisterId(canisterId)
        fetchUserActor(canisterId)
        setLoading(prev => ({ ...prev, global: false }))
      })
      .catch(e => {
        console.log(e)
        setLoading(prev => ({ ...prev, global: false }))
      })
  }, [authClient, systemActor, fetchUserActor])

  const fetchAccounts = useCallback(async () => {
    if (!actor) {
      console.log("no actor")
      return
    }
    setLoading(prev => ({ ...prev, global: true }))

    const accounts = await actor.get_account_views()

    setAccounts(accounts)
    setLoading(prev => ({ ...prev, global: false }))
  }, [actor])

  useEffect(() => {
    fetchAccounts()
  }, [fetchAccounts])

  useEffect(() => {
    fetchCanisterId()
  }, [fetchCanisterId])

  const createUser = async () => {
    setError(undefined)
    if (!systemActor || !authClient) {
      return
    }
    setLoading(prev => ({ ...prev, global: true }))
    systemActor.create_wallet_canister().then(async userControl => {
      if ("Err" in userControl) {
        setLoading(prev => ({ ...prev, global: false }))

        return console.log(userControl.Err)
      }

      fetchUserActor(userControl.Ok.canister_id.toString())
      setLoading(prev => ({ ...prev, global: false }))
    })
  }

  const installCanister = async () => {
    setError(undefined)
    if (!systemActor || !authClient) {
      return
    }
    setLoading(prev => ({ ...prev, global: true }))

    const canisterPrincipal = Principal.fromText(canisterId)

    const userControl = await systemActor.install_wallet_canister([
      canisterPrincipal
    ])

    if ("Err" in userControl) {
      setLoading(prev => ({ ...prev, global: false }))

      return setError(userControl.Err)
    }

    fetchUserActor(userControl.Ok.canister_id.toString())
    setLoading(prev => ({ ...prev, global: false }))
  }

  const refresh = useCallback(
    async (account_id: string) => {
      if (!actor) {
        console.log("no actor")
        return
      }
      setLoading(prev => ({ ...prev, [account_id]: true }))

      const account = await actor.get_account_view(account_id)

      setAccounts(prev => {
        const index = prev.findIndex(a => a.id === account_id)

        if (index === -1) {
          return prev
        }

        prev[index] = account

        return [...prev]
      })

      setLoading(prev => ({ ...prev, [account_id]: false }))
    },
    [actor]
  )

  const resetAccount = async () => {
    setError(undefined)
    if (!actor || !authClient) {
      console.log("no actor")
      return
    }

    setLoading(prev => ({ ...prev, global: true }))

    const result = await actor.reset_wallet()

    console.log(result)

    fetchAccounts()

    setLoading(prev => ({ ...prev, global: false }))
  }

  return (
    <Container maxW="2xl">
      <Head>
        <title>B3Wallet</title>
      </Head>
      <Heading textAlign="center" p={4}>
        B3Wallet Demo
      </Heading>
      <Stack as="main" position="relative" padding="2">
        {loading.global && <Loading dark />}
        {error && (
          <Text variant="error" textAlign="center" p={4}>
            {error}
          </Text>
        )}
        <Text variant="title" textAlign="center" p={4}>
          {canisterId.toString()}
        </Text>
        {!isAuthenticated ? (
          <Button onClick={login} colorScheme="green">
            Login
          </Button>
        ) : actor ? (
          <Stack overflow="hidden">
            <CreateAccount actor={actor} fetchAccounts={fetchAccounts} />
            <Accordion allowMultiple>
              {accounts.map((account, index) => (
                <AccordionItem padding="15px 0" position="relative" key={index}>
                  {({ isExpanded }) => (
                    <Account
                      key={index}
                      actor={actor}
                      isExpanded={isExpanded}
                      loading={loading[account.id]}
                      refresh={() => refresh(account.id)}
                      {...account}
                    />
                  )}
                </AccordionItem>
              ))}
            </Accordion>
            <RestoreAccount actor={actor} fetchAccounts={fetchAccounts} />
            <Button variant="solid" colorScheme="red" onClick={resetAccount}>
              Reset Account
            </Button>
            <Status actor={actor} />
          </Stack>
        ) : (
          <Stack>
            <Input
              type="text"
              placeholder="Enter Canister id"
              value={canisterId}
              onChange={e => setCanisterId(e.target.value)}
            />
            <Button onClick={installCanister}>Install Canister</Button>
            <Button onClick={createUser}>Create User</Button>
          </Stack>
        )}
        {isAuthenticated && (
          <Button
            colorScheme="red"
            onClick={() => {
              logout()
              window.location.reload()
            }}
          >
            Logout
          </Button>
        )}
      </Stack>
      <Footer
        actor={actor}
        authClient={authClient}
        version={version}
        setError={setError}
        setLoading={(global: boolean) =>
          setLoading(prev => ({ ...prev, global }))
        }
        setVersion={setVersion}
      />
    </Container>
  )
}

export default HomePage
