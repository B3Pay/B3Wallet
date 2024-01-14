import React, { useState } from "react"
import { useFieldArray, useFormContext } from "react-hook-form"
import { Button } from "@src/components/ui/button"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { Box } from "@src/components/ui/box"
import { Separator } from "@src/components/ui/separator"
import { Label } from "@src/components/ui/label"
import { Cross1Icon, PlusIcon } from "@radix-ui/react-icons"
import { Card, CardContent } from "@src/components/ui/card"

interface VectorProps extends FieldRouteProps {}

const Vector: React.FC<VectorProps> = ({
  methodField,
  errors,
  registerName,
  ...rest
}) => {
  const { control, setValue } = useFormContext()

  const { fields, append, swap, remove } = useFieldArray({
    control,
    name: registerName as never
  })
  // State to store the selected file name
  const [selectedFileName, setSelectedFileName] = useState("")

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      setSelectedFileName(file.name)
      // Process the file here or pass it to the form handler
      const reader = new FileReader()

      reader.onload = function () {
        // Use reader.result
        const uint8Array = new Uint8Array(reader.result as ArrayBuffer)
        console.log(ArrayBuffer.isView(uint8Array))

        setValue(registerName, Array.from(uint8Array))
      }

      reader.readAsArrayBuffer(file)
    }
  }

  return methodField.label === "vec nat8" ? (
    <div>
      <input
        type="file"
        onChange={handleFileChange}
        style={{ marginBottom: "10px" }}
      />
      {selectedFileName && <div>Selected File: {selectedFileName}</div>}
      {/* Add other UI elements as needed */}
    </div>
  ) : (
    <div>
      <Box className="flex justify-between items-center mt-2">
        <Label className="flex-1 w-full block text-lg font-medium">
          {methodField.label.toTitleCase()}
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
            title={methodField.label.toTitleCase()}
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
              <FieldRoute
                methodField={methodField.fields[0]}
                errors={errors?.[index as never]}
                registerName={`${registerName}.[${index}]`}
                {...rest}
              />
            </CardContent>
          </Card>
        ))}
      </Box>
    </div>
  )
}

export default Vector
