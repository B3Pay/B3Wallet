import { Account } from "declarations/b3_wallet/b3_wallet.did"
import { BigNumber, ethers, providers } from "ethers"
import { isAddress } from "ethers/lib/utils"
import { useCallback, useEffect, useState } from "react"
import { B3User } from "service/actor"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface EthAccountProps extends Account {
  actor: B3User
}

const buttonStyle = {
  color: "white",
  border: "none",
  borderRadius: "5px",
  padding: "5px",
  width: "30px",
  height: "30px",
  cursor: "pointer"
}

const EthAccount: React.FC<EthAccountProps> = ({
  actor,
  id,
  name,
  ledger: { public_keys }
}) => {
  const [to, setTo] = useState<string>("")
  const [amount, setAmount] = useState<string>("")
  const [waiting, setWaiting] = useState("Send")
  const [ethBalance, setEthBalance] = useState<BigNumber>(BigNumber.from(0))
  const [icpBalance, setIcpBalance] = useState<BigNumber>(BigNumber.from(0))
  const [newName, setNewName] = useState<string>(name)
  const [editMode, setEditMode] = useState<boolean>(false)

  const handleSignTx = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()

    setWaiting("Loading...")

    if (!actor) {
      return
    }
    const eth_address = public_keys.addresses[0][0]

    if (isAddress(eth_address) === false) {
      setWaiting("Error")
      setTimeout(() => {
        setWaiting("Send")
      }, 2000)
      return
    }

    const nonce = await provider.getTransactionCount(eth_address)
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

      console.log(serializeTx)

      setWaiting("Signing...")

      console.log({ title: "Signing transaction...", variant: "subtle" })

      const res = await actor.request_sign_transaction(
        id,
        [...serializeTx],
        97n
      )

      console.log(res)
    } catch (error) {
      console.log(error)

      setWaiting("Error")

      setTimeout(() => {
        setWaiting("Send")
      }, 2000)
    }
  }
  const getEthBalance = useCallback(async () => {
    const eth_address = public_keys.addresses[0][0]

    if (isAddress(eth_address) === false) {
      return
    }

    const balance = await provider.getBalance(eth_address)
    setEthBalance(balance)
  }, [public_keys.addresses])

  const getIcpBalance = useCallback(async () => {
    if (!actor) {
      return
    }

    const balance = await actor.request_balance(id)

    const balanceBigNumber = BigNumber.from(balance.e8s)

    setIcpBalance(balanceBigNumber)
  }, [actor, id])

  useEffect(() => {
    getEthBalance()
    getIcpBalance()
  }, [getEthBalance, getIcpBalance])

  const requestPublicKey = async () => {
    if (!actor) {
      return
    }

    await actor.request_public_key(id)
  }

  const removeAccount = async () => {
    await actor.remove_account(id)
  }

  const handleIcpTransfer = async () => {
    if (!actor) {
      return
    }

    const tokenAmount = {
      e8s: BigInt(amount)
    }

    setWaiting("Sending...")

    const res = await actor.send_icp(id, to, tokenAmount, [], [])

    console.log(res)

    setWaiting("Send")

    setTimeout(() => {
      getIcpBalance()
    }, 2000)
  }

  return (
    <div
      style={{
        border: "1px solid black",
        padding: "10px",
        margin: "10px"
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          paddingBottom: "10px",
          borderBottom: "1px dashed black"
        }}
      >
        <div>
          {editMode ? (
            <input
              type="text"
              value={newName}
              onChange={e => setNewName(e.target.value)}
            />
          ) : (
            <span style={{ fontWeight: "bold" }}>{newName}</span>
          )}
          <button
            style={{
              ...buttonStyle,
              backgroundColor: editMode ? "green" : "blue"
            }}
            onClick={async () => {
              if (editMode) {
                const currentName = await actor.rename_account(id, newName)
                setNewName(currentName)
                setEditMode(false)
              } else setEditMode(true)
            }}
          >
            {editMode ? "✔" : "✎"}
          </button>
          {editMode ? (
            <button
              onClick={() => {
                setNewName(name)
                setEditMode(false)
              }}
              style={{ ...buttonStyle, backgroundColor: "red" }}
            >
              X
            </button>
          ) : null}
        </div>
        <button
          onClick={removeAccount}
          style={{ ...buttonStyle, backgroundColor: "red" }}
        >
          X
        </button>
      </div>
      <br />
      {public_keys.ecdsa.length ? (
        <div>
          <label>ECDSA Public Keys: &nbsp;</label>
          {JSON.stringify(public_keys.ecdsa)}
        </div>
      ) : (
        <div>
          <label>Request Public Keys: &nbsp;</label>
          <button onClick={requestPublicKey}>Request</button>
        </div>
      )}
      <br />
      <label>Addresses: &nbsp;</label>
      {/* generate address using select input */}
      <select>
        {["SNS", "BTC", "ETH"].map((networkInterface, index) => (
          <option key={index} value={networkInterface}>
            {networkInterface}
          </option>
        ))}
      </select>
      <button
        onClick={() => {
          // generate address
          actor.generate_address(id, {
            EVM: 0n
          })
        }}
      >
        Generate Eth Address
      </button>

      <br />
      <ul>
        {public_keys.addresses.map(([key, value]) => (
          <li key={key}>
            <label>{key}: &nbsp;</label>
            {value}
          </li>
        ))}
      </ul>
      <label>Id: &nbsp;</label>
      {id}
      <br />
      <label>Balance Eth: &nbsp;</label>
      {ethBalance.toString()}
      <br />
      <label>Balance ICP: &nbsp;</label>
      {icpBalance.toString()}
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
      <label>Send ICP: &nbsp;</label>
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
        <button onClick={handleIcpTransfer}>{waiting}</button>
      </div>
    </div>
  )
}

export default EthAccount
