import { useSystemQuery, useSystemUpdate } from "@src/service/system"
import { Button } from "@src/components/ui/button"

interface CreateAppProps {}

const CreateApp: React.FC<CreateAppProps> = ({}) => {
  const { call, data, error, loading } = useSystemQuery({
    functionName: "get_apps"
  })

  const { call: createApp } = useSystemUpdate({
    functionName: "create_app_canister"
  })

  return (
    <div>
      {data && (
        <div>
          <h3>Apps</h3>
          <ul>
            {data.map(({ app_id, metadata, name }) => (
              <li key={app_id}>
                <a href={`/app/${app_id}`}>{name}</a>
                {metadata.map(([key, value]) => (
                  <div key={key}>
                    {key}: {Object.values(value).toString()}
                  </div>
                ))}
                <Button onClick={() => createApp([app_id])}>Create</Button>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  )
}

export default CreateApp
