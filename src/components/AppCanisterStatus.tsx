import { Principal } from "@dfinity/principal"
import { ShadowInnerIcon } from "@radix-ui/react-icons"
import { useMemo, useState } from "react"
import { useSystemUpdate } from "service/system"
import DisplayData from "./DisplayData"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface AppCanisterStatusProps {
  canisterId?: string
}

const AppCanisterStatus: React.FC<AppCanisterStatusProps> = ({
  canisterId
}) => {
  const { call, data, error, loading } = useSystemUpdate({
    functionName: "get_user_app_status"
  })
  const [input, setInput] = useState(canisterId || "")

  const principal = useMemo(() => {
    if (!input) return null
    return Principal.fromText(input)
  }, [input])

  const clickHandler = () => {
    if (!principal) return
    call([principal])
  }

  return (
    <div>
      <div className="flex items-center">
        <Input
          icon={<ShadowInnerIcon className="ml-[4px]" />}
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Canister ID"
          round="left"
          iconSize="sm"
        />
        <Button
          isLoading={loading}
          disabled={loading}
          round="right"
          variant="outline"
          color="secondary"
          onClick={clickHandler}
        >
          Wallet Status
        </Button>
      </div>
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default AppCanisterStatus
