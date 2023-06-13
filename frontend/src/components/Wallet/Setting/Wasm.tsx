import {
  Button,
  CardBody,
  CardHeader,
  Progress,
  Select,
  Stack,
  Text
} from "@chakra-ui/react"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import Error from "../../Error"
import Address from "../Address"

interface JsonFile {
  name: string
  size: number
  url: string
  wasmFile: string
  version: string
}

interface Release {
  name: string
  file: string
  url: string
  version: string | undefined
  withCandid: boolean
}

const chunkGenerator = async function* (
  wasmModule: number[],
  chunkSize = 700000
) {
  for (let start = 0; start < wasmModule.length; start += chunkSize) {
    yield wasmModule.slice(start, start + chunkSize)
  }
}

export const useLoadRelease = (actor: B3Wallet) => {
  const [progress, setProgress] = useState<number>(0)

  const uploader = useCallback(async (releaseUrl: string) => {
    const wasm = await fetch(releaseUrl)

    const wasmBuffer = await wasm.arrayBuffer()
    const wasmModule = Array.from(new Uint8Array(wasmBuffer))

    console.log(`Wasm size:`, wasmModule.length)
    let uploadedSize = 0

    for await (const chunks of chunkGenerator(wasmModule)) {
      const result = await actor.load_wasm(chunks)
      console.log(`Chunks :`, result)

      uploadedSize += chunks.length

      setProgress((uploadedSize / wasmModule.length) * 100)
    }

    console.log(`loading done.`)
  }, [])

  return {
    progress,
    uploader
  }
}

interface WasmProps {
  actor: B3Wallet
  fetchAccounts: () => void
  setLoading: (loading: boolean) => void
}

const Wasm: React.FC<WasmProps> = ({ actor, setLoading, fetchAccounts }) => {
  const [error, setError] = useState<string>()
  const [releases, setReleases] = useState<Release[]>()
  const [selectedRelease, setSelectedRelease] = useState("")
  const [version, setVersion] = useState<string>()
  const [wasmHash, setWasmHash] = useState<string>()

  const errorToast = useToastMessage()
  const { uploader, progress } = useLoadRelease(actor)

  const updateVersion = async () => actor.version().then(setVersion)
  const updateWasmVersion = async () =>
    actor.wasm_hash_string().then(setWasmHash)

  useEffect(() => {
    const fetchReleases = async () => {
      let walletName = await actor.name()
      console.log("walletName", walletName)
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
      }, [] as Release[])

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

    const wasm_version = await actor.wasm_hash_string()

    console.log("Wasm version:", wasm_version)

    if (!wasm_version || wasm_version === version) {
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
      if (wasm_version === version) {
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
      <CardBody borderTop="1px" borderColor="gray.200">
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
        </Stack>
        {wasmHash ? (
          <Stack>
            <Stack
              direction="row"
              spacing={2}
              mt={4}
              justify="space-between"
              align="center"
            >
              <Text fontSize="sm" fontWeight="semibold">
                Wasm Hash:
              </Text>
              <Address address={wasmHash} />
            </Stack>
            <Stack direction="row" spacing={2} mt={4}>
              <Button onClick={resetWasm} flex={2} colorScheme="red">
                Reset
              </Button>
              <Button onClick={upgradeCanister} flex={10} colorScheme="orange">
                Upgrade
              </Button>
            </Stack>
          </Stack>
        ) : (
          <Progress hasStripe value={progress} mt={4} />
        )}
      </CardBody>
    </Stack>
  )
}

export default Wasm
