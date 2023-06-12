import {
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
  Text
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { Release } from "declarations/b3_system/b3_system.did"
import { useCallback, useEffect, useState } from "react"
import { B3System } from "../../service/actor"
import Disclaimer from "../Disclaimer"
import Error from "../Error"
import Loading from "../Loading"

interface SystemProps {
  systemActor: B3System
  fetchUserActor: (walletCanisterId: string) => void
}

const System: React.FC<SystemProps> = ({ systemActor, fetchUserActor }) => {
  const [input, setInput] = useState<string>("")
  const [releases, setReleases] = useState<Release[]>([])
  const [selectedVersion, setSelectedVersion] = useState<string>("")

  const [error, setError] = useState<string>()
  const [loading, setLoading] = useState<boolean>()

  const [canisterId, setCanisterId] = useState<string>("")

  const fetchCanisterId = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    systemActor
      .releases()
      .then(releases => {
        setReleases(releases)

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
        .install_wallet_canister([canisterPrincipal])
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
    [systemActor, fetchCanisterId, fetchUserActor]
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
      .create_wallet_canister()
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
  }, [systemActor, fetchUserActor])

  return (
    <Card>
      <Stack borderBottom="1px solid #e2e8f0" spacing="8" p={3}>
        {error && <Error error={error} />}
        {loading && <Loading />}
        {releases.length > 0 && (
          <FormControl as="fieldset">
            <FormLabel as="label">Select a version:</FormLabel>
            <RadioGroup
              display="grid"
              gridTemplateColumns="repeat(2, 1fr)"
              gap={2}
              value={selectedVersion}
              onChange={e => setSelectedVersion(e)}
            >
              {releases.map(release => (
                <Radio
                  flex={6}
                  key={release.version}
                  value={release.version}
                  isDisabled={release.deprecated}
                >
                  <Text as="span" fontWeight="bold" mr={2}>
                    {release.name}
                  </Text>
                  {release.version}
                </Radio>
              ))}
            </RadioGroup>
            <FormHelperText>
              &nbsp;For candid interface select &quot;-candid&quot; version
            </FormHelperText>
          </FormControl>
        )}
        <FormLabel as="label">
          Install a canister by entering its id or create a new one:
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
