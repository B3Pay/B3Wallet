import { CheckIcon, CloseIcon } from "@chakra-ui/icons"
import {
  Button,
  ModalBody,
  ModalFooter,
  Stack,
  Stat,
  StatHelpText,
  StatLabel,
  Text
} from "@chakra-ui/react"
import { Principal } from "@dfinity/principal"
import { useCallback } from "react"
import { PendingRequest } from "../../../declarations/b3_wallet/b3_wallet.did"
import Parent from "../Recursive"
import Address from "./Address"

interface RequestItemProps extends PendingRequest {
  rejectHandler: (request_id: bigint) => void
  confirmHandler: (request_id: bigint) => void
}

const date = (timestamp?: bigint) => {
  if (!timestamp) return new Date()

  const time = timestamp / BigInt(1e6)
  return new Date(Number(time))
}

const RequestItem: React.FC<RequestItemProps> = ({
  consent_message,
  created_at,
  deadline,
  role,
  responses,
  allowed_signers,
  version,
  id,
  rejectHandler,
  confirmHandler
}) => {
  const isVotedBySigner = useCallback(
    (signer: Principal) => {
      return responses.reduce(
        (acc, [responseSigner, response]) => {
          if (responseSigner.toString() === signer.toString()) {
            if ("Confirm" in response) {
              acc.isConfirmed = true
              acc.isVoted = true
            } else if ("Reject" in response) {
              acc.isConfirmed = false
              acc.isVoted = true
            }
          }
          return acc
        },
        { isVoted: false, isConfirmed: false }
      )
    },
    [responses]
  )

  return (
    <Stack>
      <ModalBody>
        <Stat>
          <StatLabel>ID: &nbsp;</StatLabel>
          <StatHelpText>{id.toString()}</StatHelpText>
        </Stat>
        {Object.entries(consent_message).map(([key, value]) => (
          <Parent key={key} parent={key} child={value} />
        ))}
        <Stat>
          <StatLabel>Create at: &nbsp;</StatLabel>
          <StatHelpText>
            {date(created_at).toLocaleDateString()}{" "}
            {date(created_at).toLocaleTimeString()}
          </StatHelpText>
        </Stat>
        <Stat>
          <StatLabel>Deadline: &nbsp;</StatLabel>
          <StatHelpText>
            {date(deadline).toLocaleDateString()}{" "}
            {date(deadline).toLocaleTimeString()}
          </StatHelpText>
        </Stat>
        <Stat>
          <StatLabel>Role: &nbsp;</StatLabel>
          <StatHelpText>{Object.keys(role)[0]}</StatHelpText>
        </Stat>
        <Stat>
          <StatLabel>Responses: &nbsp;</StatLabel>
          <StatHelpText>
            {responses.length} / {allowed_signers.length}{" "}
          </StatHelpText>
        </Stat>
        <Stat>
          <StatLabel>Allowed Signers: &nbsp;</StatLabel>
          {allowed_signers.map((signer, index) => {
            const { isVoted, isConfirmed } = isVotedBySigner(signer)
            console.log(isVoted, isConfirmed)
            return (
              <StatHelpText key={index}>
                <Address address={signer.toString()}>
                  {isVoted ? (
                    isConfirmed ? (
                      <CheckIcon color="green" mr={2} />
                    ) : (
                      <CloseIcon color="red" mr={2} />
                    )
                  ) : null}
                </Address>
              </StatHelpText>
            )
          })}
        </Stat>
      </ModalBody>
      <ModalFooter borderTop="1px" borderColor="gray.200">
        <Text flex={3} fontSize="xs">
          {version}
        </Text>
        <Button
          flex={3}
          colorScheme="red"
          mr={2}
          onClick={() => rejectHandler(id)}
        >
          Reject
        </Button>
        <Button colorScheme="green" flex={3} onClick={() => confirmHandler(id)}>
          Confirm
        </Button>
      </ModalFooter>
    </Stack>
  )
}

export default RequestItem
