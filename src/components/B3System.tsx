import {
  useSystemMethodFields,
  SystemDynamicField,
  useSystemQuery
} from "service/system"
import MethodForm from "./candid/MethodForm"
import { Card, CardContent } from "./ui/card"
import { Box } from "./ui/box"
import DisplayData from "./DisplayData"
import { GlobeIcon } from "@radix-ui/react-icons"

const Candid: React.FC = () => {
  const methodFields = useSystemMethodFields()

  return (
    <Box className="grid gap-2">
      {methodFields.map((field, index) => (
        <CandidField {...field} key={index} />
      ))}
    </Box>
  )
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
    <div className="bg-line-middle">
      <MethodForm
        functionName={functionName}
        fields={fields}
        defaultValues={defaultValues}
        actorCallHandler={call}
      />
      {error || data || loading ? (
        <Card
          marginTop="sm"
          icon={<GlobeIcon />}
          iconProps={{
            color: loading ? "warning" : error ? "error" : "success"
          }}
          title={`${functionName.toTitleCase()} ${
            loading ? "Loading..." : error ? "Error!" : "Success"
          }`}
        >
          <CardContent>
            <DisplayData loading={loading} error={error} data={data} />
          </CardContent>
        </Card>
      ) : null}
    </div>
  )
}

export default Candid
