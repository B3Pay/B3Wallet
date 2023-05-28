/* eslint-disable no-unused-vars */
import { Button, Flex, Input, Select, Stack } from "@chakra-ui/react"
import { Chains } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3User } from "service/actor"

const chains = ["BTC", "EVM", "SNS"]
const btcNetworks = ["Mainnet", "Testnet", "Regtest"]

interface ChainSelectProps {
  actor: B3User
  account_id: string
  refresh: () => void
}

const ChainSelect: React.FC<ChainSelectProps> = ({
  account_id,
  actor,
  refresh
}) => {
  const [chain, setChain] = useState("")
  const [network, setNetwork] = useState("")
  const [evmChainId, setEvmChainId] = useState("")
  const [snsName, setSnsName] = useState("")
  const [loading, setLoading] = useState(false)

  const handleChainChange = (e: any) => {
    setChain(e.target.value)
    if (e.target.value !== "BTC") {
      setNetwork("")
    }
  }

  const handleNetworkChange = (e: any) => {
    setNetwork(e.target.value)
  }

  const handleEvmChainIdChange = (e: any) => {
    setEvmChainId(e.target.value)
  }

  const handleSnsNameChange = (e: any) => {
    setSnsName(e.target.value)
  }

  const generateAddress = async () => {
    setLoading(true)
    let networkObject
    if (chain === "BTC") {
      networkObject = { [network]: null }
    } else if (chain === "EVM") {
      networkObject = BigInt(evmChainId)
    } else if (chain === "SNS") {
      networkObject = snsName
    } else {
      networkObject = null
    }

    actor
      .account_generate_address(account_id, {
        [chain]: networkObject
      } as Chains)
      .then(() => {
        setLoading(false)
        refresh()
      })
      .catch(err => {
        console.log(err)
        setLoading(false)
      })
  }

  return (
    <Stack spacing="2" direction="row" justify="space-between" align="center">
      <Select value={chain} onChange={handleChainChange}>
        <option value="">Select a chain</option>
        {chains.map((chain, index) => (
          <option key={index} value={chain}>
            {chain}
          </option>
        ))}
      </Select>
      {chain === "BTC" && (
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
          value={evmChainId}
          onChange={handleEvmChainIdChange}
          placeholder="EVM Chain ID"
        />
      )}
      {chain === "SNS" && (
        <Input
          type="text"
          value={snsName}
          onChange={handleSnsNameChange}
          placeholder="SNS Name"
        />
      )}
      <Flex>
        <Button onClick={generateAddress} isLoading={loading}>
          Generate {chain} address
        </Button>
      </Flex>
    </Stack>
  )
}

export default ChainSelect
