import { useCallback, useState } from "react"
import { Button } from "@src/components/ui/button"
import { CandidRoute } from "./route"
import { FormProvider, useForm } from "react-hook-form"
import { SystemDynamicField } from "@src/service/system"
import { WalletDynamicField } from "@src/service/wallet"
import { Form } from "@src/components/ui/form"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter
} from "@src/components/ui/card"
import {
  DownloadIcon,
  EyeClosedIcon,
  EyeOpenIcon,
  ResetIcon,
  UploadIcon
} from "@radix-ui/react-icons"
import { ExtractedFunction } from "@ic-reactor/store"
import { toast } from "sonner"

type CandidFormProps = (
  | SystemDynamicField
  | WalletDynamicField
  | ExtractedFunction<any>
) & {
  expanded?: boolean
  onExpand?: () => void
  actorCallHandler: (data: [any]) => Promise<any>
}

const CandidForm: React.FC<CandidFormProps> = ({
  functionName,
  defaultValues,
  type,
  expanded = false,
  onExpand,
  fields,
  actorCallHandler
}) => {
  const [argState, setArgState] = useState<any>(null)
  const [argErrorState, setArgErrorState] = useState<any>(null)

  const methods = useForm({
    mode: "onChange",
    defaultValues
  })

  const onVerifyArgs = useCallback(
    (data: any) => {
      setArgState(null)
      setArgErrorState(null)
      const args = (Object.values(data?.data) || []) as any[]
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
      const args = (Object.values(data.data) || []) as [any]
      setArgState(args)

      try {
        if (type === "query") {
          return await actorCallHandler(args)
        }
        return toast.promise(actorCallHandler(args), {
          loading: `Calling ${functionName.toTitleCase()}...`,
          success: data => {
            if ("Ok" in data) return JSON.stringify(data.Ok, null, 2)
            return JSON.stringify(data, null, 2)
          },
          error: "Error"
        })
      } catch (error) {
        console.log("error", error)
      }
    },
    [type, actorCallHandler]
  )

  const resetHandler = useCallback(() => {
    methods.reset(defaultValues)
    setArgState(null)
    setArgErrorState(null)
  }, [defaultValues, methods])

  const expandable = onExpand !== undefined

  return (
    <FormProvider {...methods}>
      <Card
        title={functionName.toTitleCase()}
        icon={type === "query" ? <DownloadIcon /> : <UploadIcon />}
        iconProps={{
          color: type === "query" ? "success" : "warning",
          roundSide: expanded ? "tl" : "l",
          diagonalRoundSide: expanded ? "l" : "none"
        }}
        action={
          <div>
            <Button
              onClick={resetHandler}
              asIconButton
              diagonalRoundSide={expandable ? "none" : "r"}
              variant="filled"
              roundSide={expandable && !expanded ? "none" : "bl"}
              innerShadow={expanded}
              color="secondary"
            >
              <ResetIcon />
            </Button>
            {expandable && (
              <Button
                onClick={onExpand}
                asIconButton
                color="info"
                variant="filled"
                roundSide={expanded ? "tr" : "r"}
                innerShadow={expanded}
              >
                {expanded ? <EyeOpenIcon /> : <EyeClosedIcon />}
              </Button>
            )}
          </div>
        }
      >
        {expanded && (
          <Form {...methods}>
            <form noValidate onSubmit={methods.handleSubmit(callHandler)}>
              <CardContent>
                {fields.map((field, index) => (
                  <CandidRoute
                    key={index}
                    extractedField={field}
                    registerName={`data.${functionName}-arg${index}`}
                    errors={
                      (methods.formState.errors?.data as Record<string, any>)?.[
                        `${functionName}-arg${index}`
                      ]
                    }
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
                </CardDescription>
              </CardContent>
              <CardFooter>
                <Button
                  color="secondary"
                  roundSide="l"
                  onClick={methods.handleSubmit(onVerifyArgs)}
                  fullWidth
                >
                  Verify Args
                </Button>
                <Button type="submit" color="primary" roundSide="r" fullWidth>
                  Call
                </Button>
              </CardFooter>
            </form>
          </Form>
        )}
      </Card>
    </FormProvider>
  )
}

export { CandidForm }
