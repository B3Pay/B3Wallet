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

type CanisterStatuses = {
  [key in string]: CanisterStatus
}

interface HeaderProps {
  getManagmentActor: () => Promise<any>
  fetchUserActor: (id: string) => void
  systemActor?: B3System
}

const Header: React.FC<HeaderProps> = ({
  getManagmentActor,
  fetchUserActor,
  systemActor
}) => {
  const [error, setError] = useState("")
  const [isOpen, setIsOpen] = useState(false)
  const [loading, setLoading] = useState(false)

  const [statuses, setStatuses] = useState<CanisterStatuses>()
  const [selectedCanisterId, setSelectedCanisterId] = useState<string>()
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
      .get_canisters()
      .then(canisters => {
        setCanisterIds(canisters)

        const walletCanisterId =
          localStorage.getItem("walletCanisterId") || canisters[0].toString()

        setSelectedCanisterId(walletCanisterId)
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

    let canister_id: Principal

    try {
      canister_id = Principal.fromText(principal)
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

    try {
      await managementActor
        .canister_status({ canister_id })
        .then(status => setStatuses(prev => ({ ...prev, [principal]: status })))

      setControllers(controllers)
    } catch (e) {
      console.log(e)
    } finally {
      setLoading(false)
    }
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
            <Stack spacing={4}>
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
                  onChange={e => setSelectedCanisterId(e.target.value)}
                  value={selectedCanisterId?.toString()}
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
              <Button onClick={() => fetchHandler(selectedCanisterId)}>
                Fetch
              </Button>
            </Stack>
            {statuses &&
              Object.entries(statuses)?.map(
                ([canisterId, status]) =>
                  canisterId === selectedCanisterId && (
                    <CanisterControllers
                      {...status}
                      key={canisterId}
                      setParentControllers={(controllers: Array<Principal>) => {
                        setStatuses(prev => ({
                          ...prev,
                          [canisterId]: {
                            ...prev[canisterId],
                            controllers
                          }
                        }))
                      }}
                      actor={managementActor}
                      canisterId={canisterId}
                    />
                  )
              )}
          </ModalBody>
          <ModalFooter justifyContent="space-between">
            <Button
              flex={10}
              colorScheme="blue"
              mr={3}
              onClick={() => {
                onClose()
                localStorage.setItem("walletCanisterId", selectedCanisterId)
                window.location.reload()
              }}
            >
              Use
            </Button>
            <Button
              flex={2}
              variant="solid"
              colorScheme="red"
              onClick={onClose}
            >
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Card>
  )
}

export default Header
