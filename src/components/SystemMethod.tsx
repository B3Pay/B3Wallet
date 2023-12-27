import { FormProvider, useForm } from "react-hook-form"
import { SystemDynamicField } from "service/system"
import FieldSwitch from "components/candid/FieldSwitch"
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
    progressive: false,
    shouldUseNativeValidation: true,
    reValidateMode: "onChange",
    mode: "onChange",
    defaultValues
  })

  return (
    <Card>
      <FormProvider {...methods}>
        <form onSubmit={methods.handleSubmit(onSubmit)}>
          <CardHeader>
            <CardTitle>{functionName}</CardTitle>
          </CardHeader>
          <CardContent>
            {fields?.map((field, index) => {
              return (
                <FieldSwitch
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
