import { Account } from "declarations/b3_user/b3_user.did"
import { BigNumber, ethers, providers } from "ethers"
import { useCallback, useEffect, useState } from "react"
import { B3User } from "service/actor"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface EthAccountProps extends Account {
  actor?: B3User
}

const EthAccount: React.FC<EthAccountProps> = ({
  actor,
  name,
  keys,
  id,
  ecdsa
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [waiting, setWaiting] = useState("Send")
  const [balance, setBalance] = useState<BigNumber>(BigNumber.from(0))

  const handleSignTx = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()

    setWaiting("Loading...")

    if (!actor) {
      return
    }

    const nonce = await provider.getTransactionCount(keys.address)
    const gasPrice = await provider.getGasPrice().then(s => s.toHexString())
    const value = ethers.utils.parseEther(amount).toHexString()
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

      setWaiting("Signing...")

      console.log({ title: "Signing transaction...", variant: "subtle" })

      const res = (await actor.sign_transaction(id, [...serializeTx], 97n)) as
        | { Err: string }
        | { Ok: { data: string } }

      if ("Err" in res) {
        const message = res.Err ?? ""
        console.log({
          title: "Error",
          description: message,
          status: "error",
          variant: "subtle"
        })
        setWaiting("Error")

        setTimeout(() => {
          setWaiting("Send")
        }, 2000)

        return
      }
      const signedTx = Buffer.from(res.Ok.data, "hex")

      console.log({ title: "Sending transaction...", variant: "subtle" })

      setWaiting("Sending...")

      const { hash } = await provider.sendTransaction(
        "0x" + signedTx.toString("hex")
      )

      await provider.waitForTransaction(hash)

      console.log({ title: "Transaction sent", variant: "subtle" })

      setWaiting("Sent!")

      setTimeout(() => {
        setWaiting("Send")
      }, 2000)
    } catch (error) {
      console.log(error)

      setWaiting("Error")

      setTimeout(() => {
        setWaiting("Send")
      }, 2000)
    }
  }
  const getBalance = useCallback(async () => {
    const balance = await provider.getBalance(keys.address)
    setBalance(balance)
  }, [keys.address])

  useEffect(() => {
    getBalance()
  }, [getBalance])

  return (
    <div
      style={{
        border: "1px solid black",
        padding: "10px",
        margin: "10px"
      }}
    >
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
      {balance.toString()}
      <br />
      <label>Send ETH: &nbsp;</label>
      <div
        style={{
          display: "flex",
          alignItems: "center"
        }}
      >
        <input
          id="to"
          alt="To"
          type="text"
          placeholder="To"
          value={to}
          onChange={e => setTo(e.target.value)}
        />
        <input
          id="amount"
          alt="Amount"
          placeholder="Amount"
          type="text"
          value={amount}
          onChange={e => setAmount(e.target.value)}
        />
        <button onClick={handleSignTx}>{waiting}</button>
      </div>
    </div>
  )
}

export default EthAccount
