import { InfoOutlineIcon } from "@chakra-ui/icons"
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
  useDisclosure
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

  const fetchRequests = async () => {
    actor.get_pending_list().then(newRequests => {
      console.log(newRequests)
      if (newRequests.length > 0) {
        setRequests(newRequests)
        // Open modal here to confirm or reject new request
        onOpen()
      }
    })
  }

  const confirmHandler = async (request_id: bigint) => {
    actor.request_response(request_id, { Confirm: null }).then(() => {
      onClose()
      fetchAccounts()
    })
  }

  const rejectHandler = async (request_id: bigint) => {
    actor.request_response(request_id, { Reject: null }).then(() => {
      onClose()
    })
  }

  return (
    <Box>
      <IconButton
        colorScheme="orange"
        aria-label="Confirm"
        icon={<InfoOutlineIcon />}
        onClick={fetchRequests}
      />
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        {requests.length > 0 && (
          <ModalContent>
            <ModalHeader>Request ID: {requests[0].id.toString()}</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              {Object.entries(requests[0].request).map(([key, value]) => (
                <Parent key={key} parent={key} child={value} />
              ))}
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
