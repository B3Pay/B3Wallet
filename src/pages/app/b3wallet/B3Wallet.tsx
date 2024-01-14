import { WalletDynamicField, useWalletQuery } from "@src/service/wallet"
import MethodForm from "@src/components/candid/old/MethodForm"
import DisplayData from "@src/components/DisplayData"
import { useState } from "react"

interface B3WalletProps extends WalletDynamicField {}

const B3Wallet: React.FC<B3WalletProps> = ({ functionName, ...fields }) => {
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

export default B3Wallet
