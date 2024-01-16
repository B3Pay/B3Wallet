import { CandidRouteProps } from "../route"
import { FormField } from "@src/components/ui/form"

export interface NullProps extends CandidRouteProps<"null"> {}

const Null: React.FC<NullProps> = ({ registerName, shouldUnregister }) => {
  return (
    <FormField
      shouldUnregister={shouldUnregister}
      name={registerName as never}
      defaultValue={null as never}
      render={() => null as never}
    />
  )
}

export { Null }
