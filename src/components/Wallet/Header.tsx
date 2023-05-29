import { RepeatClockIcon, SettingsIcon } from "@chakra-ui/icons"
import { IconButton, Stack, Text } from "@chakra-ui/react"
import ConfirmationModal from "components/ConfirmModal"
import { B3Wallet } from "service/actor"

interface WalletHeaderProps {
  walletCanisterId: string
  actor: B3Wallet
  toggleSetting: () => void
}

const WalletHeader: React.FC<WalletHeaderProps> = ({
  walletCanisterId,
  actor,
  toggleSetting
}) => {
  return (
    <Stack direction="row" justify="space-between" align="center">
      <Text flex={1}>
        <b>Canister:</b> {walletCanisterId}
      </Text>
      <ConfirmationModal
        actor={actor}
        request={{
          type: "Transfer",
          details: "Transfer 1 ICP to 0x1234567890abcdef"
        }}
      />
      <IconButton
        colorScheme="blue"
        aria-label="Recent transactions"
        icon={<RepeatClockIcon />}
      />
      <IconButton
        aria-label="Settings"
        onClick={toggleSetting}
        icon={<SettingsIcon />}
        colorScheme="purple"
      />
    </Stack>
  )
}

export default WalletHeader
