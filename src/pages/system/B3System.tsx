import {
  useSystemMethodFields,
  SystemDynamicField,
  useSystemQuery
} from "@src/service/system"
import MethodForm from "@src/components/candid/MethodForm"
import { Box } from "@src/components/ui/box"
import DisplayData from "@src/components/DisplayData"
import { useState } from "react"

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

const CandidField: React.FC<SystemDynamicField> = ({
  functionName,
  ...fields
}) => {
  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading } = useSystemQuery({
    functionName,
    disableInitialCall: true
  })

  return (
    <div className="bg-line-middle">
      <MethodForm
        {...fields}
        expanded={expanded}
        actorCallHandler={call}
        functionName={functionName}
        onExpand={() => setExpanded(prev => !prev)}
      />
      {expanded && <DisplayData loading={loading} error={error} data={data} />}
    </div>
  )
}

export default Candid
