import {
  Badge,
  Button,
  Card,
  FormControl,
  FormHelperText,
  FormLabel,
  Input,
  InputGroup,
  Link,
  Radio,
  RadioGroup,
  Stack,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs,
  Text
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { Release, ReleaseName } from "declarations/b3_system/b3_system.did"
import { useCallback, useEffect, useState } from "react"
import { B3System } from "../../service/actor"
import Disclaimer from "../Disclaimer"
import Error from "../Error"
import Loading from "../Loading"

type ReleaseMap = [ReleaseName, Array<Release>][]

interface SystemProps {
  systemActor: B3System
  fetchUserActor: (walletCanisterId: string) => void
}

const System: React.FC<SystemProps> = ({ systemActor, fetchUserActor }) => {
  const [input, setInput] = useState<string>("")
  const [releaseMap, setReleaseMap] = useState<ReleaseMap>([])

  const [selectedWallet, setSelectedWallet] = useState<string>("")
  const [selectedVersion, setSelectedVersion] = useState<string>("")

  const [error, setError] = useState<string>()
  const [loading, setLoading] = useState<boolean>()

  const [canisterId, setCanisterId] = useState<string>("")

  const fetchCanisterId = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    systemActor
      .release_map()
      .then(releases => {
        setReleaseMap(releases)

        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })

    systemActor
      .get_canister()
      .then(({ canisters }) => {
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

  useEffect(() => {
    setLoading(true)

    fetchCanisterId()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const installCanister = useCallback(
    async (canisterId: string) => {
      setError(undefined)
      setLoading(true)

      const canisterPrincipal = Principal.fromText(canisterId)

      systemActor
        .install_wallet_canister(selectedWallet, [canisterPrincipal])
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

      const canisterPrincipal = Principal.fromText(canisterId)

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
    setLoading(true)

    systemActor
      .create_wallet_canister(selectedWallet)
      .then(async userControl => {
        if ("Err" in userControl) {
          setLoading(false)
          setError(userControl.Err)
          return console.log(userControl.Err)
        }

        setCanisterId(userControl.Ok.canisters[0].toString())
        fetchUserActor(userControl.Ok.canisters[0].toString())
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setError(e)
        setLoading(false)
      })
  }, [systemActor, selectedWallet, fetchUserActor])

  return (
    <Card>
      <Stack borderBottom="1px solid #e2e8f0" spacing="8" p={3}>
        {error && <Error error={error} />}
        {loading && <Loading />}
        <FormControl as="fieldset">
          <FormLabel as="label">Select a version:</FormLabel>
          <Tabs>
            <TabList>
              {releaseMap.map(([releaseName, releases]) => {
                const walletName = Object.keys(releaseName)[0]

                return (
                  <Tab
                    key={walletName}
                    value={walletName}
                    onClick={() => {
                      setSelectedWallet(walletName)

                      if (releases.length > 0) {
                        releases.forEach(release => {
                          if (!release.deprecated) {
                            setSelectedVersion(release.version)
                          }
                        })
                      }
                    }}
                  >
                    {walletName}
                  </Tab>
                )
              })}
            </TabList>
            <TabPanels>
              {releaseMap.length > 0 &&
                releaseMap.map(([, releases]) => (
                  <TabPanel key={releases[0].version}>
                    <RadioGroup
                      onChange={value => setSelectedVersion(value)}
                      display="flex"
                      flexDir={{ base: "column", md: "row" }}
                      flexWrap="wrap"
                      justifyContent="space-between"
                      alignItems="flex-start"
                      value={selectedVersion}
                    >
                      {releases.map(release => (
                        <Radio
                          flex={6}
                          key={release.version}
                          value={release.version}
                          isDisabled={release.deprecated}
                        >
                          <Text as="span" fontWeight="semibold" mr={2}>
                            {release.version}
                          </Text>
                          {release.deprecated && (
                            <Badge colorScheme="red">Deprecated</Badge>
                          )}
                        </Radio>
                      ))}
                    </RadioGroup>
                  </TabPanel>
                ))}
            </TabPanels>
          </Tabs>
          <FormHelperText>
            &nbsp;For candid interface select &quot;-candid&quot; version
          </FormHelperText>
        </FormControl>
        <FormLabel as="label">
          Install {selectedWallet}({selectedVersion}) on a canister by entering
          its id or create a new one:
        </FormLabel>
        <Text fontSize="sm">
          You can create a canister id on the&nbsp;
          <Link
            color="blue.500"
            isExternal
            target="_blank"
            rel="noopener noreferrer"
            href="https://nns.ic0.app/"
          >
            NNS Dapp
          </Link>
          . and add this canister as controller to your canister then click on
          the button below.
        </Text>
        <FormControl as="fieldset">
          <InputGroup>
            <Input
              flex={8}
              type="text"
              placeholder="Enter Canister id"
              value={canisterId}
              onChange={e => setCanisterId(e.target.value)}
            />
            <Button flex={4} onClick={() => installCanister(canisterId)}>
              Install Wallet
            </Button>
          </InputGroup>
          <FormHelperText fontSize="xs">
            Note: This will install the wallet canister on your canister then
            remove the controller, so you have full control over your wallet.
          </FormHelperText>
        </FormControl>
        <Stack>
          <FormLabel as="label">
            Create a canister and install the wallet canister on it.
          </FormLabel>
          <Button onClick={createCanister}>Create Canister & Install</Button>
          <FormControl as="fieldset">
            <FormHelperText fontSize="xs">
              Note: This will create a canister and install the wallet canister
              on it, then remove the controller, so you have full control over
              your wallet.
            </FormHelperText>
          </FormControl>
        </Stack>
        <Stack>
          <FormLabel as="label">Add a canister to your wallet.</FormLabel>
          <FormControl>
            <InputGroup>
              <Input
                flex={8}
                type="text"
                placeholder="Enter Canister id"
                value={input}
                onChange={e => setInput(e.target.value)}
              />
              <Button flex={4} onClick={() => addCanister(canisterId)}>
                Add Canister
              </Button>
            </InputGroup>
            <FormHelperText fontSize="xs">
              Note: This will add the canister to your own if you are one of the
              signer.
            </FormHelperText>
          </FormControl>
        </Stack>
        <Disclaimer />
      </Stack>
    </Card>
  )
}

export default System
