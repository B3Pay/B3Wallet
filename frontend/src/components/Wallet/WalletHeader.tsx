import { AtSignIcon, RepeatClockIcon, SettingsIcon } from "@chakra-ui/icons"
import { Box, IconButton, Stack, StackProps } from "@chakra-ui/react"
import { Mode } from "."
import { B3Wallet } from "../../service/actor"
import Address from "./Address"
import ConfirmationModal from "./ConfirmModal"

interface WalletHeaderProps extends StackProps {
  walletCanisterId: string
  actor: B3Wallet
  mode: Mode
  toggleMode: (mode: Mode) => void
  fetchAccounts: () => void
  refreshWallet: () => void
}

const WalletHeader: React.FC<WalletHeaderProps> = ({
  walletCanisterId,
  actor,
  mode,
  toggleMode,
  fetchAccounts,
  refreshWallet,
  ...rest
}) => {
  return (
    <Stack
      direction="row"
      paddingBottom={1}
      marginBottom={2}
      borderBottom="1px solid"
      borderColor="gray.200"
      justify="space-between"
      align="center"
      position="relative"
      {...rest}
    >
      <Box
        animation="spin 2s linear infinite"
        transition="all 0.25s ease-in-out"
        pos="absolute"
        border="1px solid"
        borderRadius="full"
        width="30px"
        bottom={-0.5}
        borderColor={
          mode === Mode.Processed
            ? "blue.500"
            : mode === Mode.Settings
            ? "purple.500"
            : mode === Mode.Accounts
            ? "pink.500"
            : "gray.500"
        }
        right={
          mode === Mode.Processed
            ? "calc(60px + 20px)"
            : mode === Mode.Settings
            ? "calc(30px + 10px)"
            : mode === Mode.Accounts
            ? "0px"
            : "0px"
        }
      />
      <Address flex={1} address={walletCanisterId} overflow="hidden" />
      <ConfirmationModal
        actor={actor}
        fetchAccounts={fetchAccounts}
        refreshWallet={refreshWallet}
      />
      <IconButton
        variant={mode === Mode.Processed ? "solid" : "outline"}
        colorScheme="blue"
        onClick={() => toggleMode(Mode.Processed)}
        aria-label="Recent transactions"
        icon={<RepeatClockIcon />}
      />
      <IconButton
        variant={mode === Mode.Settings ? "solid" : "outline"}
        aria-label="Settings"
        onClick={() => toggleMode(Mode.Settings)}
        icon={<SettingsIcon />}
        colorScheme="purple"
      />
      <IconButton
        variant={mode === Mode.Accounts ? "solid" : "outline"}
        colorScheme="pink"
        onClick={() => toggleMode(Mode.Accounts)}
        aria-label="Accounts"
        icon={<AtSignIcon />}
      />
    </Stack>
  )
}

export default WalletHeader
