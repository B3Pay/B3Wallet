import { ArrowBackIcon, ArrowForwardIcon } from "@chakra-ui/icons"
import {
  Box,
  Button,
  Modal,
  ModalCloseButton,
  ModalContent,
  ModalHeader,
  ModalOverlay,
  Stack,
  useDisclosure,
  useInterval
} from "@chakra-ui/react"
import { ProcessedRequest } from "declarations/b3_payment/b3_payment.did"
import { useState } from "react"
import ProcessedItem from "./ProcessedItem"

interface ProcessedModalProps {
  requests: ProcessedRequest[]
  fetchRequests: () => void
  checkRequest: (id: bigint) => Promise<void>
}

const ProcessedModal: React.FC<ProcessedModalProps> = ({
  requests,
  fetchRequests,
  checkRequest
}) => {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [index, setIndex] = useState(0)

  const modalOpenHeader = () => {
    if (requests.length > 0) {
      onOpen()
    } else {
      fetchRequests()
    }
  }

  useInterval(async () => {
    fetchRequests()
  }, 10000)

  return (
    <Box>
      <Button
        colorScheme="blue"
        variant={requests.length > 0 ? "solid" : "ghost"}
        aria-label="Confirm"
        onClick={modalOpenHeader}
      >
        Processed({requests.length})
      </Button>
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Processed Requests ({requests.length})</ModalHeader>
          <ModalCloseButton />
          {requests.length > 1 && (
            <Stack direction="row" spacing={4} align="center" mx={2}>
              <Button
                width="100%"
                aria-label="Previous"
                variant="outline"
                onClick={() =>
                  setIndex(prev => {
                    if (prev > 0) {
                      return prev - 1
                    }
                    return prev
                  })
                }
                children={<ArrowBackIcon />}
                disabled={index === 0}
              />
              <Button
                width="100%"
                variant="outline"
                aria-label="Next"
                children={<ArrowForwardIcon />}
                onClick={() =>
                  setIndex(prev => {
                    if (prev + 1 < requests.length) {
                      return prev + 1
                    }
                    return prev
                  })
                }
                disabled={index === requests.length - 1}
              />
            </Stack>
          )}
          {requests[index] && <ProcessedItem {...requests[index]} />}
        </ModalContent>
      </Modal>
    </Box>
  )
}

export default ProcessedModal
