import { Principal as PrincipalId } from "@dfinity/principal"
import { useFormContext } from "react-hook-form"
import { RouteProps } from "../route"
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@src/components/ui/form"
import { Input } from "@src/components/ui/input"
import {
  ClipboardIcon,
  InfoCircledIcon,
  PersonIcon
} from "@radix-ui/react-icons"
import { Button } from "@src/components/ui/button"
import { useUserPrincipal } from "@src/service/system"

export interface PrincipalProps extends RouteProps {}

const Principal: React.FC<PrincipalProps> = ({
  registerName,
  errors,
  extractedField,
  shouldUnregister
}) => {
  const userPrincipal = useUserPrincipal()
  const { setValue, resetField, setError } = useFormContext()

  const blurHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.value === "") {
      setValue(registerName as never, "" as never)
      return
    }
    const inputValue = e.target.value
    resetField(registerName as never, { defaultValue: inputValue as never })
    const isValid = validate(inputValue)

    if (isValid === true) {
      const principal = PrincipalId.fromText(inputValue)

      setValue(registerName as never, principal as never)
    } else {
      setError(registerName as never, {
        type: "manual",
        message: isValid
      })
    }
  }

  function validate(x: any) {
    if (x._isPrincipal === true) {
      return true
    }
    try {
      if (x.length < 7) {
        throw new Error("Principal is too short")
      }
      const principal = PrincipalId.fromText(x)

      const validate = extractedField.validate(principal)

      if (typeof validate === "string") {
        throw new Error(validate)
      }
      return true
    } catch (error) {
      return (error as any).message
    }
  }

  const errorMessage = errors?.message?.toString()

  return (
    <FormItem>
      <FormLabel>{extractedField.label.toTitleCase()}</FormLabel>
      <div className="flex items-center">
        <FormControl>
          <FormField
            shouldUnregister={shouldUnregister}
            name={registerName}
            defaultValue={extractedField.defaultValue}
            rules={{ ...extractedField, validate }}
            render={({ field }) => (
              <Input
                {...field}
                className="flex-1"
                icon={<InfoCircledIcon />}
                color="alert"
                roundSide="l"
                type={extractedField.type}
                placeholder={extractedField.type}
                closeHandler={() => {
                  setValue(registerName as never, "" as never)
                }}
                onBlur={blurHandler}
              />
            )}
          />
        </FormControl>
        {userPrincipal && (
          <Button
            roundSide="none"
            asIconButton
            color="alert"
            variant="outline"
            onClick={() => {
              setValue(registerName as never, userPrincipal as never)
            }}
          >
            <PersonIcon />
          </Button>
        )}
        <Button
          roundSide="r"
          variant="outline"
          color="alert"
          asIconButton
          onClick={() => {
            navigator.clipboard.readText().then(text => {
              setValue(registerName as never, text as never)
            })
          }}
        >
          <ClipboardIcon />
        </Button>
      </div>
      <FormMessage>{errorMessage}</FormMessage>
    </FormItem>
  )
}

export { Principal }
