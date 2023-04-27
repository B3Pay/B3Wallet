import { Account } from "declarations/b3_user/b3_user.did"
import { JsonRpcProvider, Transaction, formatEther, parseEther } from "ethers"
import { useState } from "react"
import { B3User } from "service/actor"

const provider = new JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface EthAccountProps extends Account {
  actor?: B3User
}

export const EthAccount: React.FC<EthAccountProps> = ({
  actor,
  name,
  keys,
  id,
  ecdsa
}) => {
  const [to, setTo] = useState<string>("")
  const [loading, setLoading] = useState(false)
  const [balance, setBalance] = useState<string>("")

  const handleSignTx = async e => {
    e.preventDefault()

    if (actor === undefined) {
      console.log({
        title: "Error",
        description: "Actor is undefined",
        status: "error",
        variant: "subtle"
      })
      return
    }

    setLoading(true)

    const nonce = await provider.getTransactionCount(keys.address)
    const gasPrice = await provider.getFeeData().then(s => s.gasPrice)
    const value = parseEther("0.0001")
    const data = "0x00"
    const gasLimit = BigInt("24000")

    const tx = new Transaction()

    tx.nonce = nonce
    tx.gasPrice = gasPrice
    tx.gasLimit = gasLimit
    tx.to = to
    tx.value = value
    tx.data = data

    const serializeTx = Buffer.from(
      tx.unsignedSerialized.slice(2) + "808080",
      "hex"
    )

    console.log({ title: "Signing transaction...", variant: "subtle" })

    console.log(serializeTx)

    const res = await actor.sign_transaction(id, 97n, serializeTx)

    if ("Err" in res) {
      const message = res.Err ?? ""
      console.log({
        title: "Error",
        description: message,
        status: "error",
        variant: "subtle"
      })
      return
    }

    const signedTx = Buffer.from(res.Ok.data, "hex")

    console.log(signedTx)

    console.log({ title: "Sending transaction...", variant: "subtle" })

    const { hash } = await provider.broadcastTransaction(
      "0x" + signedTx.toString("hex")
    )

    await provider.waitForTransaction(hash)

    setLoading(false)

    const balance = await provider.getBalance(keys.address)
    setBalance(formatEther(balance))
  }

  return (
    <div>
      <label>Name: &nbsp;</label>
      {name}
      {Object.entries(ecdsa).map(([key, value]) => (
        <div key={key}>
          <label>{key}: &nbsp;</label>
          {JSON.stringify(value)}
        </div>
      ))}
      <label>Address: &nbsp;</label>
      {keys.address}
      <br />
      <label>Id: &nbsp;</label>
      {id}
      <br />
      <label>Balance: &nbsp;</label>
      {balance}
      <br />
      <label>Send ETH: &nbsp;</label>
      <input
        id="to"
        alt="To"
        type="text"
        value={to}
        onChange={e => setTo(e.target.value)}
      />
      <button onClick={handleSignTx}>Send</button>
    </div>
  )
}

export default EthAccount
