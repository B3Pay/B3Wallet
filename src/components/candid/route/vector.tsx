import React from "react"
import { useFieldArray } from "react-hook-form"
import { Button } from "@src/components/ui/button"
import { Route, RouteProps } from "."
import { Box } from "@src/components/ui/box"
import { Label } from "@src/components/ui/label"
import { Separator } from "@src/components/ui/separator"
import { Cross1Icon, PlusIcon } from "@radix-ui/react-icons"
import { Card, CardContent } from "@src/components/ui/card"

export interface VectorProps extends RouteProps<"vector"> {}

const Vector: React.FC<VectorProps> = ({
  extractedField,
  errors,
  registerName,
  shouldUnregister
}) => {
  const { fields, append, swap, remove } = useFieldArray({
    name: registerName as never,
    shouldUnregister
  })

  return (
    <div>
      <Box className="flex justify-between items-center mt-2">
        <Label className="flex-1 w-full block text-lg font-medium">
          {extractedField.label.toTitleCase()}
        </Label>
        <Button onClick={() => append("")} asIconButton>
          <PlusIcon />
        </Button>
      </Box>
      {fields.length > 0 && <Separator className="my-2" />}
      <Box className="grid gap-2">
        {fields.map((item, index) => (
          <Card
            key={item.id}
            noShadow
            dashedBorder
            border={2}
            icon={index + 1}
            iconProps={{
              color: "warning",
              roundSide: "tl",
              diagonalRoundSide: "l"
            }}
            title={extractedField.label.toTitleCase()}
            action={
              <Box className="flex">
                {index !== 0 && (
                  <Button
                    roundSide="bl"
                    onClick={() => swap(index, index - 1)}
                    color="secondary"
                    asIconButton
                    innerShadow
                    className="border-dashed "
                  >
                    ↑
                  </Button>
                )}
                {index !== fields.length - 1 && (
                  <Button
                    roundSide={index > 0 ? "none" : "bl"}
                    onClick={() => swap(index, index + 1)}
                    color="info"
                    asIconButton
                    innerShadow
                    className="border-dashed"
                  >
                    ↓
                  </Button>
                )}
                <Button
                  onClick={() => remove(index)}
                  diagonalRoundSide={fields.length === 1 ? "r" : "none"}
                  roundSide="tr"
                  asIconButton
                  innerShadow
                  noShadow
                  color="error"
                  className="border-dashed"
                >
                  <Cross1Icon />
                </Button>
              </Box>
            }
          >
            <CardContent>
              <Route
                extractedField={extractedField.fields[0]}
                errors={errors?.[index as never]}
                registerName={`${registerName}.[${index}]`}
              />
            </CardContent>
          </Card>
        ))}
      </Box>
    </div>
  )
}

export { Vector }
