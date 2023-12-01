import { Principal } from "@dfinity/principal"
import { PlusCircledIcon } from "@radix-ui/react-icons"
import { useState } from "react"
import { useSystemMethod } from "service/system"
import DisplayData from "./DisplayData"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface AddWalletProps {
  canisterId?: string
}

const AddWallet: React.FC<AddWalletProps> = ({ canisterId }) => {
  const { call, data, error, loading } = useSystemMethod("add_wallet_canister")

  const [input, setInput] = useState(canisterId || "")

  const addWalletHandler = async () => {
    if (!input) return

    const principal = Principal.fromText(input)

    const res = await call(principal)
    console.log(res)
  }

  return (
    <div>
      <div className="flex items-center">
        <Input
          icon={<PlusCircledIcon className="ml-[4px]" />}
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Canister ID"
          round="left"
          iconSize="sm"
        />
        <Button
          round="right"
          variant="outline"
          color="secondary"
          onClick={addWalletHandler}
        >
          Add Wallet
        </Button>
      </div>
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default AddWallet