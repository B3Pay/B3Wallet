import { useSystemQuery } from "@src/service/system"
import { UserStatus } from "@src/declarations/b3system/b3system.did"
import { Principal } from "@dfinity/principal"
import AppList from "./AppList"
import CreateAccount from "./CreateAccount"
import App from "./App"

interface AccountProps {}

const extractStatus = (status: UserStatus) => {
  return Object.entries(status).reduce(
    (acc, [status, value]) => {
      if (value) {
        return { status, value }
      } else {
        return { status, value: [] as Principal[] }
      }
    },
    { status: "", value: [] as Principal[] }
  )
}

const Account: React.FC<AccountProps> = () => {
  const { call, data } = useSystemQuery({
    functionName: "get_user_status",
    onError: error => {
      console.log("error", error)
    }
  })

  const { status, value } = extractStatus(data || ({} as UserStatus))

  switch (status.toString()) {
    case "Applications":
      return (
        <div className="grid grid-cols-1 gap-1">
          {value.map((principal, index) => (
            <App key={index} principal={principal} />
          ))}
        </div>
      )
    case "Registered":
      return <AppList refreshHandler={() => call()} />
    default:
      return <CreateAccount refreshHandler={() => call()} />
  }
}

export default Account
