import { RepeatIcon } from "@chakra-ui/icons"
import {
  Button,
  Card,
  Container,
  IconButton,
  Input,
  Stack,
  Text
} from "@chakra-ui/react"
import { AccountIdentifier } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import Address from "components/Address"
import PendingModal from "components/ConfirmModal"
import ProcessedModal from "components/ProcessedModal"
import Store from "components/store"
import {
  ChainEnum,
  PendingRequest,
  ProcessedRequest,
  Request
} from "declarations/b3_payment/b3_payment.did"
import { B3_PAYMENT_CANISTER_ID } from "helpers/config"
import useToastMessage from "hooks/useToastMessage"
import Head from "next/head"
import { useEffect, useState } from "react"
import { B3Payment, makeB3PaymentActor } from "service/actor"
import { Footer } from "../components/Footer"
import Header from "../components/Header"
import Loading from "../components/Loading"

function HomePage() {
  const [loading, setLoading] = useState(false)
  const [walletCanisterId, setWalletCanisterId] = useState<Principal>()
  const [isConnected, setIsConnected] = useState<boolean>(false)
  const [paymentActor, setPaymentActor] = useState<B3Payment>()

  const [pendingRequests, setPendingRequests] = useState<PendingRequest[]>()
  const [processedRequests, setProcessedRequests] =
    useState<ProcessedRequest[]>()

  const toast = useToastMessage()

  useEffect(() => {
    const localWalletCanisterId = localStorage.getItem("walletCanisterId")

    const getPaymentActor = async () => {
      try {
        setLoading(true)
        const actor = await makeB3PaymentActor()

        console.log(actor)
        if (localWalletCanisterId) {
          const canisterId = Principal.fromText(localWalletCanisterId)
          setWalletCanisterId(canisterId)
          actor.is_connected(canisterId).then(setIsConnected)
        }

        setPaymentActor(actor)
      } catch (error) {
        toast({
          title: "Error",
          description: error.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      } finally {
        setLoading(false)
      }
    }

    getPaymentActor()
  }, [toast])

  const reuqest_connect = async (canisterId: Principal) => {
    if (!canisterId) {
      return
    }
    const connectStatus = await paymentActor?.is_connected(canisterId)

    if (connectStatus) {
      setIsConnected(true)
      localStorage.setItem("walletCanisterId", canisterId.toString())
      return
    }

    console.log("request connect", canisterId.toString())

    try {
      const requestId = await paymentActor?.request_connect(canisterId)

      console.log(requestId)
    } catch (error) {
      toast({
        title: "Error",
        description: error.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    }
  }

  const buyHandler = async (product: any) => {
    try {
      setLoading(true)
      let chain: ChainEnum
      const principal = Principal.fromText(B3_PAYMENT_CANISTER_ID)

      let to: string = ""

      switch (product.network) {
        case "BTC":
          chain = {
            BTC: {
              Testnet: null
            }
          }
          to = "mftXpN36ENw4sh8J9Bw5AtQQrUWQmUBpxD"
          break
        case "CKBTC":
          chain = {
            CKBTC: {
              Testnet: null
            }
          }
          to = principal.toText()
          break
        case "ICP":
          chain = {
            ICP: null
          }
          to = AccountIdentifier.fromPrincipal({
            principal
          }).toHex()
          break
        case "ETH":
          chain = {
            EVM: 0n
          }
          to = "0x31ab7b1de658bda5a56a6ef43cf7ce36a15cc1e1"
          break
        default:
          chain = {
            CKBTC: {
              Testnet: null
            }
          }
          break
      }

      const args: Request = {
        SendToken: {
          chain,
          to,
          amount: product.price,
          account_id: "-default"
        }
      }

      console.log(args)

      const requestId = await paymentActor?.request_maker(
        walletCanisterId,
        args,
        `Buy ${product.name} for ${(product.price / 10 ** 8).toString()} ${
          product.network
        }, https://b3payment.ic0.app/${product.id}`,
        []
      )

      let requestIds: any = localStorage.getItem("requestIds")

      if (requestIds) {
        requestIds = JSON.parse(requestIds)
      } else {
        requestIds = []
      }

      requestIds.push(requestId.toString())

      localStorage.setItem("requestIds", JSON.stringify(requestIds))

      console.log(requestId)
    } catch (error) {
      toast({
        title: "Error",
        description: error.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoading(false)
    }
  }

  const getRequests = async (requestId: bigint) => {
    if (!walletCanisterId) {
      return
    }
    try {
      setLoading(true)
      await paymentActor.check_processed_request(walletCanisterId, requestId)

      console.log(pendingRequests)
    } catch (error) {
      toast({
        title: "Error",
        description: "Not Processed Yet",
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoading(false)
    }
  }
  const checkProcessedRequests = async () => {
    if (!walletCanisterId) {
      return
    }
    try {
      let processsed = await paymentActor.check_processed_requests(
        walletCanisterId
      )

      console.log(processsed)

      const localRequestIds = localStorage.getItem("requestIds")
      let requestIds: string[] = []

      if (localRequestIds) {
        requestIds = JSON.parse(localRequestIds)
      }

      console.log(requestIds)

      processsed = processsed.filter(request =>
        requestIds.some(id => request.request.id.toString() === id)
      )

      console.log(processsed)

      setProcessedRequests(processsed)
    } catch (error) {
      toast({
        title: "Error",
        description: error.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoading(false)
    }
  }

  const checkPendingRequests = async () => {
    if (!walletCanisterId) {
      return
    }
    try {
      let requestIds: any = localStorage.getItem("requestIds")

      if (requestIds) {
        requestIds = JSON.parse(requestIds)
      } else {
        requestIds = []
      }

      const requests = await paymentActor.check_pending_requests(
        walletCanisterId
      )

      setPendingRequests(requests)

      console.log(requests)
    } catch (error) {
      toast({
        title: "Error",
        description: error.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    if (!paymentActor) return

    checkPendingRequests()
    checkProcessedRequests()
  }, [paymentActor, toast])

  const canisterInputHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    let canisterPrincipal: Principal

    try {
      canisterPrincipal = Principal.fromText(e.target.value)
    } catch (e) {
      console.log(e)

      setLoading(false)
      return
    }

    setWalletCanisterId(canisterPrincipal)
  }

  return (
    <Container maxW="2xl" p={1}>
      <Head>
        <title>B3Payment</title>
      </Head>
      <Header />
      <Card p={2} mb={2}>
        <Stack direction="row" spacing="2" justify="space-between">
          <Text size="lg" textAlign="center" my={2}>
            Payment Canister
          </Text>
          <Address address={B3_PAYMENT_CANISTER_ID} />
        </Stack>
      </Card>
      <Stack as="main" minH="100px" position="relative" justify="space-between">
        {loading && <Loading title="Loading Wallet" />}
        {paymentActor ? (
          <Stack spacing="2">
            {isConnected ? (
              <Card p={2}>
                <Stack
                  direction="row"
                  spacing="2"
                  justify="space-between"
                  wrap="wrap"
                >
                  <Address address={walletCanisterId.toString()} />
                  <Stack direction="row" spacing="2" justify="end">
                    <IconButton
                      aria-label="Refresh"
                      colorScheme="green"
                      variant="ghost"
                      icon={<RepeatIcon />}
                      onClick={() => {
                        checkPendingRequests()
                        checkProcessedRequests()
                      }}
                    />
                    <Button
                      colorScheme="red"
                      onClick={() => {
                        localStorage.removeItem("walletCanisterId")
                        setIsConnected(false)
                      }}
                      isLoading={loading}
                    >
                      Disconnect
                    </Button>
                    {pendingRequests?.length > 0 && (
                      <PendingModal
                        requests={pendingRequests}
                        checkRequest={getRequests}
                        fetchRequests={checkPendingRequests}
                      />
                    )}
                    {processedRequests?.length > 0 && (
                      <ProcessedModal
                        requests={processedRequests}
                        checkRequest={getRequests}
                        fetchRequests={checkProcessedRequests}
                      />
                    )}
                  </Stack>
                </Stack>
              </Card>
            ) : (
              <Card p={2}>
                <Stack>
                  <Text fontSize="sm">Status</Text>
                  <Text fontSize="md">Not Connected</Text>
                  <Stack direction="row" spacing="2">
                    <Input
                      flex={6}
                      placeholder="canister id"
                      onChange={canisterInputHandler}
                      value={walletCanisterId?.toString()}
                    />
                    <Button
                      flex={6}
                      onClick={() => reuqest_connect(walletCanisterId)}
                      isLoading={loading}
                    >
                      Connect
                    </Button>
                  </Stack>
                </Stack>
              </Card>
            )}
            <Card p={2}>
              <Store buyHandler={buyHandler} />
            </Card>
          </Stack>
        ) : (
          <Loading dark title="Fetching" />
        )}
      </Stack>
      <Footer />
    </Container>
  )
}

export default HomePage
