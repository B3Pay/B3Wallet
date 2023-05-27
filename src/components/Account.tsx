/* eslint-disable no-unused-vars */
import { WalletAccount } from "declarations/b3_wallet/b3_wallet.did"
import { ethers, providers } from "ethers"
import { isAddress } from "ethers/lib/utils"
import { useCallback, useEffect, useState } from "react"
import { B3User } from "service/actor"
import { Address } from "./Address"
import ChainsSelect from "./ChainSelect"

const provider = new providers.JsonRpcProvider(
  "https://data-seed-prebsc-2-s1.binance.org:8545"
)

interface AccountProps extends WalletAccount {
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

interface Balances {
  EVM: BigInt
  BTC: BigInt
  ICP: BigInt
}

const Account: React.FC<AccountProps> = ({
  actor,
  id,
  name,
  ledger: { keys }
}) => {
  const [loading, setLoading] = useState(false)
  const [balances, setBalances] = useState<Balances>({
    EVM: 0n,
    BTC: 0n,
    ICP: 0n
  })
  const [newName, setNewName] = useState<string>(name)
  const [editMode, setEditMode] = useState<boolean>(false)

  const handleEthTransfer = async (
    from: string,
    to: string,
    amount: string
  ) => {
    console.log(`Transfering ${amount} ETH from ${from} to ${to}`)
    setLoading(true)

    const nonce = await provider.getTransactionCount(from)
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

      setLoading(true)

      console.log({ title: "Signing transaction...", variant: "subtle" })

      const res = await actor.request_sign_transaction(
        id,
        [...serializeTx],
        97n
      )

      console.log(res)
    } catch (error) {
      console.log(error)

      setLoading(false)
    }
  }

  const handleBtcTransfer = async (
    from: string,
    to: string,
    amount: string
  ) => {
    console.log(`Transfering ${amount} BTC from ${from} to ${to}`)
    setLoading(true)

    const res = await actor.request_transfer_btc(
      id,
      { Regtest: null },
      to,
      BigInt(amount)
    )

    console.log(res)

    setLoading(false)

    setTimeout(() => {
      getIcpBalance()
    }, 2000)
  }

  const handleIcpTransfer = async (
    from: string,
    to: string,
    amount: string
  ) => {
    console.log(`Transfering ${amount} ICP from ${from} to ${to}`)
    const tokenAmount = {
      e8s: BigInt(amount)
    }

    setLoading(true)

    const res = await actor.account_send_icp(id, to, tokenAmount, [], [])

    console.log(res)

    setLoading(false)
  }

  const getEthBalance = useCallback(async () => {
    const address = ""
    if (isAddress(address) === false) {
      return
    }

    const balance = await provider.getBalance(address)

    setBalances(prev => ({ ...prev, ETH: balance.toBigInt() }))
  }, [])

  const getBtcBalance = useCallback(async () => {
    console.log(keys.addresses)
    if (keys.addresses.length <= 1) {
      return
    }

    const balance = await actor.request_balance_btc(id, { Regtest: null }, [])

    console.log(balance)

    setBalances(prev => ({ ...prev, BTC: balance }))
  }, [actor, id, keys.addresses])

  const getIcpBalance = useCallback(async () => {
    const balance = await actor.account_icp_balance(id, [])

    console.log(balance)

    setBalances(prev => ({ ...prev, ICP: balance.e8s }))
  }, [actor, id])

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
    getEthBalance()
    getBtcBalance()
    getIcpBalance()
  }, [getEthBalance, getBtcBalance, getIcpBalance])

  const refresh = useCallback(async () => {
    await getEthBalance()
    await getIcpBalance()
    await getBtcBalance()
  }, [getEthBalance, getIcpBalance, getBtcBalance])

  const requestPublicKey = async () => {
    await actor.account_request_public_key(id)
  }

  const removeAccount = async () => {
    await actor.account_remove(id)
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
                await actor.account_rename(id, newName)
                setNewName(newName)
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
        <div>
          <button
            style={{
              ...buttonStyle,
              backgroundColor: "green"
            }}
            onClick={refresh}
          >
            ↻
          </button>
          <button
            onClick={removeAccount}
            style={{ ...buttonStyle, backgroundColor: "red" }}
          >
            X
          </button>
        </div>
      </div>
      <br />
      {keys.ecdsa.length ? (
        <div>
          <label>ECDSA Public Keys: &nbsp;</label>
          {keys.ecdsa.toString()}
        </div>
      ) : (
        <div>
          <label>Request Public Keys: &nbsp;</label>
          <button onClick={requestPublicKey}>Request</button>
        </div>
      )}
      <br />
      <label>Addresses: &nbsp;</label>
      <ChainsSelect account_id={id} actor={actor} />
      <br />
      <label>Id: &nbsp;</label>
      <span>{id}</span>
      <div>
        {keys.addresses.map((item, index) => {
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
            <Address
              key={index}
              actor={actor}
              symbol={symbol}
              address={item[1]}
              balance={balances[symbol]}
              network={network}
              handleTransfer={handleTransfer[symbol]}
              handleBalance={handleBalance[symbol]}
            />
          )
        })}
      </div>
    </div>
  )
}

export default Account
