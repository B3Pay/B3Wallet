import { useCallback, useState } from "react"
import { Button } from "@src/components/ui/button"
import FieldRoute from "./FieldRoute"
import { useForm } from "react-hook-form"
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
import { ActorMethodField } from "@ic-reactor/store"

type MethodFormProps = (
  | SystemDynamicField
  | WalletDynamicField
  | ActorMethodField<any>
) & {
  expanded?: boolean
  onExpand?: () => void
  actorCallHandler: (data: [any]) => Promise<any>
}

const MethodForm: React.FC<MethodFormProps> = ({
  functionName,
  label,
  defaultValues,
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

  const onSubmit = useCallback(
    (data: any) => {
      console.log("data", data)
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
      console.log("data", data)
      setArgState(null)
      setArgErrorState(null)
      const args = (Object.values(data.data) || []) as [any]
      console.log("args", args)
      setArgState(args)

      try {
        const result = await actorCallHandler(args)
        console.log("result", result)
      } catch (error) {
        console.log("error", error)
      }
    },
    [actorCallHandler]
  )

  const expandable = onExpand !== undefined

  return (
    <Card
      title={functionName.toTitleCase()}
      icon={label === "query" ? <DownloadIcon /> : <UploadIcon />}
      iconProps={{
        color: label === "query" ? "success" : "warning",
        roundSide: expanded ? "tl" : "l",
        diagonalRoundSide: expanded ? "l" : "none"
      }}
      action={
        <div>
          <Button
            onClick={() => {
              setArgState(null)
              setArgErrorState(null)
              methods.reset()
            }}
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
          <form noValidate onSubmit={methods.handleSubmit(onSubmit)}>
            <CardContent>
              {fields?.map((field, index) => (
                <FieldRoute
                  key={index}
                  methodField={field}
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
      )}
    </Card>
  )
}

export default MethodForm
