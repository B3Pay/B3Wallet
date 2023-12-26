import { Actor } from "@dfinity/agent"
import { useMemo } from "react"
import { useSystemActorStore } from "service/system"
import Form from "./candid/Form"

interface CandidProps {}

const Candid: React.FC<CandidProps> = () => {
  const { actor } = useSystemActorStore()

  const field = useMemo(() => {
    if (!actor) return { fields: [] }

    return Actor.interfaceOf(actor).extractField()
  }, [actor])

  return (
    <div className="p-2 max-w-3xl mx-auto">
      {field.fields.map(field => (
        <Form {...field} key={field.label} />
      ))}
    </div>
  )
}

export default Candid
