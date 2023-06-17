/* eslint-disable no-unused-vars */
import { Button, Flex, Input, Select, Stack } from "@chakra-ui/react"
import { IS_LOCAL } from "helpers/config"
import { ChainSymbol, handleChainType } from "helpers/utiles"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service/actor"

const chains: ChainSymbol[] = ["BTC", "CKBTC", "EVM", "ICRC", "ICP"]
const btcNetworks = IS_LOCAL
  ? ["Mainnet", "Testnet", "Regtest"]
  : ["Mainnet", "Testnet"]

interface CreateAddressProps {
  actor: B3Wallet | B3BasicWallet
  accountId: string
  refetchAccount: () => void
}

const CreateAddress: React.FC<CreateAddressProps> = ({
  actor,
  accountId,
  refetchAccount
}) => {
  const [chain, setChain] = useState<ChainSymbol>()
  const [network, setNetwork] = useState("")

  const [loading, setLoading] = useState(false)
  const errorToast = useToastMessage()

  const handleChainChange = (e: any) => {
    setChain(e.target.value)

    setNetwork("")
  }

  const handleNetworkChange = (e: any) => {
    setNetwork(e.target.value)
  }

  const createAddress = async () => {
    setLoading(true)
    let chainType = handleChainType(network, chain)

    actor
      .account_create_address(accountId, chainType)
      .then(() => {
        setLoading(false)
        refetchAccount()
      })
      .catch(err => {
        errorToast({
          title: "Error",
          description: err.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoading(false)
      })
  }

  return (
    <Stack
      spacing="2"
      p={1}
      direction="row"
      justify="space-between"
      align="center"
    >
      <Select value={chain} onChange={handleChainChange}>
        <option value="">Select a chain</option>
        {chains.map((chain, index) => (
          <option key={index} value={chain}>
            {chain}
          </option>
        ))}
      </Select>
      {(chain === "BTC" || chain === "CKBTC") && (
        <Select value={network} onChange={handleNetworkChange}>
          <option value="">Select a network</option>
          {btcNetworks.map((network, index) => (
            <option key={index} value={network}>
              {network}
            </option>
          ))}
        </Select>
      )}
      {chain === "EVM" && (
        <Input
          type="text"
          value={network}
          onChange={handleNetworkChange}
          placeholder="EVM Chain ID"
        />
      )}
      {chain === "ICRC" && (
        <Input
          type="text"
          value={network}
          onChange={handleNetworkChange}
          placeholder="ICRC Address"
        />
      )}
      <Flex>
        <Button onClick={createAddress} isLoading={loading}>
          Create {chain} address
        </Button>
      </Flex>
    </Stack>
  )
}

export default CreateAddress
