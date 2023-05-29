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
  Stat,
  StatHelpText,
  StatLabel,
  useDisclosure
} from "@chakra-ui/react"
import { PendingRequest } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3Wallet } from "service/actor"

const parent = (key: string, value: any) =>
  value && typeof value === "object" ? (
    <Stat key={key}>
      <StatLabel>{key}: &nbsp;</StatLabel>
      {child(value)}
    </Stat>
  ) : (
    <Stat key={key}>
      <StatLabel>{key}: &nbsp;</StatLabel>
      <StatHelpText>{value?.toString()}</StatHelpText>
    </Stat>
  )

const child = (value: any) =>
  value &&
  (value._isPrincipal ? (
    value.toText()
  ) : typeof value === "object" ? (
    Array.isArray(value) || typeof value[0] === "number" ? (
      value.toString()
    ) : (
      <Box ml={2}>
        {Object.entries(value).map(([key, value]) => parent(key, value))}
      </Box>
    )
  ) : (
    value.toString()
  ))

interface Request {
  type: string
  details: string
}

interface ConfirmationModalProps {
  actor: B3Wallet
  request: Request
}

const ConfirmationModal: React.FC<ConfirmationModalProps> = ({ actor }) => {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [requests, setRequests] = useState<PendingRequest[]>([])

  // useInterval(() => {
  //   // Call your function to check for new requests here.
  //   // If a new request is found, update the state.
  //   actor.get_requests().then(newRequests => {
  //     console.log(newRequests)
  //     if (newRequests.length > 0) {
  //       setRequests(newRequests)
  //       // Open modal here to confirm or reject new request
  //     }
  //   })
  // }, 10000) // Run every 5 seconds when isChecking is true

  const fetchRequests = async () => {
    actor.get_requests().then(newRequests => {
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
              {Object.entries(requests[0].request).map(([key, value]) =>
                parent(key, value)
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
