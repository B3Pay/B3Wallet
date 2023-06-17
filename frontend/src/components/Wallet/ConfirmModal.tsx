import { InfoOutlineIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
  IconButton,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalHeader,
  ModalOverlay,
  Stack,
  Stat,
  StatHelpText,
  StatLabel,
  Text,
  useDisclosure
} from "@chakra-ui/react"
import { useMemo, useState } from "react"
import { PendingRequest } from "../../../declarations/b3_wallet/b3_wallet.did"
import { B3Wallet } from "../../service/actor"
import Parent from "../Recursive"

interface ConfirmationModalProps {
  actor: B3Wallet
  fetchAccounts: () => void
  refreshWallet: () => void
}

const ConfirmationModal: React.FC<ConfirmationModalProps> = ({
  actor,
  fetchAccounts,
  refreshWallet
}) => {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [requests, setRequests] = useState<PendingRequest[]>([])

  const fetchRequests = async () =>
    actor.get_pending_list().then(newRequests => {
      setRequests(newRequests)
    })

  const modalOpenHeader = () => {
    if (requests.length > 0) {
      onOpen()
    } else {
      fetchRequests()
    }
  }

  const confirmHandler = async (request_id: bigint) => {
    actor.response(request_id, { Confirm: null }).then(async () => {
      onClose()
      await fetchRequests()
      fetchAccounts()
    })
  }

  const rejectHandler = async (request_id: bigint) => {
    actor.response(request_id, { Reject: null }).then(() => {
      onClose()
      fetchRequests()
    })
  }

  console.log({ requests })

  // useInterval(async () => {
  //   fetchRequests()
  // }, 10000)

  const date = useMemo(() => {
    if (!requests[0]) return new Date()

    const time = requests[0].deadline / BigInt(1e6)
    return new Date(Number(time))
  }, [requests[0]])

  return (
    <Box>
      <IconButton
        colorScheme="red"
        variant={requests.length > 0 ? "solid" : "ghost"}
        aria-label="Confirm"
        icon={<InfoOutlineIcon />}
        onClick={modalOpenHeader}
      />
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        {requests.length > 0 && (
          <ModalContent>
            <ModalHeader>{requests[0].consent_message.message}</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              {Object.entries(requests[0].consent_message).map(
                ([key, value]) => (
                  <Parent key={key} parent={key} child={value} />
                )
              )}
              <Stat>
                <StatLabel>Deadline: &nbsp;</StatLabel>
                <StatHelpText>
                  {date.toLocaleDateString()} {date.toLocaleTimeString()}
                </StatHelpText>
              </Stat>
              <Stat>
                <StatLabel>Role: &nbsp;</StatLabel>
                <StatHelpText>{Object.keys(requests[0].role)[0]}</StatHelpText>
              </Stat>
              <Stat>
                <StatLabel>Responses: &nbsp;</StatLabel>
                <StatHelpText>
                  {requests[0].responses.length} / {}{" "}
                </StatHelpText>
              </Stat>
              {Object.entries(requests[0].request).map(([key, value]) => (
                <Parent key={key} parent={key} child={value} />
              ))}
            </ModalBody>
            <Stack direction="row" p={4} align="center" justify="space-between">
              <Text flex={2} fontSize="xs">
                {requests[0].version}
              </Text>
              <Button
                flex={3}
                colorScheme="red"
                onClick={() => rejectHandler(requests[0].id)}
              >
                Reject
              </Button>
              <Button
                colorScheme="blue"
                flex={3}
                onClick={() => confirmHandler(requests[0].id)}
              >
                Confirm
              </Button>
            </Stack>
          </ModalContent>
        )}
      </Modal>
    </Box>
  )
}

export default ConfirmationModal
