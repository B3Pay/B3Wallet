import { useSystemQuery } from "@src/service/system"

import InstallApp from "./InstallApp"
import { Card } from "./ui/card"

interface AppListProps {
  refreshHandler?: () => void
}

const AppList: React.FC<AppListProps> = ({ refreshHandler }) => {
  const { call, data, error, loading } = useSystemQuery({
    functionName: "get_apps",
    refetchOnMount: true
  })

  return (
    <div className="grid grid-cols-1 gap-2">
      <Card
        title="B3Forge"
        titleProps={{
          className:
            "text-2xl font-bold px-4 py-2 flex-1 flex items-center justify-center"
        }}
      />
      {loading && <div>Loading...</div>}
      {error && <div>{error.message}</div>}
      {data &&
        data.map(appView => (
          <InstallApp
            key={appView.app_id}
            appRefresher={call}
            refreshHandler={refreshHandler}
            {...appView}
          />
        ))}
    </div>
  )
}

export default AppList
