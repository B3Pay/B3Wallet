import { useSystemQuery } from "@src/service/system"
import { errorHandler, objectToString } from "@src/lib/utils"
import { Card, CardContent } from "./ui/card"
import { GlobeIcon, Link2Icon } from "@radix-ui/react-icons"
import { Principal } from "@dfinity/principal"
import { Button } from "./ui/button"
import { useRouter } from "next/router"

interface AppProps {
  principal: Principal
}

const App: React.FC<AppProps> = ({ principal }) => {
  const { push } = useRouter()

  const { call, data, error, loading } = useSystemQuery({
    functionName: "get_user_app_status",
    args: [principal]
  })

  if (!loading && !error && data === undefined) return null
  return (
    <Card
      marginTop="sm"
      icon={<GlobeIcon />}
      iconProps={{
        color: loading ? "warning" : error ? "error" : "success",
        roundSide: "tl",
        diagonalRoundSide: "l"
      }}
      title={loading ? "Loading..." : error ? "Error!" : principal.toText()}
      action={
        <Button
          asIconButton
          diagonalRoundSide="r"
          variant="filled"
          color="secondary"
          onClick={() =>
            push(
              `${
                window.location.origin
              }/candid?canisterId=${principal.toText()}`
            )
          }
        >
          <Link2Icon />
        </Button>
      }
    >
      <CardContent>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {errorHandler(error as Error)}</span> : null}
        {data &&
          Object.entries(data).map(([key, value]) => (
            <div key={key}>
              <label>{key.toTitleCase()}: &nbsp;</label>
              <span>{objectToString(value)}</span>
            </div>
          ))}
      </CardContent>
    </Card>
  )
}

export default App
