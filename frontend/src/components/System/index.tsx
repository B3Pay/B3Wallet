import {
  Box,
  Button,
  Card,
  FormControl,
  FormHelperText,
  FormLabel,
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
import { Release, ReleaseName } from "declarations/b3_system/b3_system.did"
import { B3_SYSTEM_CANISTER_ID, IS_LOCAL } from "helpers/config"
import { useCallback, useEffect, useState } from "react"
import { B3System } from "../../service"
import Disclaimer from "../Disclaimer"
import Loading from "../Loading"
import WalletError from "../WalletError"

type ReleaseMap = [ReleaseName, Array<Release>][]

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

          fetchUserActor(userControl.Ok.canisters[0].toString())
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

      let canisterPrincipal: Principal
      console.log(canisterId)
      try {
        canisterPrincipal = Principal.fromText(canisterId)
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
        <WalletError error={error} mb={1} borderRadius="base" shadow="base" />
      )}
      {loading && <Loading />}
      <Card>
        <Tabs isFitted variant="enclosed">
          <TabList mb="1em">
            <Tab>Install</Tab>
            <Tab>Anonymously</Tab>
          </TabList>
          <Stack spacing={2} px={4}>
            <FormLabel as="label">Select a Wallet:</FormLabel>
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
            <PrincipalCard address={principal} />
          </Stack>
          <TabPanels>
            <TabPanel>
              <FormControl as="fieldset">
                <Stack spacing={2} mt={4}>
                  <FormLabel as="label">
                    Install {selectedWallet} on a canister by entering its id
                    below:
                  </FormLabel>
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
                            ? "http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/"
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
                      Click on the button below to install the wallet canister
                      on your canister.
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
                    remove the controller, so you have full control over your
                    wallet.
                  </FormHelperText>
                </Stack>
                <Stack spacing={2} mt={4}>
                  <FormLabel as="label" mt={4}>
                    Or Create a canister and install the wallet canister on it.
                  </FormLabel>
                  <Button onClick={createCanister}>
                    Create Canister & Install
                  </Button>
                  <FormHelperText fontSize="xs">
                    Note: This will create a canister and install the wallet
                    canister on it, then remove the controller, so you have full
                    control over your wallet.
                  </FormHelperText>
                </Stack>
                <Stack spacing={2} my={4}>
                  <FormLabel as="label" mt={4}>
                    Or Add a canister to your wallet.
                  </FormLabel>
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
                    Note: This will add the canister to your own if you are one
                    of the signer.
                  </FormHelperText>
                </Stack>
              </FormControl>
              <Disclaimer />
            </TabPanel>
            <TabPanel>
              <FormControl as="fieldset">
                <FormLabel as="label">
                  Anonymously use the user interface
                </FormLabel>
                <Text fontSize="sm" mb={4}>
                  no canister will be installed or connected to your wallet.
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
                    Go
                  </Button>
                </InputGroup>
                <FormHelperText fontSize="xs">
                  Note: This will save on the local storage the canister id, so
                  you can use it later.
                </FormHelperText>
              </FormControl>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Card>
    </Box>
  )
}

export default System
