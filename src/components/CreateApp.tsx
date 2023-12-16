import { CreateAppArgs, Value } from "declarations/b3_system/b3_system.did"
import { useState } from "react"
import { useSystemUpdate } from "service/system"
import DisplayData from "./DisplayData"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const [appName, setAppName] = useState("b3-wallet")
  const [metadata, setMetadata] = useState<Array<[string, Value]>>([])
  const [description, setDescription] = useState("")

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

  return (
    <div>
      <h2>Create App</h2>
      <Input
        type="text"
        placeholder="App Name"
        value={appName}
        onChange={e => setAppName(e.target.value)}
      />
      <Input
        type="text"
        placeholder="Description"
        value={description}
        onChange={e => setDescription(e.target.value)}
      />
      <Input
        type="text"
        placeholder="Metadata"
        value={metadata}
        onChange={e => setMetadata(e.target.value)}
      />
      <Button isLoading={loading} onClick={callCreateApp}>
        Create App
      </Button>

      <DisplayData loading={loading} error={error} data={data} />
    </div>
  )
}

export default CreateApp
