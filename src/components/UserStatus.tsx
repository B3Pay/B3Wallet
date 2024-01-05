import { ReloadIcon } from "@radix-ui/react-icons"
import { useEffect } from "react"
import { useSystemUpdate } from "@src/service/system"
import AddWallet from "./AddWallet"
import DisplayData from "./DisplayData"
import InstallWallet from "./InstallWallet"
import { Button } from "./ui/button"

interface UserStatusProps {}

const UserStatus: React.FC<UserStatusProps> = ({}) => {
  const { call, data, error, loading } = useSystemUpdate({
    functionName: "get_user_status"
  })

  useEffect(() => {
    call()
  }, [])

  return (
    <div>
      <h2>Wallet Status</h2>
      <Button onClick={() => call()} asIconButton isLoading={loading}>
        <ReloadIcon />
      </Button>
      <DisplayData loading={loading} error={error} data={data} />
      <InstallWallet />
      <AddWallet />
    </div>
  )
}

export default UserStatus
