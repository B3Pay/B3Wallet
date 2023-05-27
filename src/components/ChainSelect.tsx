/* eslint-disable no-unused-vars */
import { Chains } from "declarations/b3_wallet/b3_wallet.did"
import { useState } from "react"
import { B3User } from "service/actor"

const inputStyle = {
  flex: 4,
  height: 40,
  fontSize: 16,
  border: "1px solid black",
  borderRadius: 2,
  padding: "0 10px",
  marginRight: 10
}

const chains = ["BTC", "EVM", "SNS"]
const btcNetworks = ["Mainnet", "Testnet", "Regtest"]

interface ChainSelectProps {
  actor: B3User
  account_id: string
}

const ChainSelect: React.FC<ChainSelectProps> = ({ account_id, actor }) => {
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
      })
      .catch(err => {
        console.log(err)
        setLoading(false)
      })
  }

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        width: "100%"
      }}
    >
      <select style={inputStyle} value={chain} onChange={handleChainChange}>
        <option value="">Select a chain</option>
        {chains.map((chain, index) => (
          <option key={index} value={chain}>
            {chain}
          </option>
        ))}
      </select>

      {chain === "BTC" && (
        <select
          style={inputStyle}
          value={network}
          onChange={handleNetworkChange}
        >
          <option value="">Select a network</option>
          {btcNetworks.map((network, index) => (
            <option key={index} value={network}>
              {network}
            </option>
          ))}
        </select>
      )}

      {chain === "EVM" && (
        <input
          style={inputStyle}
          type="text"
          value={evmChainId}
          onChange={handleEvmChainIdChange}
          placeholder="EVM Chain ID"
        />
      )}

      {chain === "SNS" && (
        <input
          style={inputStyle}
          type="text"
          value={snsName}
          onChange={handleSnsNameChange}
          placeholder="SNS Name"
        />
      )}

      <button
        style={{
          flex: 4
        }}
        onClick={generateAddress}
      >
        {loading
          ? `Generating ${chain} address...`
          : `Generate ${chain} address`}
      </button>
    </div>
  )
}

export default ChainSelect
