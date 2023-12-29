import { useCallback, useState } from "react"
import { Button } from "components/ui/button"
import FieldRoute from "./FieldRoute"
import { useForm } from "react-hook-form"
import { SystemDynamicField, useSystemQuery } from "service/system"
import { Form } from "components/ui/form"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter
} from "components/ui/card"
import { GlobeIcon, ResetIcon } from "@radix-ui/react-icons"

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
    <Card
      marginBottom="md"
      title={functionName.toTitleCase()}
      icon={<GlobeIcon />}
      action={
        <Button
          onClick={() => {
            setArgState(null)
            setArgErrorState(null)
            methods.reset()
          }}
          noShadow
          asIconButton
          color="secondary"
          variant="filled"
          roundSide="tr"
          className="rounded-bl-lg"
        >
          <ResetIcon />
        </Button>
      }
    >
      <Form {...methods}>
        <form noValidate onSubmit={methods.handleSubmit(onSubmit)}>
          <CardContent>
            {fields?.map((field, index) => (
              <FieldRoute
                key={index}
                methodField={field}
                registerName={`${functionName}-arg${index}`}
                errors={methods.formState.errors[`${functionName}-arg${index}`]}
              />
            ))}
            <CardDescription className="flex flex-col mt-2 space-y-2 overflow-auto">
              {argState && (
                <span>
                  (
                  {argState
                    .map((arg: any) => JSON.stringify(arg, null, 2))
                    .join(", ")}
                  )
                </span>
              )}
              {argErrorState && (
                <span>
                  <strong>Arguments Error</strong>
                  {argErrorState}
                </span>
              )}
              {error && (
                <span>
                  <strong>Error</strong>
                  {error.message}
                </span>
              )}
              {loading && (
                <span>
                  <strong>Loading</strong>
                  Calling...
                </span>
              )}
              {data && (
                <span>
                  <strong>Results</strong>
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
              )}
            </CardDescription>
          </CardContent>
          <CardFooter>
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
          </CardFooter>
        </form>
      </Form>
    </Card>
  )
}

export default MethodForm
