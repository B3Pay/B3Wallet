import {
  useWalletMethodFields,
  WalletDynamicField,
  useWalletQuery
} from "service/wallet"
import MethodForm from "./candid/MethodForm"

const Candid: React.FC = () => {
  const methodFields = useWalletMethodFields()

  return methodFields.map((field, index) => (
    <CandidField {...field} key={index} />
  ))
}

interface CandidProps extends WalletDynamicField {}

const CandidField: React.FC<CandidProps> = ({
  functionName,
  fields,
  defaultValues
}) => {
  const { call, data, error, loading } = useWalletQuery({
    functionName,
    disableInitialCall: true
  })

  return (
    <div className="p-2 max-w-3xl mx-auto">
      <MethodForm
        functionName={functionName}
        fields={fields}
        defaultValues={defaultValues}
        actorCallHandler={call}
      />{" "}
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
