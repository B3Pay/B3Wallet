import { useSystemUpdate } from "@src/service/system"
import { Button } from "./ui/button"
import FieldRoute from "./candid/FieldRoute"
import { useForm } from "react-hook-form"
import { Form } from "./ui/form"
import { useCallback } from "react"
import { Card, CardContent, CardFooter } from "./ui/card"
import { PersonIcon, ReloadIcon } from "@radix-ui/react-icons"
import { toast } from "sonner"

interface CreateAccountProps {
  refreshHandler?: () => void
}

const CreateAccount: React.FC<CreateAccountProps> = ({ refreshHandler }) => {
  const { call, loading, field } = useSystemUpdate({
    functionName: "create_user",
    throwOnError: true,
    onError: error => {
      if (error?.message.includes("User already exists")) {
        toast.error("User already exists!")
      }
    }
  })

  const methods = useForm({
    mode: "onChange",
    defaultValues: field?.defaultValues
  })

  const onSubmit = useCallback(
    (data: any) => {
      const args = (Object.values(data?.data) || []) as [any]

      console.log("args", args)

      toast.promise(call(args), {
        loading: "Loading...",
        success: data => {
          return `Success: ${JSON.stringify(data)}`
        },
        error: "Error"
      })
    },
    [field]
  )

  return (
    <Card
      title="Create Account"
      icon={<PersonIcon />}
      action={
        <Button
          asIconButton
          diagonalRoundSide="r"
          variant="filled"
          color="secondary"
          onClick={refreshHandler}
          isLoading={loading}
        >
          <ReloadIcon />
        </Button>
      }
    >
      <Form {...methods}>
        <form noValidate onSubmit={methods.handleSubmit(onSubmit)}>
          <CardContent>
            {field?.fields?.map((field, index) => (
              <FieldRoute
                key={index}
                methodField={field}
                registerName={`data.create_user-arg${index}`}
                errors={
                  (methods.formState.errors?.data as Record<string, any>)?.[
                    `create_user-arg${index}`
                  ]
                }
              />
            ))}
          </CardContent>
          <CardFooter>
            <Button
              color="success"
              isLoading={loading}
              disabled={loading}
              fullWidth
            >
              Create Account
            </Button>
          </CardFooter>
        </form>
      </Form>
    </Card>
  )
}

export default CreateAccount
