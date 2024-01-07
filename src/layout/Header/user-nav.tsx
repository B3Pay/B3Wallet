import { AvatarIcon } from "@radix-ui/react-icons"
import { Avatar, AvatarFallback } from "@src/components/ui/avatar"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuTrigger
} from "@src/components/ui/dropdown-menu"
import { cn } from "@src/lib/utils"
import { useSystemAuthClient } from "@src/service/system"
import Address from "@src/components/Address"
import { Button } from "@src/components/ui/button"

interface UserNavProps {
  className?: string
}

const UserNav: React.FC<UserNavProps> = ({ className }) => {
  const {
    login,
    logout,
    loginLoading,
    loginError,
    identity,
    authenticating,
    authenticated
  } = useSystemAuthClient()

  return authenticated ? (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Avatar className={cn("h-8 w-8", className)} asButton>
          {/* <AvatarImage src="/avatars/01.png" alt="@shadcn" /> */}
          <AvatarFallback>
            {identity?.getPrincipal().toText().slice(0, 2)}
          </AvatarFallback>
        </Avatar>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56" align="end" forceMount>
        <DropdownMenuLabel className="font-normal">
          <Address
            size="sm"
            asMenuItem
            address={identity!.getPrincipal().toText()}
          />
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem>
            Profile
            <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>
            Billing
            <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>
            Settings
            <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>New Team</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuItem onClick={() => logout()} disabled={authenticating}>
          Log out
          <DropdownMenuShortcut>⇧⌘Q</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  ) : (
    <Button
      onClick={() =>
        login({
          identityProvider:
            process.env.DFX_NETWORK === "ic"
              ? "https://identity.ic0.app/#authorize"
              : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.127.0.0.1:4943/#authorize`
        })
      }
      disabled={loginLoading || authenticating}
      isLoading={loginLoading}
      asIconButton
    >
      <AvatarIcon />
    </Button>
  )
}

export default UserNav
