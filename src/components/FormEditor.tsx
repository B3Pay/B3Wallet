import React, { useEffect, useState } from "react"
import { DragDropContext, Droppable, Draggable } from "react-beautiful-dnd"
import { Input } from "./ui/input"
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectGroup,
  SelectLabel,
  SelectItem,
  SelectValue
} from "@src/components/ui/select"
import { Box } from "./ui/box"
import { Icon } from "./ui/icon"
import {
  CheckIcon,
  Cross2Icon,
  DragHandleDots1Icon,
  FontRomanIcon,
  ListBulletIcon,
  Pencil1Icon,
  Pencil2Icon
} from "@radix-ui/react-icons"
import { Button } from "./ui/button"
import { Card, CardContent, CardFooter } from "./ui/card"
import { Label } from "./ui/label"
import { useForm } from "react-hook-form"

const colors = [
  "primary",
  "secondary",
  "error",
  "success",
  "warning",
  "info",
  "muted"
] as const

type Color = (typeof colors)[number]

const initialFields: {
  id: string
  editableLabel: string
  editing: boolean
  label: string
  color: Color
}[] = [
  {
    id: "field-1",
    label: "Field 1",
    editableLabel: "Field 1",
    editing: true,
    color: colors[3]
  },
  {
    id: "field-2",
    label: "Field 2",
    editableLabel: "Field 2",
    editing: true,
    color: colors[3]
  },
  {
    id: "field-3",
    label: "Field 3",
    editableLabel: "Field 3",
    editing: true,
    color: colors[3]
  }
]

const FormEditor = () => {
  const { register, handleSubmit, setValue, getValues } = useForm()

  const loadFormState = () => {
    const serializedState = localStorage.getItem("formState")
    return serializedState ? JSON.parse(serializedState) : initialFields
  }

  const [fields, setFields] = useState(initialFields)

  useEffect(() => {
    setFields(loadFormState())
  }, [])

  const onDragEnd = (result: any) => {
    if (!result.destination) return
    const items = Array.from(fields)
    const [reorderedItem] = items.splice(result.source.index, 1)
    items.splice(result.destination.index, 0, reorderedItem)
    setFields(items)
  }

  const handleColorChange = (value: Color, index: number) => {
    console.log("value", value, "index", index)
    setFields(prevFields => {
      const newFields = [...prevFields]
      newFields[index].color = value
      return newFields
    })
  }

  const saveFormState = () => {
    const serializedState = JSON.stringify(fields)
    localStorage.setItem("formState", serializedState)
  }

  const handleLabelChange = (newLabel: string, index: number) => {
    setFields(prevFields => {
      const newFields = [...prevFields]
      newFields[index].editableLabel = newLabel
      return newFields
    })
  }

  const saveLabel = (index: number) => {
    setFields(prevFields => {
      const newFields = [...prevFields]
      newFields[index].label = newFields[index].editableLabel
      return newFields
    })
  }

  return (
    <Card title="Form Editor" icon={<ListBulletIcon />}>
      <CardContent>
        <DragDropContext onDragEnd={onDragEnd}>
          <Droppable droppableId="fields">
            {provided => (
              <div
                {...provided.droppableProps}
                ref={provided.innerRef}
                className="flex flex-col"
              >
                {fields.map((field, index) => (
                  <Draggable
                    key={field.id}
                    draggableId={field.id}
                    index={index}
                  >
                    {provided => (
                      <Box
                        ref={provided.innerRef}
                        {...provided.draggableProps}
                        {...provided.dragHandleProps}
                        style={{
                          ...provided.draggableProps.style
                        }}
                        className="flex mb-2 justify-between items-end"
                      >
                        <Icon roundSide="l" color={field.color}>
                          <DragHandleDots1Icon />
                        </Icon>
                        <Box className="flex flex-col flex-grow">
                          {field.editing ? (
                            <div className="flex">
                              <Icon
                                size="sm"
                                roundSide="tl"
                                variant="filled"
                                color={field.color}
                              >
                                <FontRomanIcon />
                              </Icon>
                              <Input
                                type="text"
                                size="sm"
                                color={field.color}
                                placeholder={field.label}
                                className="flex-grow"
                                variant="filled"
                                roundSide="none"
                                value={field.editableLabel}
                                onChange={e =>
                                  handleLabelChange(e.target.value, index)
                                }
                                onBlur={() => saveLabel(index)}
                              />
                              <Button
                                roundSide="none"
                                size="sm"
                                variant="filled"
                                color="success"
                                onClick={() =>
                                  setFields(prevFields => {
                                    const newFields = [...prevFields]
                                    newFields[index].editing = false
                                    return newFields
                                  })
                                }
                                asIconButton
                                aria-label="save field"
                              >
                                <CheckIcon className="w-3 h-3" />
                              </Button>
                              <Button
                                roundSide="tr"
                                variant="filled"
                                size="sm"
                                color="error"
                                onClick={() =>
                                  setFields(prevFields => {
                                    const newFields = [...prevFields]
                                    newFields[index].editing = false
                                    newFields[index].editableLabel =
                                      newFields[index].label
                                    return newFields
                                  })
                                }
                                asIconButton
                                aria-label="close edit field"
                              >
                                <Cross2Icon className="w-3 h-3" />
                              </Button>
                            </div>
                          ) : (
                            <div>
                              <Label>{field.label}</Label>
                              <Button
                                size="xs"
                                variant="link"
                                roundSize="lg"
                                onClick={() =>
                                  setFields(prevFields => {
                                    const newFields = [...prevFields]
                                    newFields[index].editing = true
                                    return newFields
                                  })
                                }
                                asIconButton
                                aria-label="Edit field"
                              >
                                <Pencil1Icon />
                              </Button>
                            </div>
                          )}
                          <Input
                            type="text"
                            color={field.color}
                            placeholder={field.label}
                            className="flex-grow"
                            roundSide="none"
                          />
                        </Box>
                        <Select
                          onValueChange={(value: Color) =>
                            handleColorChange(value, index)
                          }
                          value={field.color}
                        >
                          <SelectTrigger roundSide="r" color={field.color}>
                            <SelectValue placeholder="Select a Color" />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectGroup>
                              <SelectLabel>Colors</SelectLabel>
                              {colors.map(color => (
                                <SelectItem value={color} key={color}>
                                  {color.toTitleCase()}
                                </SelectItem>
                              ))}
                            </SelectGroup>
                          </SelectContent>
                        </Select>
                      </Box>
                    )}
                  </Draggable>
                ))}
                {provided.placeholder}
              </div>
            )}
          </Droppable>
        </DragDropContext>
      </CardContent>
      <CardFooter>
        <Button onClick={saveFormState} fullWidth>
          Save
        </Button>
      </CardFooter>
    </Card>
  )
}

export default FormEditor
