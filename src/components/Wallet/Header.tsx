import { AtSignIcon, RepeatClockIcon, SettingsIcon } from "@chakra-ui/icons"
import { Divider, IconButton, Stack, StackProps } from "@chakra-ui/react"
import ConfirmationModal from "components/Wallet/ConfirmModal"
import { B3Wallet } from "service/actor"
import { Mode } from "."
import Address from "./Address"

interface WalletHeaderProps extends StackProps {
  walletCanisterId: string
  actor: B3Wallet
  mode: Mode
  toggleMode: (mode: Mode) => void
  fetchAccounts: () => void
}

const WalletHeader: React.FC<WalletHeaderProps> = ({
  walletCanisterId,
  actor,
  mode,
  toggleMode,
  fetchAccounts,
  ...rest
}) => {
  return (
    <Stack direction="row" justify="space-between" align="center" {...rest}>
      <Address flex={1} address={walletCanisterId} />
      <ConfirmationModal actor={actor} fetchAccounts={fetchAccounts} />
      <Divider orientation="vertical" />
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
