import {
  useWalletMethodFields,
  WalletDynamicField,
  useWalletQuery
} from "service/wallet"
import MethodForm from "./candid/MethodForm"
import { Box } from "./ui/box"
import DisplayData from "./DisplayData"
import { useState } from "react"

const Candid: React.FC = () => {
  const methodFields = useWalletMethodFields()

  return (
    <Box className="grid gap-2">
      {methodFields.map((field, index) => (
        <CandidField {...field} key={index} />
      ))}
    </Box>
  )
}

interface CandidProps extends WalletDynamicField {}

const CandidField: React.FC<CandidProps> = ({ functionName, ...fields }) => {
  const [expanded, setExpanded] = useState(false)

  const { call, data, error, loading } = useWalletQuery({
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
