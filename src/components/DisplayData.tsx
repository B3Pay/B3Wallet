import { errorHandler, objectToString } from "lib/utils"

interface DisplayDataProps {
  loading: boolean
  error: unknown
  data: any
}

const DisplayData: React.FC<DisplayDataProps> = ({ loading, error, data }) => {
  return (
    <>
      <label>Response: &nbsp;</label>
      {loading ? <span>Loading...</span> : null}
      {error ? <span>Error: {errorHandler(error as Error)}</span> : null}
      {data && <p className="break-all">{objectToString(data)}</p>}
    </>
  )
}

export default DisplayData
