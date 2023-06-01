/* eslint-disable no-unused-vars */
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
  AccordionPanel,
  Avatar,
  Box,
  Button,
  Flex,
  Heading,
  IconButton,
  Input,
  Stack,
  Text
} from "@chakra-ui/react"

import { Chains, WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import { ethers, providers } from "ethers"
import { isAddress } from "ethers/lib/utils"
import { useCallback, useEffect, useState } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../Loading"
import { Chain } from "./Chain"
import ChainsSelect from "./ChainSelect"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface AccountProps extends WalletAccountView {
  actor: B3Wallet
  loading: boolean
  isExpanded: boolean
  refresh: () => void
}

interface Balances {
  EVM: bigint
  BTC: bigint
  ICP: bigint
}

const Account: React.FC<AccountProps> = ({
  actor,
  id,
  name,
  loading,
  refresh,
  addresses,
  environment,
  isExpanded
}) => {
  const [loadings, setLoadings] = useState({
    global: false,
    EVM: false,
    BTC: false,
    ICP: false
  })
  const [balances, setBalances] = useState<Balances>({
    EVM: 0n,
    BTC: 0n,
    ICP: 0n
  })
  const [newName, setNewName] = useState<string>(name)
  const [editMode, setEditMode] = useState<boolean>(false)

  const getEthBalance = useCallback(async () => {
    const address = ""
    if (isAddress(address) === false) {
      return
    }

    setLoadings(prev => ({ ...prev, EVM: true }))
    provider
      .getBalance(address)
      .then(balance => {
        setBalances(prev => ({ ...prev, ETH: balance.toBigInt() }))
        setLoadings(prev => ({ ...prev, EVM: false }))
      })
      .catch(err => {
        console.log(err)
        setLoadings(prev => ({ ...prev, EVM: false }))
      })
  }, [])

  const getBtcBalance = useCallback(async () => {
    setLoadings(prev => ({ ...prev, BTC: true }))
    if (addresses.length <= 1) {
      return
    }

    actor
      .account_balance_btc(id, { Regtest: null }, [])
      .then(balance => {
        setBalances(prev => ({ ...prev, BTC: balance }))
        setLoadings(prev => ({ ...prev, BTC: false }))
      })
      .catch(err => {
        console.log(err)
        setLoadings(prev => ({ ...prev, BTC: false }))
      })
  }, [actor, id, addresses])

  const getIcpBalance = useCallback(async () => {
    setLoadings(prev => ({ ...prev, ICP: true }))
    actor
      .account_icp_balance(id, [])
      .then(balance => {
        setBalances(prev => ({ ...prev, ICP: balance.e8s }))
        setLoadings(prev => ({ ...prev, ICP: false }))
      })
      .catch(err => {
        console.log(err)
        setLoadings(prev => ({ ...prev, ICP: false }))
      })
  }, [actor, id])

  const handleEthTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ETH from ${from} to ${to}`)
      setLoadings(prev => ({ ...prev, EVM: true }))

      const nonce = await provider.getTransactionCount(from)
      const gasPrice = await provider.getGasPrice().then(s => s.toHexString())
      const value = ethers.utils.parseEther(amount.toString()).toHexString()
      const data = "0x00"
      const gasLimit = ethers.BigNumber.from("24000").toHexString()
      const transaction = {
        nonce,
        gasPrice,
        gasLimit,
        to,
        value,
        data
      }

      try {
        const serializeTx = Buffer.from(
          ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
          "hex"
        )

        console.log(serializeTx)

        setLoadings(prev => ({ ...prev, EVM: false }))

        console.log({ title: "Signing transaction...", variant: "subtle" })

        const res = await actor.request_sign_transaction(
          id,
          [...serializeTx],
          97n
        )

        console.log(res)
      } catch (error) {
        console.log(error)

        setLoadings(prev => ({ ...prev, EVM: false }))
      }

      setTimeout(() => {
        getEthBalance()
      }, 2000)
    },
    [actor, getEthBalance, id]
  )

  const handleBtcTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} BTC from ${from} to ${to}`)
      setLoadings(prev => ({ ...prev, BTC: true }))

      await actor
        .request_transfer_btc(
          {
            account_id: id,
            to,
            network: { Regtest: null },
            amount
          },
          []
        )
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, BTC: false }))

          setTimeout(() => {
            getBtcBalance()
          }, 2000)
        })
        .catch(err => {
          console.log(err)
          setLoadings(prev => ({ ...prev, BTC: false }))
        })
    },
    [actor, getBtcBalance, id]
  )

  const handleIcpTransfer = useCallback(
    async (from: string, to: string, amount: bigint) => {
      console.log(`Transfering ${amount} ICP from ${from} to ${to}`)
      const tokenAmount = {
        e8s: BigInt(amount)
      }

      setLoadings(prev => ({ ...prev, ICP: true }))

      await actor
        .account_send_icp(id, to, tokenAmount, [], [])
        .then(res => {
          console.log(res)

          setLoadings(prev => ({ ...prev, ICP: false }))

          setTimeout(() => {
            getIcpBalance()
          }, 2000)
        })
        .catch(err => {
          console.log(err)
          setLoadings(prev => ({ ...prev, ICP: false }))
        })
    },
    [actor, getIcpBalance, id]
  )

  const handleTransfer = {
    EVM: handleEthTransfer,
    BTC: handleBtcTransfer,
    ICP: handleIcpTransfer
  }

  const handleBalance = {
    EVM: getEthBalance,
    BTC: getBtcBalance,
    ICP: getIcpBalance
  }

  useEffect(() => {
    if (!isExpanded) {
      return
    }
    getEthBalance()
    getBtcBalance()
    getIcpBalance()
  }, [getEthBalance, getBtcBalance, getIcpBalance, isExpanded])

  const requestPublicKey = async () => {
    setLoadings(prev => ({ ...prev, global: true }))
    actor
      .account_request_public_key(id)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
        refresh()
      })
      .catch(e => {
        console.log(e)
        refresh()
        setLoadings(prev => ({ ...prev, global: false }))
      })
  }

  const removeAccount = async () => {
    setLoadings(prev => ({ ...prev, global: true }))

    actor
      .account_remove(id)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
        refresh()
      })
      .catch(e => {
        console.log(e)
        setLoadings(prev => ({ ...prev, global: false }))
      })
  }

  const handleAddressRemove = async (chain: string, network: string) => {
    setLoadings(prev => ({ ...prev, global: true }))
    let networkObject
    if (chain === "BTC") {
      networkObject = { [network]: null }
    } else if (chain === "EVM") {
      networkObject = BigInt(network)
    } else if (chain === "SNS") {
      networkObject = network
    } else {
      networkObject = null
    }

    actor
      .account_remove_address(id, {
        [chain]: networkObject
      } as Chains)
      .then(() => {
        setLoadings(prev => ({ ...prev, global: false }))
        refresh()
      })
      .catch(e => {
        setLoadings(prev => ({ ...prev, global: false }))
        console.log(e)
      })
  }

  const noPublickey = addresses.length === 1

  return (
    <Box position="relative">
      {(loadings.global || loading) && <Loading />}
      <Stack alignItems="center" justify="space-between" direction="row">
        <Flex
          flex="10"
          gap="2"
          alignItems="center"
          zIndex={1}
          overflow="hidden"
        >
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
                const renameArgs = {
                  account_id: id,
                  new_name: newName
                }

                await actor.request_account_rename(renameArgs, [])
                setNewName(newName)
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
            colorScheme="blue"
            aria-label="Refresh account"
            icon={<RepeatIcon />}
            onClick={refresh}
          />
          <IconButton
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
      <AccordionPanel px={0} fontSize="14">
        <Stack spacing="2">
          {!noPublickey && (
            <ChainsSelect account_id={id} actor={actor} refresh={refresh} />
          )}
          {addresses.map((item, index) => {
            const symbol = Object.keys(item[0])[0] as keyof Balances
            const chains = Object.values(item[0])[0]
            const network =
              typeof chains === "bigint"
                ? chains.toString()
                : typeof chains === "object"
                ? chains === null
                  ? undefined
                  : Object.keys(chains)[0]
                : chains

            return (
              <Chain
                key={index}
                chain={symbol}
                address={item[1]}
                balance={balances[symbol]}
                network={network}
                loading={loadings[symbol]}
                handleTransfer={handleTransfer[symbol]}
                handleBalance={handleBalance[symbol]}
                handlerAddressRemove={handleAddressRemove}
              />
            )
          })}
          {noPublickey && (
            <Button onClick={requestPublicKey} isLoading={loadings.global}>
              Request PublicKey
            </Button>
          )}
        </Stack>
      </AccordionPanel>
    </Box>
  )
}

export default Account
