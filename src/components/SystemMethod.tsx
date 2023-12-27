"use client"
import { FormProvider, useForm } from "react-hook-form"
import { SystemDynamicField } from "service/system"
import FormField from "components/candid/FormField"
import { Button } from "./ui/button"

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
    <FormProvider {...methods}>
      <form
        onSubmit={methods.handleSubmit(onSubmit)}
        className="border border-gray-500 rounded p-2 mt-2 w-full"
      >
        <h1 className="text-xl font-bold mb-4">{functionName}</h1>
        {fields?.map((field, index) => {
          return (
            <div key={index} className="mb-2">
              <FormField
                field={field}
                registerName={`${functionName}-arg${index}`}
                errors={methods.formState.errors[`${functionName}-arg${index}`]}
              />
            </div>
          )
        })}
        <Button type="submit">Submit</Button>
      </form>
    </FormProvider>
  )
}

export default SystemMethod
