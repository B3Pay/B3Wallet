import { useSystemQuery, useSystemUpdate } from "@src/service/system"

import App from "./App"

interface AppsProps {}

const Apps: React.FC<AppsProps> = ({}) => {
  const { data, error, loading } = useSystemQuery({
    functionName: "get_apps"
  })

  return (
    <div className="grid grid-cols-1 gap-4">
      {loading && <div>Loading...</div>}
      {error && <div>{error.message}</div>}
      {data && data?.map(appView => <App key={appView.app_id} {...appView} />)}
    </div>
  )
}

export default Apps
