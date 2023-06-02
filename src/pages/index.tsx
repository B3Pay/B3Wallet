import { Button, Card, Container, Stack, Text } from "@chakra-ui/react"
import Disclaimer from "components/Disclaimer"
import { Footer } from "components/Footer"
import Header from "components/Header"
import Loading from "components/Loading"
import System from "components/System"
import Wallet from "components/Wallet"
import useAuthClient from "hooks/useAuthClient"
import useToastMessage from "hooks/useToastMessage"
import Head from "next/head"
import { useCallback, useState } from "react"
import { B3Wallet, makeB3WalletActor } from "service/actor"

function HomePage() {
  const {
    isAuthenticated,
    isAuthenticating,
    authClient,
    login,
    logout,
    systemActor
  } = useAuthClient()

  const [loading, setLoading] = useState(false)
  const [walletCanisterId, setWalletCanisterId] = useState<string>("")
  const [walletActor, setWalletActor] = useState<B3Wallet>()
  const [version, setVersion] = useState<string>("")

  const toast = useToastMessage()

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      if (!authClient || !canisterId) return
      setWalletCanisterId(canisterId)

      const userActor = makeB3WalletActor(canisterId, authClient.getIdentity())

      console.log("fetching user actor")
      setLoading(true)

      userActor
        .version()
        .then(async version => {
          console.log("user actor version", version)
          setVersion(version)
          setWalletActor(userActor)
          setLoading(false)
        })
        .catch(e => {
          toast({
            title: "Error",
            description: e.message,
            status: "error",
            duration: 5000,
            isClosable: true
          })

          setWalletCanisterId("")
          setLoading(false)
        })
    },
    [authClient, toast]
  )

  return (
    <Container maxW="2xl" p={1}>
      <Head>
        <title>B3Wallet</title>
      </Head>
      <Header />
      <Stack as="main" minH="100px" position="relative" justify="space-between">
        {isAuthenticating && <Loading title="Authenticating" />}
        {loading && <Loading title="Loading Wallet" />}
        {isAuthenticated ? (
          walletActor ? (
            <Wallet
              actor={walletActor}
              walletCanisterId={walletCanisterId}
              version={version}
            />
          ) : systemActor ? (
            <System systemActor={systemActor} fetchUserActor={fetchUserActor} />
          ) : (
            <Loading dark title="Fetching" />
          )
        ) : (
          <Stack spacing="2">
            <Card p={2}>
              <Disclaimer noTitle />
            </Card>
            <Card p={2}>
              <Stack>
                <Text fontSize="sm">Connect your wallet to get started</Text>
                <Button onClick={login} colorScheme="green">
                  Login
                </Button>
              </Stack>
            </Card>
          </Stack>
        )}
      </Stack>
      {isAuthenticated && (
        <Card p={2} mt={2}>
          <Stack spacing="4">
            <Button
              variant="solid"
              colorScheme="red"
              onClick={() => {
                logout()
                window.location.reload()
              }}
            >
              Logout
            </Button>
          </Stack>
        </Card>
      )}
      <Footer />
    </Container>
  )
}

export default HomePage
