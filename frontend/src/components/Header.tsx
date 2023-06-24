import { SettingsIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
  Card,
  FormControl,
  FormLabel,
  Heading,
  IconButton,
  Image,
  Input,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Select,
  Stack
} from "@chakra-ui/react"
import { ManagementCanisterRecord } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import useToastMessage from "hooks/useToastMessage"
import { useCallback, useEffect, useState } from "react"
import { B3System, CanisterStatus } from "service"
import Error from "./Error"
import Loading from "./Loading"
import CanisterControllers from "./Wallet/Setting/CanisterController"

interface HeaderProps {
  getManagmentActor: () => Promise<any>
  systemActor?: B3System
}

const Header: React.FC<HeaderProps> = ({ getManagmentActor, systemActor }) => {
  const [error, setError] = useState("")
  const [isOpen, setIsOpen] = useState(false)
  const [loading, setLoading] = useState(false)

  const [status, setStatus] = useState<CanisterStatus>()
  const [canisterId, setCanisterId] = useState<string>()
  const [canisterIds, setCanisterIds] = useState<Principal[]>([])

  const [canisterIdInput, setCanisterIdInput] = useState("")

  const [controllers, setControllers] = useState<Principal[]>([])
  const [managementActor, setManagementActor] =
    useState<ManagementCanisterRecord>()

  const errorToast = useToastMessage()

  const fetchCanisterIds = useCallback(async () => {
    if (!systemActor) return

    setLoading(true)

    systemActor
      ?.get_canister()
      .then(({ canisters }) => {
        console.log(canisters[0])

        setCanisterIds(canisters)
        const walletCanisterId = canisters[0].toString()

        setCanisterId(walletCanisterId)
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
  }, [systemActor])

  useEffect(() => {
    if (!getManagmentActor) return

    getManagmentActor()?.then(setManagementActor)

    fetchCanisterIds()
  }, [fetchCanisterIds, getManagmentActor])

  const fetchHandler = async (principal: string) => {
    if (!managementActor) return

    let signerId: Principal

    try {
      signerId = Principal.fromText(principal)
    } catch (e) {
      return errorToast({
        title: "Invalid Principal",
        description: e.message,
        status: "error",
        duration: 9000,
        isClosable: true
      })
    }

    setLoading(true)

    await managementActor
      .canister_status({ canister_id: signerId })
      .then(setStatus)

    setControllers(controllers)
    setLoading(false)
  }

  const addCanister = async (canisterId: string) => {
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
        setLoading(false)
        fetchCanisterIds()
      })
      .catch(e => {
        console.log(e)
        setError(e)
        setLoading(false)
      })
  }

  const onClose = () => setIsOpen(false)
  const onOpen = () => setIsOpen(true)

  return (
    <Card mb={2}>
      <Stack
        direction="row"
        justifyContent="space-between"
        alignItems="center"
        px={2}
      >
        <IconButton
          aria-label="B3Wallet Logo"
          variant="ghost"
          icon={<Image boxSize="35px" src="logo.svg"></Image>}
        />
        <Heading size="lg" textAlign="center" my={2}>
          B3Wallet Demo
        </Heading>
        {canisterIds.length > 0 ? (
          <IconButton
            aria-label="Settings"
            variant="ghost"
            colorScheme="blue"
            icon={<SettingsIcon />}
            onClick={onOpen}
          />
        ) : (
          <Box />
        )}
      </Stack>
      <Modal isOpen={isOpen} onClose={onClose} size="xl">
        <ModalOverlay />
        <ModalContent position="relative">
          {loading && <Loading title="Loading Wallet" />}
          <ModalHeader>Settings</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            {error && (
              <Error error={error} mb={2} borderRadius="base" shadow="base" />
            )}
            <Stack spacing={4} pb={4}>
              <FormControl id="addWallet">
                <FormLabel>Add Wallet Canister</FormLabel>
                <Stack direction="row">
                  <Input
                    placeholder="Canister ID"
                    value={canisterIdInput}
                    onChange={e => setCanisterIdInput(e.target.value)}
                  />
                  <Button onClick={() => addCanister(canisterIdInput)}>
                    Add
                  </Button>
                </Stack>
              </FormControl>
              <FormControl id="walletName">
                <FormLabel>Your Wallet Canisters</FormLabel>
                <Select
                  placeholder="Select wallet"
                  onChange={e => setCanisterId(e.target.value)}
                  value={canisterId?.toString()}
                >
                  {canisterIds.map(canisterId => (
                    <option
                      key={canisterId.toString()}
                      value={canisterId.toString()}
                    >
                      {canisterId.toString()}
                    </option>
                  ))}
                </Select>
              </FormControl>
              <Button onClick={() => fetchHandler(canisterId)}>Fetch</Button>
            </Stack>
            {status && (
              <CanisterControllers
                status={status}
                setStatus={setStatus}
                actor={managementActor}
                canisterId={canisterId}
              />
            )}
          </ModalBody>
          <ModalFooter>
            <Button variant="ghost" onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Card>
  )
}

export default Header
