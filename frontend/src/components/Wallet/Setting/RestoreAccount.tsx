import {
  Accordion,
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  Box,
  Button,
  CardBody,
  Input,
  Select,
  Stack,
  Text
} from "@chakra-ui/react"
import Loading from "components/Loading"
import { Environment } from "declarations/b3_wallet/b3_wallet.did"
import { IS_LOCAL } from "helpers/config"
import useToastMessage from "hooks/useToastMessage"
import { useState } from "react"
import { B3BasicWallet, B3Wallet } from "service"

interface RestoreAccountProps {
  actor: B3Wallet | B3BasicWallet
  fetchAccounts: () => void
}

const RestoreAccount: React.FC<RestoreAccountProps> = ({
  actor,
  fetchAccounts
}) => {
  const [loading, setLoading] = useState(false)
  const [nonce, setNonce] = useState("")

  const errorToast = useToastMessage()

  const [environment, setEnvironment] = useState<Environment>(
    IS_LOCAL
      ? {
          Development: null
        }
      : {
          Production: null
        }
  )

  function onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    setNonce(e.target.value)
  }

  const createAccount = async () => {
    if (!nonce) {
      return
    }

    let nonceNumber = BigInt(nonce)

    setLoading(true)
    actor
      .account_restore(environment, nonceNumber)
      .then(() => {
        setLoading(false)
        fetchAccounts()
      })
      .catch(e => {
        errorToast({
          title: "Error",
          description: e.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })

        setLoading(false)
      })
  }

  return (
    <Stack
      direction="column"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
    >
      {loading && <Loading title="Restoring account" />}
      <Accordion allowToggle>
        <AccordionItem border="none" _focus={{ boxShadow: "none" }}>
          <Box>
            <Stack
              direction="row"
              justify="space-between"
              align="center"
              px={4}
              py={2}
            >
              <Text flex={6} fontSize="md" fontWeight="bold">
                Restore Account
              </Text>
              <Stack fontSize="sm" fontWeight="semibold">
                <AccordionButton borderRadius="lg" flex={1}>
                  <AccordionIcon />
                </AccordionButton>
              </Stack>
            </Stack>
            <AccordionPanel>
              <CardBody
                borderTop="1px"
                borderColor="gray.200"
                position="relative"
                py={4}
                px={2}
              >
                <Stack direction="row" justify="space-between" align="center">
                  <Select
                    flex={6}
                    value={Object.keys(environment)[0]}
                    onChange={e => {
                      const env = e.target.value

                      setEnvironment({ [env]: null } as Environment)
                    }}
                  >
                    {IS_LOCAL && (
                      <option value="Development">Development</option>
                    )}
                    <option value="Production">Production</option>
                    <option value="Staging">Staging</option>
                  </Select>
                  <Input
                    id="nonce"
                    alt="Name"
                    flex={2}
                    type="number"
                    placeholder="Nonce"
                    value={nonce}
                    onChange={onChangeName}
                  />
                  <Button
                    onClick={createAccount}
                    isLoading={loading}
                    flex={4}
                    colorScheme="green"
                  >
                    Restore
                  </Button>
                </Stack>
              </CardBody>
            </AccordionPanel>
          </Box>
        </AccordionItem>
      </Accordion>
    </Stack>
  )
}

export default RestoreAccount
