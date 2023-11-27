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
import { B3Wallet } from "service"

interface AccountTitleProps {
  name: string
  hidden: boolean
  isExpanded: boolean
  environment: Environment
  id: string
  actor: B3Wallet
  refetchAccount: () => void
  refetchAccounts: () => void
}

const AccountTitle: React.FC<AccountTitleProps> = ({
  name,
  hidden,
  environment,
  id,
  actor,
  isExpanded,
  refetchAccount,
  refetchAccounts
}) => {
  const [newName, setNewName] = useState<string>(name)
  const [editMode, setEditMode] = useState<boolean>(false)
  const [loadings, setLoadings] = useState({
    remove: false,
    rename: false,
    hide: false
  })

  const removeAccount = async () => {
    setLoadings(prev => ({ ...prev, remove: true }))

    try {
      await actor.account_remove(id)
      refetchAccounts()
    } catch (e) {
      Toast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoadings(prev => ({ ...prev, remove: false }))
    }
  }

  const hideAccount = async () => {
    setLoadings(prev => ({ ...prev, hide: true }))

    try {
      // @ts-ignore
      if (hidden) await actor.account_show(id)
      else await actor.account_hide(id)

      refetchAccount()
    } catch (e) {
      Toast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoadings(prev => ({ ...prev, hide: false }))
    }
  }

  const renameAccount = async () => {
    setLoadings(prev => ({ ...prev, rename: true }))

    try {
      await actor.account_rename(id, newName)
      refetchAccount()
      setEditMode(false)
    } catch (e) {
      Toast({
        title: "Error",
        description: e.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    } finally {
      setLoadings(prev => ({ ...prev, rename: false }))
    }
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
          isLoading={loadings.rename}
          icon={editMode ? <CheckIcon /> : <EditIcon />}
          onClick={async () => {
            if (editMode) renameAccount()
            else setEditMode(true)
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
      {isExpanded && (
        <Stack direction="row" flex="2" justify="end">
          <IconButton
            size="xs"
            colorScheme="blue"
            aria-label="refetchAccount account"
            icon={<RepeatIcon />}
            onClick={refetchAccount}
          />
          {/* <IconButton
            size="xs"
            aria-label="Hide account"
            colorScheme="orange"
            icon={hidden ? <ViewIcon /> : <ViewOffIcon />}
            onClick={hideAccount}
          /> */}
          <IconButton
            size="xs"
            aria-label="Remove account"
            colorScheme="red"
            icon={<DeleteIcon />}
            onClick={removeAccount}
          />
        </Stack>
      )}
      <AccordionButton borderRadius="md" width={50}>
        <AccordionIcon />
      </AccordionButton>
    </Stack>
  )
}

export default AccountTitle
