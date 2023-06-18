import {
  ArrowBackIcon,
  ArrowForwardIcon,
  InfoOutlineIcon
} from "@chakra-ui/icons"
import {
  Box,
  Button,
  IconButton,
  Modal,
  ModalCloseButton,
  ModalContent,
  ModalHeader,
  ModalOverlay,
  Stack,
  useDisclosure,
  useInterval
} from "@chakra-ui/react"
import Loading from "components/Loading"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { PendingRequest } from "../../../declarations/b3_wallet/b3_wallet.did"
import { B3Wallet } from "../../service/actor"
import RequestItem from "./RequestItem"

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
  const [index, setIndex] = useState(0)

  const [loading, setLoading] = useState(false)

  const errorToast = useToastMessage()

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
    setLoading(true)
    try {
      await actor.response(request_id, { Confirm: null })
      onClose()
      await fetchRequests()

      fetchAccounts()
    } catch (e) {
      console.log(e)
      errorToast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    }

    setLoading(false)
    refreshWallet()
  }

  const rejectHandler = async (request_id: bigint) => {
    setLoading(true)
    try {
      await actor.response(request_id, { Reject: null })
      onClose()
      await fetchRequests()

      fetchAccounts()
    } catch (e) {
      console.log(e)
      errorToast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    }

    setLoading(false)
  }

  useInterval(async () => {
    fetchRequests()
  }, 5000)

  return (
    <Box>
      {loading && <Loading title="Loading Wallet" />}
      <IconButton
        colorScheme="red"
        variant={requests.length > 0 ? "solid" : "ghost"}
        aria-label="Confirm"
        icon={<InfoOutlineIcon />}
        onClick={modalOpenHeader}
      />
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Pending Requests ({requests.length})</ModalHeader>
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
          {requests[index] && (
            <RequestItem
              {...requests[index]}
              rejectHandler={rejectHandler}
              confirmHandler={confirmHandler}
            />
          )}
        </ModalContent>
      </Modal>
    </Box>
  )
}

export default ConfirmationModal
