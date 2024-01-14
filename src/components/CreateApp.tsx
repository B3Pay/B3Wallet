import { useSystemUpdate } from "@src/service/system"
import DisplayData from "./DisplayData"
import MethodForm from "./candid/old/MethodForm"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const { call, data, error, loading, field } = useSystemUpdate({
    functionName: "create_app"
  })
  console.log("field", field)
  return (
    <div>
      <h2>Create App</h2>
      {field ? (
        <MethodForm expanded actorCallHandler={call} {...field} />
      ) : null}
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default CreateApp
