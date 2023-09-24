import { Accordion, AccordionItem, Stack, Text } from "@chakra-ui/react"

import { ProcessedOperation } from "declarations/b3_wallet/b3_wallet.did"
import { useEffect, useState } from "react"
import { B3Wallet } from "service"
import ProcessedItem from "./ProcessedItem"

interface ProcessedProps {
  setLoading: (loading: boolean) => void
  actor: B3Wallet
}

const ProcessedList: React.FC<ProcessedProps> = ({ setLoading, actor }) => {
  const [processedList, setProcessedList] = useState<ProcessedOperation[]>([])

  useEffect(() => {
    console.log("fetching processed")
    setLoading(true)

    actor
      .get_processed_list()
      .then(processed => {
        console.log(processed)
        setProcessedList(processed.reverse())
        setLoading(false)
      })
      .catch(e => {
        console.log(e)
        setLoading(false)
      })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return (
    <Stack spacing={4}>
      <Text fontSize="xl" fontWeight="bold">
        Processed
      </Text>
      <Accordion allowToggle>
        {processedList.map((request, i) => (
          <AccordionItem key={i} border="none" _focus={{ boxShadow: "none" }}>
            {({ isExpanded }) => (
              <ProcessedItem {...request} isExpanded={isExpanded} />
            )}
          </AccordionItem>
        ))}
      </Accordion>
    </Stack>
  )
}

export default ProcessedList
