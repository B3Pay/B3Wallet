import {
  Button,
  ButtonGroup,
  FormControl,
  FormHelperText,
  FormLabel,
  Input,
  Link,
  Radio,
  RadioGroup,
  Stack,
  Text
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { Release } from "declarations/b3_system/b3_system.did"
import { useCallback, useEffect, useState } from "react"
import { B3System } from "service/actor"
import Disclaimer from "./Disclaimer"
import Error from "./Error"
import Loading from "./Loading"

interface InstallCanisterProps {
  systemActor: B3System
  fetchUserActor: (walletCanisterId: string) => void
}

const InstallCanister: React.FC<InstallCanisterProps> = ({
  systemActor,
  fetchUserActor
}) => {
  const [error, setError] = useState<string>()
  const [loading, setLoading] = useState<boolean>()

  const [releases, setReleases] = useState<Release[]>([])
  const [canisterId, setCanisterId] = useState<string>("")
  const [selectedVersion, setSelectedVersion] = useState<string>("")

  useEffect(() => {
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
      .then(({ canister_id }) => {
        const walletCanisterId = canister_id.toString()

        setCanisterId(walletCanisterId)
        fetchUserActor(walletCanisterId)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const createCanister = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    systemActor
      .create_wallet_canister([selectedVersion])
      .then(async userControl => {
        if ("Err" in userControl) {
          setLoading(false)
          setError(userControl.Err)
          return console.log(userControl.Err)
        }

        setCanisterId(userControl.Ok.canister_id.toString())
        fetchUserActor(userControl.Ok.canister_id.toString())
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setError(e)
        setLoading(false)
      })
  }, [systemActor, fetchUserActor, selectedVersion])

  const installCanister = useCallback(
    async (canisterId: string) => {
      setError(undefined)
      setLoading(true)

      const canisterPrincipal = Principal.fromText(canisterId)

      systemActor
        .install_wallet_canister([canisterPrincipal], [selectedVersion])
        .then(async userControl => {
          if ("Err" in userControl) {
            setLoading(false)

            return setError(userControl.Err)
          }

          fetchUserActor(userControl.Ok.canister_id.toString())
          setLoading(false)
        })
        .catch(e => {
          console.log(e)
          setError(e)
          setLoading(false)
        })
    },
    [systemActor, fetchUserActor, selectedVersion]
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
          setLoading(false)
        })
    },
    [systemActor, fetchUserActor]
  )

  return (
    <Stack borderBottom="1px solid #e2e8f0" spacing="8">
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
                {release.version}
              </Radio>
            ))}
          </RadioGroup>
          <FormHelperText>
            &nbsp;For candid interface select &quot;-candid&quot; version
          </FormHelperText>
        </FormControl>
      )}
      <Stack>
        <Text>
          Install a canister by entering its id and clicking the button below.
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
          .
        </Text>
        <ButtonGroup>
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
        </ButtonGroup>
      </Stack>
      <Stack>
        <Text>Or create a new canister and install it.</Text>
        <Button onClick={createCanister}>Create Canister & Install</Button>
      </Stack>
      <Stack>
        <Text>Or using shared wallet canister.</Text>
        <ButtonGroup>
          <Input
            flex={8}
            type="text"
            placeholder="Enter Canister id"
            value={canisterId}
            onChange={e => setCanisterId(e.target.value)}
          />
          <Button flex={4} onClick={() => addCanister(canisterId)}>
            Add Wallet
          </Button>
        </ButtonGroup>
      </Stack>
      <Disclaimer />
    </Stack>
  )
}

export default InstallCanister
