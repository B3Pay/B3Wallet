import {
  CheckIcon,
  CloseIcon,
  DeleteIcon,
  EditIcon,
  RepeatIcon
} from "@chakra-ui/icons"
import {
  AccordionButton,
  AccordionIcon,
  Avatar,
  Box,
  Flex,
  Heading,
  IconButton,
  Input,
  Stack,
  Text,
  Toast
} from "@chakra-ui/react"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import React, { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"
import { Loadings } from "."

interface AccountTitleProps {
  name: string
  environment: Environment
  id: string
  actor: B3Wallet | B3BasicWallet
  setLoadings: React.Dispatch<React.SetStateAction<Loadings>>
  refetchAccount: () => void
}

const AccountTitle: React.FC<AccountTitleProps> = ({
  name,
  environment,
  id,
  actor,
  setLoadings,
  refetchAccount
}) => {
  const [newName, setNewName] = useState<string>(name)
  const [editMode, setEditMode] = useState<boolean>(false)

  const removeAccount = async () => {
    setLoadings(prev => ({ ...prev, global: true }))

    actor
      .account_remove(id)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
        refetchAccount()
      })
      .catch(e => {
        Toast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoadings(prev => ({ ...prev, global: false }))
      })
  }

  return (
    <Stack alignItems="center" justify="space-between" direction="row">
      <Flex flex="10" gap="2" alignItems="center" zIndex={1} overflow="hidden">
        <Avatar size="sm" name={name} />
        {editMode ? (
          <Input
            type="text"
            value={newName}
            onChange={e => setNewName(e.target.value)}
          />
        ) : (
          <Box>
            <Heading size="xs">{name}</Heading>
            <Text fontSize="12">{Object.keys(environment)}</Text>
          </Box>
        )}
        <IconButton
          variant="ghost"
          colorScheme="blue"
          aria-label="Edit account name"
          icon={editMode ? <CheckIcon /> : <EditIcon />}
          onClick={async () => {
            if (editMode) {
              await actor.account_rename(id, newName)

              refetchAccount()
              setEditMode(false)
            } else setEditMode(true)
          }}
        />
        {editMode ? (
          <IconButton
            variant="ghost"
            colorScheme="red"
            aria-label="Edit account name"
            icon={<CloseIcon />}
            onClick={() => {
              setNewName(name)
              setEditMode(false)
            }}
          />
        ) : null}
      </Flex>
      <Stack direction="row" flex="2" justify="end">
        <IconButton
          size="xs"
          colorScheme="blue"
          aria-label="refetchAccount account"
          icon={<RepeatIcon />}
          onClick={refetchAccount}
        />
        <IconButton
          size="xs"
          aria-label="Remove account"
          colorScheme="red"
          icon={<DeleteIcon />}
          onClick={removeAccount}
        />
      </Stack>
      <AccordionButton borderRadius="md" width={50}>
        <AccordionIcon />
      </AccordionButton>
    </Stack>
  )
}

export default AccountTitle
