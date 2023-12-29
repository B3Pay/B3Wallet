import { useCallback, useState } from "react"
import { Button } from "components/ui/button"
import FieldRoute from "./FieldRoute"
import { useForm } from "react-hook-form"
import { SystemDynamicField, useSystemQuery } from "service/system"
import { Form } from "components/ui/form"

interface MethodFormProps extends SystemDynamicField {}

const MethodForm: React.FC<MethodFormProps> = ({
  functionName,
  defaultValues,
  fields
}) => {
  const [argState, setArgState] = useState<any>(null)
  const [argErrorState, setArgErrorState] = useState<any>(null)

  const { data, loading, error, call } = useSystemQuery({
    functionName,
    disableInitialCall: true
  })

  const methods = useForm({
    progressive: false,
    shouldUseNativeValidation: true,
    reValidateMode: "onChange",
    mode: "onChange",
    defaultValues
  })

  const onSubmit = useCallback(
    (data: any) => {
      setArgState(null)
      setArgErrorState(null)
      const args = (Object.values(data) || []) as any[]
      console.log("args", args)

      let errorMessages = ""
      const isInvalid = args.some((arg, i) => {
        const validateArg = fields[i]?.validate(arg)
        if (typeof validateArg === "string") {
          errorMessages = validateArg
          return false
        }
        return true
      })

      if (isInvalid === true) {
        setArgState(args)
        return args
      } else {
        setArgErrorState(errorMessages)
      }
    },
    [fields]
  )

  const callHandler = useCallback(
    async (data: any) => {
      setArgState(null)
      setArgErrorState(null)
      const args = (Object.values(data) || []) as [any]
      console.log("args", args)
      setArgState(args)

      try {
        const result = await call(args)
        console.log("result", result)
      } catch (error) {
        console.log("error", error)
      }
    },
    [call]
  )

  return (
    <Form {...methods}>
      <form
        onSubmit={methods.handleSubmit(onSubmit)}
        className="border border-gray-500 rounded p-2 mt-2 w-full"
      >
        <h1 className="text-xl font-bold mb-4">{functionName}</h1>
        {fields?.map((field, index) => {
          return (
            <div key={index} className="mb-2">
              <FieldRoute
                methodField={field}
                registerName={`${functionName}-arg${index}`}
                errors={methods.formState.errors[`${functionName}-arg${index}`]}
              />
            </div>
          )
        })}
        {argState && (
          <fieldset className="border p-2 my-2 rounded">
            <legend className="font-semibold">Arguments</legend>
            <span className="text-sm">
              (
              {argState
                .map((arg: any) => JSON.stringify(arg, null, 2))
                .join(", ")}
              )
            </span>
          </fieldset>
        )}
        {argErrorState && (
          <fieldset className="border p-2 my-2 text-red-500 border-red-500 rounded">
            <legend className="font-semibold">Arguments Error</legend>
            <span className="text-sm">
              <div>{argErrorState}</div>
            </span>
          </fieldset>
        )}
        {error && (
          <fieldset className="border p-2 my-2 text-red-500 border-red-500 rounded">
            <legend className="font-semibold">Error</legend>
            <span className="text-sm">
              <div>{error.message}</div>
            </span>
          </fieldset>
        )}
        {loading && (
          <fieldset className="border p-2 my-2 rounded">
            <legend className="font-semibold">Loading</legend>
            <span className="text-sm">Calling...</span>
          </fieldset>
        )}
        {data && (
          <fieldset className="border p-2 my-2 rounded">
            <legend className="font-semibold">Results</legend>
            <span className="text-sm">
              {!data ? (
                <div>Calling...</div>
              ) : (
                JSON.stringify(
                  data,
                  (_, value) =>
                    typeof value === "bigint" ? value.toString() : value,
                  2
                )
              )}
            </span>
          </fieldset>
        )}
        <div className="flex items-center w-full">
          <Button type="submit" color="secondary" roundSide="l" fullWidth>
            Verify Args
          </Button>
          <Button
            color="primary"
            onClick={methods.handleSubmit(callHandler)}
            roundSide="r"
            fullWidth
          >
            Call
          </Button>
        </div>
      </form>
    </Form>
  )
}

export default MethodForm
