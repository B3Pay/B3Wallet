import { useSystemMethodFields } from "service/system"
import MethodForm from "./candid/MethodForm"

interface CandidProps {}

const Candid: React.FC<CandidProps> = () => {
  const methodFields = useSystemMethodFields()

  return (
    <div className="p-2 max-w-3xl mx-auto">
      {methodFields.map((field, index) => (
        <MethodForm {...field} key={index} />
      ))}
    </div>
  )
}

export default Candid
