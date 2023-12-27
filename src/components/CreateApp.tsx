import { useSystemUpdate } from "service/system"
import DisplayData from "./DisplayData"
import SystemMethod from "./SystemMethod"
import { CreateAppArgs } from "declarations/b3_system/b3_system.did"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const { call, data, error, loading, field } = useSystemUpdate({
    functionName: "create_app"
  })

  console.log("field", field)

  const onSubmit = (args: any) => {
    console.log("args", args)
    const create_app_args = Object.values(args) as [CreateAppArgs]

    call(create_app_args)
  }

  return (
    <div>
      <h2>Create App</h2>
      {field ? <SystemMethod onSubmit={onSubmit} {...field} /> : null}
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default CreateApp
