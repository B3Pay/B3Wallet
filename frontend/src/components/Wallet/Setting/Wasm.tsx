import { RepeatIcon } from "@chakra-ui/icons"
import {
  Button,
  CardBody,
  CardHeader,
  IconButton,
  Progress,
  Select,
  Stack,
  Text
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { Release, ReleaseName } from "declarations/b3_system/b3_system.did"
import useLoadRelease from "hooks/useLoadRelease"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3System, B3Wallet } from "service/actor"
import Error from "../../Error"
import Address from "../Address"

interface JsonFile {
  name: string
  size: number
  url: string
  wasmFile: string
  version: string
}

interface OnlineRelease {
  name: string
  file: string
  url: string
  version: string | undefined
  withCandid: boolean
}

interface WasmProps {
  actor: B3Wallet
  systemActor: B3System
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
}

interface UnknownRelease {
  name: string
  hash: Uint8Array | number[]
  version: string
  size: bigint
}

const unknownRelease = (
  hash: Uint8Array | number[],
  size: bigint
): UnknownRelease => ({
  hash,
  size,
  name: "Custom",
  version: "Custom"
})

const Wasm: React.FC<WasmProps> = ({
  actor,
  systemActor,
  setLoading,
  fetchAccounts
}) => {
  const [error, setError] = useState<string>()
  const [releases, setReleases] = useState<OnlineRelease[]>()
  const [selectedRelease, setSelectedRelease] = useState("")
  const [version, setVersion] = useState<string>()

  const [loadedRelease, setLoadedRelease] = useState<Release | UnknownRelease>()

  const errorToast = useToastMessage()
  const { uploader, progress, wasmLoading } = useLoadRelease(actor)

  const updateVersion = async () => actor.version().then(setVersion)

  const updateWasmVersion = async () => {
    await actor
      .wasm_details()
      .then(async ({ hash, size }) => {
        console.log("Wasm details", hash, size)
        try {
          let walletName = await actor.name()
          const releaseName = { [walletName]: null } as ReleaseName

          const release = await systemActor.get_release_by_hash_string(
            releaseName,
            hash
          )
          console.log("Release", release)
          if (release) {
            setLoadedRelease(release)
          }
        } catch (e: any) {
          console.log("Release not found", e)
          if (size === 0n) {
            setLoadedRelease(undefined)
          } else {
            setLoadedRelease(unknownRelease(hash, size))
          }
        }
      })
      .catch(() => setLoadedRelease(undefined))
  }

  useEffect(() => {
    const fetchReleases = async () => {
      let walletName = await actor.name()
      const response = await fetch("wasm/releases.json")

      const releases = (await response.json()) as JsonFile[]

      const releaseMap = releases.reduce((acc, release) => {
        if (release.name !== walletName) {
          return acc
        }

        const withCandid = release.wasmFile.includes("candid")

        acc.push({
          name: release.name,
          file: release.wasmFile,
          url: release.url,
          version: release.version,
          withCandid
        })

        return acc
      }, [] as OnlineRelease[])

      setReleases(releaseMap)
    }

    console.log("Fetching releases")

    updateVersion()
    updateWasmVersion()
    fetchReleases()
  }, [])

  const loadCanisterWasm = useCallback(async () => {
    setError(undefined)
    setLoading(true)

    await uploader(selectedRelease)

    console.log("Wasm loaded")

    updateWasmVersion()
    setLoading(false)
  }, [actor, selectedRelease, setLoading])

  const upgradeCanister = async () => {
    setError(undefined)
    if (!loadedRelease.version || loadedRelease.version === version) {
      console.log("Canister already At this version")
      return
    }

    setLoading(true)

    try {
      await actor.upgrage_wallet()
    } catch (e: any) {
      console.log(e)
    }

    actor.version().then(version => {
      console.log("Canister upgraded")
      if (loadedRelease.version === version) {
        errorToast({
          title: "Success",
          description: `Canister upgraded to version ${version}`,
          status: "success",
          duration: 5000,
          isClosable: true
        })
      } else {
        errorToast({
          title: "Error",
          description: "Canister upgrade failed",
          status: "error",
          duration: 5000,
          isClosable: true
        })
      }
      updateWasmVersion()
      fetchAccounts()
      setLoading(false)
    })
  }

  const resetWasm = async () => {
    setError(undefined)

    setLoading(true)

    try {
      await actor.unload_wasm()
    } catch (e: any) {
      errorToast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    }

    console.log("Canister reset")

    updateWasmVersion()
    setLoading(false)
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
    >
      <CardHeader pb={2}>
        <Stack direction="row" justify="space-between" align="center">
          <Text fontSize="md" fontWeight="bold">
            Wallet Wasm
          </Text>
          <Text fontSize="sm" fontWeight="bold" color="gray.600">
            {version}
          </Text>
        </Stack>
      </CardHeader>
      <CardBody borderTop="1px" borderColor="gray.200" position="relative">
        {wasmLoading && (
          <Loading title="Wasm loading">
            <Progress hasStripe value={progress} height={2} />
          </Loading>
        )}
        <Stack fontSize="sm" fontWeight="semibold">
          {releases && (
            <Stack direction="row" spacing={2}>
              <Select
                flex={8}
                value={selectedRelease}
                onChange={e => setSelectedRelease(e.target.value)}
              >
                <option value="">Select version</option>
                {releases.map(({ version, url, withCandid }) => (
                  <option key={url} value={url}>
                    {version} {withCandid ? "(candid)" : ""}
                  </option>
                ))}
              </Select>
              <Button onClick={loadCanisterWasm} flex={4} colorScheme="blue">
                Load Wasm
              </Button>
            </Stack>
          )}
          {error && <Error error={error} />}
          {loadedRelease ? (
            <Stack
              direction="column"
              borderWidth="1px"
              borderRadius="lg"
              overflow="hidden"
            >
              <CardHeader pb={2}>
                <Stack direction="row" justify="space-between" align="center">
                  <Text fontSize="md" fontWeight="bold">
                    Loaded Wasm
                  </Text>
                  <Stack direction="row" align="center">
                    <Text>{loadedRelease.version}</Text>
                    <IconButton
                      aria-label="Refresh"
                      icon={<RepeatIcon />}
                      onClick={updateWasmVersion}
                      size="xs"
                    />
                  </Stack>
                </Stack>
              </CardHeader>
              <CardBody borderTop="1px" borderColor="gray.200">
                <Stack spacing={2}>
                  <Stack direction="row" justify="space-between" align="center">
                    <Text fontSize="sm" fontWeight="semibold">
                      Release Name:
                    </Text>
                    <Text fontSize="sm" fontWeight="semibold">
                      {loadedRelease.name}
                    </Text>
                  </Stack>
                  <Stack direction="row" justify="space-between" align="center">
                    <Text fontSize="sm" fontWeight="semibold">
                      Release Size:
                    </Text>
                    <Text fontSize="sm" fontWeight="semibold">
                      {(loadedRelease.size / 1000n).toLocaleString()} kb
                    </Text>
                  </Stack>
                  {loadedRelease.date && (
                    <Stack
                      direction="row"
                      justify="space-between"
                      align="center"
                    >
                      <Text fontSize="sm" fontWeight="semibold">
                        Release Date:
                      </Text>
                      <Text fontSize="sm" fontWeight="semibold">
                        {nanoTimeStampToDate(loadedRelease.date)}
                      </Text>
                    </Stack>
                  )}
                  <Stack direction="row" justify="space-between" align="center">
                    <Text fontSize="sm" fontWeight="semibold">
                      Release Hash:
                    </Text>
                    <Address address={arrayToHex(loadedRelease.hash)} />
                  </Stack>
                  <Stack direction="row" spacing={2}>
                    <Button onClick={resetWasm} flex={2} colorScheme="red">
                      Reset
                    </Button>
                    <Button
                      onClick={upgradeCanister}
                      flex={10}
                      colorScheme="orange"
                    >
                      Upgrade
                    </Button>
                  </Stack>
                </Stack>
              </CardBody>
            </Stack>
          ) : (
            <Text fontSize="sm" fontWeight="semibold">
              No wasm loaded
            </Text>
          )}
        </Stack>
      </CardBody>
    </Stack>
  )
}

export default Wasm

const nanoTimeStampToDate = (timestampInNano: bigint) => {
  const timestampInMilli = Number(timestampInNano) / 1000000

  const date = new Date(timestampInMilli)

  return date.toLocaleString()
}

const arrayToHex = (array: Uint8Array | number[]) => {
  return Array.from(array, byte => {
    return ("0" + (byte & 0xff).toString(16)).slice(-2)
  }).join("")
}
