import { useSystemUpdate } from "service/system"
import DisplayData from "./DisplayData"
import MethodForm from "./candid/MethodForm"
import { GlobeIcon } from "@radix-ui/react-icons"
import { Card, CardContent } from "./ui/card"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const { call, data, error, loading, field } = useSystemUpdate({
    functionName: "create_app"
  })

  return (
    <div>
      <h2>Create App</h2>
      {field ? (
        <MethodForm expanded actorCallHandler={call} {...field} />
      ) : null}
      {error || data || loading ? (
        <Card
          marginTop="sm"
          icon={<GlobeIcon />}
          iconProps={{
            color: loading ? "warning" : error ? "error" : "success",
            roundSide: "tl",
            diagonalRoundSide: "l"
          }}
          title={`Create App ${
            loading ? "Loading..." : error ? "Error!" : "Success"
          }`}
        >
          <CardContent>
            <DisplayData loading={loading} error={error} data={data} />
          </CardContent>
        </Card>
      ) : null}{" "}
    </div>
  )
}

export default CreateApp
