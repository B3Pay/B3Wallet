import { errorHandler, objectToString } from "lib/utils"

interface DisplayDataProps {
  loading: boolean
  error: unknown
  data: any
}

const DisplayData: React.FC<DisplayDataProps> = ({ loading, error, data }) => {
  return (
    <section>
      <label>Response: &nbsp;</label>
      {loading ? <span>Loading...</span> : null}
      {error ? <span>Error: {errorHandler(error as Error)}</span> : null}
      {data && <span>{objectToString(data)}</span>}
    </section>
  )
}

export default DisplayData
