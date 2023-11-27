import { Stack, Stat, StatHelpText, StatLabel } from "@chakra-ui/react"

export const RecursiveChild = (value: any) => {
  return (
    value &&
    (value._isPrincipal ? (
      value.toText()
    ) : typeof value === "object" ? (
      Array.isArray(value) || typeof value[0] === "bigint" ? (
        value.toString()
      ) : (
        <Stack ml={2}>
          {Object.entries(value).map(([key, value]: any[]) =>
            key === "env" ? (
              <Stat key={key}>
                <StatLabel>env: &nbsp;</StatLabel>
                <Stat>{Object.keys(value[0])[0]}</Stat>
              </Stat>
            ) : (
              <RecursiveParent key={key} parent={key} child={value} />
            )
          )}
        </Stack>
      )
    ) : (
      value.toString()
    ))
  )
}

interface ParentProps {
  parent: string
  child: any
}

const RecursiveParent: React.FC<ParentProps> = ({ parent, child }) => {
  return parent === "deadline" ? (
    <Stat>
      <StatLabel>{parent}: &nbsp;</StatLabel>
      <StatHelpText>{child?.toString()}</StatHelpText>
    </Stat>
  ) : child ? (
    typeof child === "object" ? (
      <Stat>
        <StatLabel>{parent}: &nbsp;</StatLabel>
        {RecursiveChild(child)}
      </Stat>
    ) : (
      <Stat>
        <StatLabel>{parent}: &nbsp;</StatLabel>
        <StatHelpText>{child?.toString()}</StatHelpText>
      </Stat>
    )
  ) : (
    <>{parent.toString()}</>
  )
}
export default RecursiveParent
