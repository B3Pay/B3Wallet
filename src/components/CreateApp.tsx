import { CreateAppArgs, Value } from "declarations/b3_system/b3_system.did"
import { useState } from "react"
import { useSystemActorStore, useSystemUpdate } from "service/system"
import DisplayData from "./DisplayData"
import FormComponent from "./RenderCandid"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const [appName, setAppName] = useState("b3-wallet")
  const [metadata, setMetadata] = useState<Array<[string, Value]>>([])
  const [description, setDescription] = useState("")

  const { methodState } = useSystemActorStore()

  const { call, data, error, loading } = useSystemUpdate({
    functionName: "create_app"
  })

  const callCreateApp = () => {
    const createAppArgs: CreateAppArgs = {
      name: appName,
      metadata,
      description
    }

    call([createAppArgs])
  }

  console.log(methodState["update_app"].types.argTypes)

  return (
    <div>
      <h2>Create App</h2>
      <FormComponent
        argTypes={methodState["create_app"].types.argTypes}
        buttonName="Create App"
        clickHandler={callCreateApp}
        loading={loading}
      />
      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default CreateApp
