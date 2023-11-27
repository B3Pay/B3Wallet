import {
  Alert,
  Box,
  Button,
  Card,
  Circle,
  FormControl,
  FormHelperText,
  Input,
  InputGroup,
  Link,
  ListItem,
  Select,
  Stack,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs,
  Text,
  UnorderedList
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import Address from "components/Wallet/Address"
import PrincipalCard from "components/Wallet/PrincipalCard"
import { Release, ReleaseNames } from "declarations/b3_system/b3_system.did"
import { B3_SYSTEM_CANISTER_ID, IS_LOCAL } from "helpers/config"
import { useCallback, useEffect, useState } from "react"
import { B3System } from "../../service"
import Disclaimer from "../Disclaimer"
import Loading from "../Loading"
import WalletError from "../WalletError"

type ReleaseMap = [ReleaseNames, Array<Release>][]

interface SystemProps {
  principal: string
  systemActor: B3System
  fetchUserActor: (walletCanisterId: string) => Promise<void>
}

const System: React.FC<SystemProps> = ({
  principal,
  systemActor,
  fetchUserActor
}) => {
  const [releaseMap, setReleaseMap] = useState<ReleaseMap>([])

  const [selectedWallet, setSelectedWallet] = useState<string>("b3_wallet")

  const [error, setError] = useState<string>()
  const [loading, setLoading] = useState<boolean>()

  const [canisterId, setCanisterId] = useState<string>("")
  const [anonymousCanisterId, setAnonymousCanisterId] = useState<string>("")

  const fetchCanisterId = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    systemActor
      .get_canisters()
      .then(canisters => {
        console.log(canisters[0])
        const walletCanisterId = canisters[0].toString()

        setCanisterId(walletCanisterId)
        fetchUserActor(walletCanisterId)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
  }, [systemActor, fetchUserActor])

  const fetchReleases = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    systemActor
      .release_map()
      .then(releases => {
        console.log(releases)
        setReleaseMap(releases)

        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
  }, [systemActor, fetchUserActor])

  useEffect(() => {
    const localWalletCanisterId = localStorage.getItem("walletCanisterId")

    fetchReleases()

    if (localWalletCanisterId) {
      fetchUserActor(localWalletCanisterId)
    } else {
      fetchCanisterId()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const installCanister = useCallback(
    async (canisterId: string) => {
      setError(undefined)
      if (!selectedWallet) {
        return setError("Please select a wallet")
      }

      setLoading(true)

      let canisterPrincipal: Principal

      try {
        canisterPrincipal = Principal.fromText(canisterId)
      } catch (e) {
        console.log(e)

        setLoading(false)
        return setError("Invalid canister id!")
      }

      systemActor
        .install_wallet_canister(selectedWallet, canisterPrincipal)
        .then(async userControl => {
          if ("Err" in userControl) {
            setLoading(false)

            return setError(userControl.Err)
          }

          const canisterId = userControl.Ok.canisters[0].toString()

          localStorage.setItem("walletCanisterId", canisterId)
          fetchUserActor(canisterId)
          setLoading(false)
        })
        .catch(e => {
          console.log(e)
          setError(e)
          fetchCanisterId()
          setLoading(false)
        })
    },
    [systemActor, selectedWallet, fetchCanisterId, fetchUserActor]
  )

  const addCanister = useCallback(
    async (canisterId: string) => {
      setError(undefined)
      setLoading(true)

      let canisterPrincipal: Principal

      try {
        canisterPrincipal = Principal.fromText(canisterId)
      } catch (e) {
        console.log(e)

        setLoading(false)
        return setError("Invalid canister id!")
      }

      systemActor
        .add_wallet_canister(canisterPrincipal)
        .then(() => {
          fetchUserActor(canisterId)
          setLoading(false)
        })
        .catch(e => {
          console.log(e)
          setError(e)
          fetchCanisterId()
          setLoading(false)
        })
    },
    [systemActor, fetchCanisterId, fetchUserActor]
  )

  const createCanister = useCallback(async () => {
    setError(undefined)
    if (!selectedWallet) {
      return setError("Please select a wallet")
    }

    setLoading(true)

    systemActor
      .create_wallet_canister(selectedWallet)
      .then(async userControl => {
        if ("Err" in userControl) {
          setLoading(false)
          setError(userControl.Err)
          return console.log(userControl.Err)
        }

        const canisterId = userControl.Ok.canisters[0].toString()

        setCanisterId(canisterId)
        fetchUserActor(canisterId)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setError(e)
        setLoading(false)
      })
  }, [systemActor, selectedWallet, fetchUserActor])

  const anonymouslyRun = useCallback(
    async (canisterId: string) => {
      setError(undefined)
      setLoading(true)

      try {
        Principal.fromText(canisterId)
      } catch (e) {
        console.log(e)

        setLoading(false)
        return setError("Invalid canister id!")
      }

      localStorage.setItem("canisterId", canisterId)

      await fetchUserActor(canisterId)

      setLoading(false)
    },
    [fetchUserActor]
  )

  return (
    <Box position="relative">
      {error && (
        <WalletError error={error} mb={1} borderRadius="md" shadow="md" />
      )}
      {loading && <Loading />}
      <Card mb={2} borderRadius="md" padding={2}>
        <Alert fontSize="md">
          Welcome to the Wallet Installation page. Here, you can install your
          wallet on a new or existing canister. The process is straightforward
          and involves a few steps. Please follow the instructions carefully to
          ensure a successful installation.
        </Alert>
      </Card>
      <Card>
        <Tabs isFitted variant="enclosed">
          <TabList mb="1em" bg="gray.100" borderTopRadius="md">
            <Tab _selected={{ color: "hotpink", bg: "white" }}>
              Install Wallet
            </Tab>
            <Tab _selected={{ color: "indigo", bg: "white" }}>Add Wallet</Tab>
          </TabList>
          <TabPanels>
            <TabPanel padding={2} pt={0}>
              <Stack mb={2} spacing={2}>
                <Text borderBottom="1px solid" borderColor="gray.100" p={2}>
                  If you're looking to create a new canister, you have two
                  options. You can use the system to create a canister
                  automatically, or you can create one manually if you prefer.
                  The system canister creation process is simpler and
                  recommended for most users. However, if you want more control
                  over the creation process, you might prefer to create the
                  canister manually.
                </Text>
                <Stack spacing={2}>
                  <Text fontSize="md">
                    <b>Step 1:</b> Select a wallet to install
                  </Text>
                  <Select
                    onChange={e => setSelectedWallet(e.target.value)}
                    value={selectedWallet}
                    size="lg"
                  >
                    {releaseMap.map(([releaseName]) => {
                      const walletName = Object.keys(releaseName)[0]
                      return (
                        <option key={walletName} value={walletName}>
                          {walletName}
                        </option>
                      )
                    })}
                  </Select>
                </Stack>
                <Text fontSize="md">
                  <b>Step 2:</b> One of the following options:
                </Text>
                <Stack spacing={2}>
                  <FormControl
                    border="1px solid"
                    borderColor="gray.200"
                    borderRadius="md"
                  >
                    <Stack
                      direction="row"
                      padding={2}
                      mb={2}
                      alignItems="center"
                      alignContent="center"
                      borderBottom="1px solid"
                      borderColor="gray.200"
                    >
                      <Circle
                        size="25px"
                        bg="hotpink"
                        color="white"
                        fontSize={12}
                      >
                        A
                      </Circle>
                      <Text fontSize="md" fontWeight="bold">
                        Install {selectedWallet} on a new canister
                      </Text>
                    </Stack>
                    <Stack spacing={2} padding={2}>
                      <Text>
                        This process will create a new canister and install the
                        wallet canister on it.
                      </Text>
                      <Button onClick={createCanister}>
                        Create Canister & Install
                      </Button>
                      <FormHelperText fontSize="xs">
                        Note: This will create a canister and install the wallet
                        canister on it, then remove the controller, so you have
                        full control over your wallet.
                      </FormHelperText>
                    </Stack>
                  </FormControl>
                  <FormControl
                    border="1px solid"
                    borderColor="gray.200"
                    borderRadius="md"
                  >
                    <Stack
                      direction="row"
                      padding={2}
                      mb={2}
                      alignItems="center"
                      alignContent="center"
                      borderBottom="1px solid"
                      borderColor="gray.200"
                    >
                      <Circle
                        size="25px"
                        bg="hotpink"
                        color="white"
                        fontSize={12}
                      >
                        B
                      </Circle>
                      <Text fontSize="md" fontWeight="bold">
                        Install {selectedWallet} on a canister that you already
                        control
                      </Text>
                    </Stack>
                    <Stack spacing={2} padding={2}>
                      <Text>
                        If you already own a canister, you can directly start
                        from the second step. There's no need to create a new
                        canister unless you want a separate one for this wallet.
                      </Text>
                      <UnorderedList fontSize="sm">
                        <ListItem>
                          Create a canister on the&nbsp;
                          <Link
                            color="blue.500"
                            isExternal
                            target="_blank"
                            rel="noopener noreferrer"
                            href={
                              IS_LOCAL
                                ? "http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:4943/"
                                : "https://nns.ic0.app/"
                            }
                          >
                            NNS Dapp
                          </Link>
                          .
                        </ListItem>
                        <ListItem>
                          Add this system canister as controller
                          <Address
                            address={B3_SYSTEM_CANISTER_ID}
                            overflow="break-word"
                          />
                        </ListItem>
                        <ListItem>
                          Copy the canister id and paste it in the input below.
                        </ListItem>
                        <ListItem>
                          Click on the button below to install the wallet
                          canister on your canister.
                        </ListItem>
                      </UnorderedList>
                      <InputGroup>
                        <Input
                          flex={8}
                          type="text"
                          placeholder="Enter Canister id"
                          value={canisterId}
                          onChange={e => setCanisterId(e.target.value)}
                        />
                        <Button
                          flex={4}
                          onClick={() => installCanister(canisterId)}
                        >
                          Install Wallet
                        </Button>
                      </InputGroup>
                      <FormHelperText fontSize="xs">
                        Note: This will install the wallet on your canister then
                        remove the controller, so you have full control over
                        your wallet.
                      </FormHelperText>
                    </Stack>
                  </FormControl>
                </Stack>
              </Stack>
            </TabPanel>
            <TabPanel padding={2} pt={0}>
              <Stack
                spacing={2}
                borderBottom="1px solid"
                borderColor="gray.100"
              >
                <Text p={2}>
                  If you're looking to become a signer in another user's wallet,
                  please ensure you have the wallet owner's permission. As a
                  signer, you'll be able to authorize transactions from the
                  wallet, so it's important that this is set up correctly.
                  Follow the instructions provided by the wallet owner to add
                  your principal ID as a signer.
                </Text>
                <PrincipalCard address={principal} mb={2} />
              </Stack>
              <Text fontSize="md" py={2}>
                One of the following options:
              </Text>
              <Stack>
                <FormControl
                  border="1px solid"
                  borderColor="gray.200"
                  borderRadius="md"
                >
                  <Stack spacing={2}>
                    <Stack
                      direction="row"
                      padding={2}
                      mb={2}
                      alignItems="center"
                      alignContent="center"
                      borderBottom="1px solid"
                      borderColor="gray.200"
                    >
                      <Circle
                        size="25px"
                        bg="indigo"
                        color="white"
                        fontSize={12}
                      >
                        A
                      </Circle>
                      <Text fontSize="md" fontWeight="bold">
                        Add wallet you are signer of
                      </Text>
                    </Stack>
                    <Stack spacing={2} padding={2}>
                      <Text>
                        If you are one of the signer of a wallet, you can add it
                        to your own wallet.
                      </Text>
                      <InputGroup>
                        <Input
                          flex={8}
                          type="text"
                          placeholder="Enter Canister id"
                          value={anonymousCanisterId}
                          onChange={e => setAnonymousCanisterId(e.target.value)}
                        />
                        <Button
                          flex={4}
                          onClick={() => addCanister(anonymousCanisterId)}
                        >
                          Add Canister
                        </Button>
                      </InputGroup>
                      <FormHelperText fontSize="xs">
                        Note: This will add the canister to your own if you are
                        one of the signer.
                      </FormHelperText>
                    </Stack>
                  </Stack>
                </FormControl>
                <FormControl
                  border="1px solid"
                  borderColor="gray.200"
                  borderRadius="md"
                >
                  <Stack spacing={2}>
                    <Stack
                      direction="row"
                      padding={2}
                      mb={2}
                      alignItems="center"
                      alignContent="center"
                      borderBottom="1px solid"
                      borderColor="gray.200"
                    >
                      <Circle
                        size="25px"
                        bg="indigo"
                        color="white"
                        fontSize={12}
                      >
                        B
                      </Circle>
                      <Text fontSize="md" fontWeight="bold">
                        Use a wallet you are signer of
                      </Text>
                    </Stack>
                    <Stack spacing={2} padding={2}>
                      <Text>
                        No canister will be installed or connected to your
                        wallet.
                      </Text>
                      <InputGroup>
                        <Input
                          flex={8}
                          type="text"
                          placeholder="Enter Canister id"
                          value={anonymousCanisterId}
                          onChange={e => setAnonymousCanisterId(e.target.value)}
                        />
                        <Button
                          flex={4}
                          onClick={() => anonymouslyRun(anonymousCanisterId)}
                        >
                          Use Canister
                        </Button>
                      </InputGroup>
                      <FormHelperText fontSize="xs">
                        Note: This will save on the local storage the canister
                        id, so you can use it later.
                      </FormHelperText>
                    </Stack>
                  </Stack>
                </FormControl>
              </Stack>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Card>
      <Disclaimer asCard />
    </Box>
  )
}

export default System
