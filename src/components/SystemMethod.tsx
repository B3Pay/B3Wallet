import { FormProvider, useForm } from "react-hook-form"
import { SystemDynamicField } from "service/system"
import FieldRoute from "components/candid/FieldRoute"
import { Button } from "./ui/button"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "./ui/card"

interface SystemMethodProps extends SystemDynamicField {
  onSubmit: (data: any) => void
}

const SystemMethod: React.FC<SystemMethodProps> = ({
  fields,
  functionName,
  defaultValues,
  onSubmit
}) => {
  const methods = useForm({
    shouldUseNativeValidation: true,
    mode: "onChange",
    defaultValues
  })

  return (
    <Card>
      <FormProvider {...methods}>
        <form onSubmit={methods.handleSubmit(onSubmit)} noValidate>
          <CardHeader>
            <CardTitle>{functionName.toTitleCase()}</CardTitle>
          </CardHeader>
          <CardContent>
            {fields?.map((field, index) => {
              return (
                <FieldRoute
                  key={field.label}
                  methodField={field}
                  registerName={`${functionName}-arg${index}`}
                  errors={
                    methods.formState.errors[`${functionName}-arg${index}`]
                  }
                />
              )
            })}
          </CardContent>
          <CardFooter>
            <Button type="submit">Submit</Button>
          </CardFooter>
        </form>
      </FormProvider>
    </Card>
  )
}

export default SystemMethod
