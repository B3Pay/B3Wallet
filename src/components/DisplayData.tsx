import { errorHandler, objectToString } from "lib/utils"
import { Card, CardContent } from "./ui/card"
import { GlobeIcon } from "@radix-ui/react-icons"

interface DisplayDataProps {
  loading: boolean
  error: unknown
  data: any
}

const DisplayData: React.FC<DisplayDataProps> = ({ loading, error, data }) => {
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
      title={loading ? "Loading..." : error ? "Error!" : "Success"}
    >
      <CardContent>
        <label>Response: &nbsp;</label>
        {loading ? <span>Loading...</span> : null}
        {error ? <span>Error: {errorHandler(error as Error)}</span> : null}
        {data ? (
          <p className="break-all">{objectToString(data)}</p>
        ) : typeof data === "bigint" ? (
          <p className="break-all">{data.toString()}</p>
        ) : null}
      </CardContent>
    </Card>
  )
}

export default DisplayData
