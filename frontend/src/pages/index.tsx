import {
  Button,
  Card,
  Container,
  Stack,
  Text,
  useToast
} from "@chakra-ui/react"
import Footer from "components/Footer"
import Head from "next/head"
import { useCallback, useState } from "react"
import Disclaimer from "../components/Disclaimer"
import Header from "../components/Header"
import Loading from "../components/Loading"
import System from "../components/System"
import Wallet from "../components/Wallet"
import useAuthClient from "../hooks/useAuthClient"
import { B3Wallet, createB3WalletActor } from "../service/"

function HomePage() {
  const {
    isAuthenticated,
    isAuthenticating,
    authClient,
    login,
    logout,
    principal,
    systemActor,
    getManagmentActor
  } = useAuthClient()

  const [loading, setLoading] = useState(false)

  const [walletCanisterId, setWalletCanisterId] = useState<string>("")
  const [walletActor, setWalletActor] = useState<B3Wallet>()
  const [walletName, setWalletName] = useState<string>("")

  const toast = useToast()

  const fetchUserActor = useCallback(
    async (canisterId: string) => {
      console.log(typeof canisterId)
      if (!authClient || !canisterId || canisterId === "undefined") return
      setWalletCanisterId(canisterId)

      const userActor = createB3WalletActor(
        canisterId,
        authClient.getIdentity()
      )

      console.log("fetching user actor")
      setLoading(true)

      userActor
        .name()
        .then(async name => {
          if (!name) {
            throw new Error("No name found")
          }
          console.log("user actor name", name)
          setWalletActor(userActor)
          setWalletName(name)
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
      <Header
        systemActor={systemActor}
        getManagmentActor={getManagmentActor}
        principal={principal}
      />
      <Stack as="main" minH="100px" position="relative" justify="space-between">
        {isAuthenticating && <Loading title="Authenticating" />}
        {loading && <Loading title="Loading Wallet" />}
        {isAuthenticated ? (
          walletActor ? (
            <Wallet
              principal={principal}
              walletName={walletName}
              actor={walletActor}
              systemActor={systemActor}
              walletCanisterId={walletCanisterId}
            />
          ) : systemActor ? (
            <System
              principal={principal}
              systemActor={systemActor}
              fetchUserActor={fetchUserActor}
            />
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
                <Text fontSize="sm">
                  Login with your Internet Identity to continue
                </Text>
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
