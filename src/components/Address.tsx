/* eslint-disable no-unused-vars */
import React, { useState } from "react"
import { B3User } from "service/actor"

const buttonStyle = {
  color: "white",
  border: "none",
  borderRadius: "5px",
  padding: "5px",
  width: "30px",
  height: "30px",
  cursor: "pointer"
}

interface AddressProps {
  actor: B3User
  balance: BigInt
  symbol: string
  address: string
  network: string
  handleTransfer: (from: string, to: string, amount: string) => Promise<void>
  handleBalance: () => Promise<void>
}

export const Address: React.FC<AddressProps> = ({
  balance,
  symbol,
  address,
  network,
  handleTransfer,
  handleBalance
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [loading, setLoading] = useState<boolean>(false)

  console.log({ symbol, network, address, balance })

  return (
    <div
      style={{
        border: "1px solid black",
        padding: "10px",
        margin: "5px"
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          paddingBottom: "5px",
          borderBottom: "0.5px solid black"
        }}
      >
        <strong>{symbol}</strong>
        <strong>{network}</strong>
        <button
          style={{
            ...buttonStyle,
            backgroundColor: "green"
          }}
          onClick={handleBalance}
        >
          ↻
        </button>
      </div>
      <p>
        <strong>Address: </strong>
        {address}
      </p>
      <p>
        <strong>Balance: </strong>
        {balance.toString()}
      </p>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between"
        }}
      >
        <input
          id="to"
          alt="To"
          type="text"
          placeholder="To"
          style={{
            flex: 5
          }}
          value={to}
          onChange={e => setTo(e.target.value)}
        />
        <input
          id="amount"
          alt="Amount"
          placeholder="Amount"
          style={{
            flex: 5
          }}
          type="text"
          value={amount}
          onChange={e => setAmount(e.target.value)}
        />
        <button
          style={{
            flex: 2
          }}
          onClick={() => {
            setLoading(true)
            handleTransfer(address, to, amount)
              .then(() => {
                setLoading(false)
                setTo("")
                setAmount("")
              })
              .catch(() => setLoading(false))
          }}
        >
          {loading ? `Sending ${symbol}...` : `Send ${symbol}`}
        </button>
      </div>
    </div>
  )
}
