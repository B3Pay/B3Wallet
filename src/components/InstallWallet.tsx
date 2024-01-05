import { Principal } from "@dfinity/principal"
import { ShadowInnerIcon } from "@radix-ui/react-icons"
import { useState } from "react"
import { useSystemUpdate } from "@src/service/system"
import DisplayData from "./DisplayData"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface InstallWalletProps {
  canisterId?: string
}

const InstallWallet: React.FC<InstallWalletProps> = ({ canisterId }) => {
  const { call, data, error, loading } = useSystemUpdate({
    functionName: "install_app"
  })

  const [input, setInput] = useState(canisterId || "")

  const installWalletHandler = async () => {
    if (!input) return

    const principal = Principal.fromText(input)

    const res = await call([principal, "b3wallet"])
    console.log(res)
  }

  return (
    <div>
      <div className="flex items-center">
        <Input
          icon={<ShadowInnerIcon />}
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Canister ID"
          roundSide="l"
        />
        <Button
          roundSide="r"
          variant="outline"
          color="secondary"
          onClick={installWalletHandler}
          isLoading={loading}
        >
          Install Wallet
        </Button>
      </div>
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default InstallWallet
