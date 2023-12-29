import React from "react"
import { useFieldArray, useFormContext } from "react-hook-form"
import { Button } from "components/ui/button"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { Box } from "components/ui/box"
import { Separator } from "components/ui/separator"
import { Label } from "components/ui/label"
import { Cross1Icon, PlusIcon } from "@radix-ui/react-icons"
import { Icon } from "components/ui/icon"
import { Card, CardContent } from "components/ui/card"

interface VectorProps extends FieldRouteProps {}

const Vector: React.FC<VectorProps> = ({
  methodField,
  errors,
  registerName
}) => {
  const { control } = useFormContext()

  const { fields, append, swap, remove } = useFieldArray({
    control,
    name: registerName as never
  })

  return (
    <Box className="mb-2">
      <div className="flex justify-between items-center">
        <Label className="flex-1 w-full block text-lg font-medium">
          {methodField.label.toTitleCase()}
        </Label>
        <Button onClick={() => append("")} asIconButton>
          <PlusIcon />
        </Button>
      </div>
      {fields.length > 0 && <Separator className="my-2" />}
      {fields.map((item, index) => (
        <Card
          key={item.id}
          noShadow
          margin="sm"
          dashedBorder
          color="primary"
          icon={
            <Icon roundSide="tl" className="border-dashed">
              {index + 1}
            </Icon>
          }
          title={methodField.label.toTitleCase()}
          action={
            <Box className="flex">
              <div className="flex">
                {index !== 0 && (
                  <Button
                    onClick={() => swap(index, index - 1)}
                    color="secondary"
                    asIconButton
                    roundSide="none"
                    className="border-dashed"
                  >
                    ↑
                  </Button>
                )}
                {index !== fields.length - 1 && (
                  <Button
                    onClick={() => swap(index, index + 1)}
                    color="info"
                    roundSide="none"
                    asIconButton
                    className="border-dashed"
                  >
                    ↓
                  </Button>
                )}
              </div>
              <Button
                onClick={() => remove(index)}
                roundSide="tr"
                asIconButton
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
            <FieldRoute
              methodField={methodField.fields?.[0]}
              errors={errors?.[index as never]}
              registerName={`${registerName}.[${index}]`}
            />
          </CardContent>
        </Card>
      ))}
    </Box>
  )
}

export default Vector
