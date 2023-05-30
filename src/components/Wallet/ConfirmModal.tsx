import { InfoOutlineIcon, RepeatIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
  IconButton,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  useDisclosure,
  useInterval
} from "@chakra-ui/react"
import { PendingRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3Wallet } from "service/actor"
import Parent from "../Parent"

interface ConfirmationModalProps {
  actor: B3Wallet
  fetchAccounts: () => void
}

const ConfirmationModal: React.FC<ConfirmationModalProps> = ({
  actor,
  fetchAccounts
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
    actor.request_response(request_id, { Confirm: null }).then(async () => {
      onClose()
      await fetchRequests()
      fetchAccounts()
    })
  }

  const rejectHandler = async (request_id: bigint) => {
    actor.request_response(request_id, { Reject: null }).then(() => {
      onClose()
      fetchRequests()
    })
  }

  useInterval(async () => {
    fetchRequests()
  }, 10000)

  return (
    <Box>
      <IconButton
        colorScheme="green"
        variant={"ghost"}
        aria-label="Refresh"
        icon={<RepeatIcon />}
        onClick={fetchAccounts}
      />
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
            <ModalHeader> {requests[0].consent_message.method}</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              {Object.entries(requests[0].consent_message).map(
                ([key, value]) => (
                  <Parent key={key} parent={key} child={value} />
                )
              )}
            </ModalBody>
            <ModalFooter>
              <Button
                colorScheme="blue"
                mr={3}
                onClick={() => confirmHandler(requests[0].id)}
              >
                Confirm
              </Button>
              <Button
                colorScheme="red"
                onClick={() => rejectHandler(requests[0].id)}
              >
                Reject
              </Button>
            </ModalFooter>
          </ModalContent>
        )}
      </Modal>
    </Box>
  )
}

export default ConfirmationModal
