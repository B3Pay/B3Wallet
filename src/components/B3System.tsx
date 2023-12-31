import {
  useSystemMethodFields,
  SystemDynamicField,
  useSystemQuery
} from "service/system"
import MethodForm from "./candid/MethodForm"

const Candid: React.FC = () => {
  const methodFields = useSystemMethodFields()

  return methodFields.map((field, index) => (
    <CandidField {...field} key={index} />
  ))
}

interface CandidProps extends SystemDynamicField {}

const CandidField: React.FC<CandidProps> = ({
  functionName,
  fields,
  defaultValues
}) => {
  const { call, data, error, loading } = useSystemQuery({
    functionName,
    disableInitialCall: true
  })

  return (
    <div>
      <MethodForm
        functionName={functionName}
        fields={fields}
        defaultValues={defaultValues}
        actorCallHandler={call}
      />
      {error && (
        <span>
          <strong>Error</strong>
          {error.message}
        </span>
      )}
      {loading && (
        <span>
          <strong>Loading</strong>
          Calling...
        </span>
      )}
      {data ? (
        <span>
          <strong>Results</strong>
          {!data ? (
            <div>Calling...</div>
          ) : (
            JSON.stringify(
              data,
              (_, value) =>
                typeof value === "bigint" ? value.toString() : value,
              2
            )
          )}
        </span>
      ) : null}
    </div>
  )
}

export default Candid
