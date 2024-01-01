import {
  useWalletMethodFields,
  WalletDynamicField,
  useWalletQuery
} from "service/wallet"
import MethodForm from "./candid/MethodForm"
import { Card, CardContent } from "./ui/card"

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
    <div>
      <MethodForm
        functionName={functionName}
        fields={fields}
        defaultValues={defaultValues}
        actorCallHandler={call}
      />
      {error && (
        <Card title={functionName.toTitleCase()}>
          <CardContent>
            <strong>Error</strong>
            {error.message}
          </CardContent>
        </Card>
      )}
      {loading && (
        <Card title={functionName.toTitleCase()}>
          <CardContent>
            <strong>Loading</strong>
            Calling...
          </CardContent>
        </Card>
      )}
      {data ? (
        <Card title={functionName.toTitleCase()}>
          <CardContent>
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
          </CardContent>
        </Card>
      ) : null}
    </div>
  )
}

export default Candid
