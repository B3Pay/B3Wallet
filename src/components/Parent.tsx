import { Stack, Stat, StatHelpText, StatLabel } from "@chakra-ui/react"

const Child = (value: any) =>
  value &&
  (value._isPrincipal ? (
    value.toText()
  ) : typeof value === "object" ? (
    Array.isArray(value) || typeof value[0] === "number" ? (
      value.toString()
    ) : (
      <Stack ml={2}>
        {Object.entries(value).map(([key, value]) => (
          <Parent key={key} parent={key} child={value} />
        ))}
      </Stack>
    )
  ) : value === "true" ? (
    "Yes"
  ) : value === "false" ? (
    "No"
  ) : (
    value.toString()
  ))

interface ParentProps {
  parent: string
  child: any
}

const Parent: React.FC<ParentProps> = ({ parent, child }) =>
  child ? (
    typeof child === "object" ? (
      <Stat>
        <StatLabel>{parent}: &nbsp;</StatLabel>
        {Child(child)}
      </Stat>
    ) : (
      <Stat>
        <StatLabel>{parent}: &nbsp;</StatLabel>
        <StatHelpText>{child?.toString()}</StatHelpText>
      </Stat>
    )
  ) : null

export default Parent
