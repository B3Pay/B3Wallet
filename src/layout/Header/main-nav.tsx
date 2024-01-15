import NextLink from "next/link"
import * as React from "react"
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList
} from "@src/components/ui/navigation-menu"
import { useRouter } from "next/router"
import {
  NavigationMenuIndicator,
  NavigationMenuLinkProps
} from "@radix-ui/react-navigation-menu"
import { cn } from "@src/lib/utils"

const navItems = [
  {
    title: "Home",
    href: "/"
  },
  {
    title: "Forge",
    href: "/forge"
  },
  {
    title: "System",
    href: "/system"
  },
  {
    title: "Wallet",
    href: "/app/b3wallet"
  },
  {
    title: "Candid",
    href: "/candid"
  }
]

export function MainNav({ className }: React.HTMLAttributes<HTMLElement>) {
  return (
    <NavigationMenu className={className}>
      <NavigationMenuList>
        {navItems.map((item, index) => (
          <NavigationMenuItem key={index} className="pr-2">
            <Link href={item.href}>{item.title}</Link>
          </NavigationMenuItem>
        ))}
        <NavigationMenuIndicator className="NavigationMenuIndicator" />
      </NavigationMenuList>
    </NavigationMenu>
  )
}

interface LinkProps extends NavigationMenuLinkProps {
  href: string
}

const Link: React.FC<LinkProps> = ({ href, ...props }) => {
  const router = useRouter()
  const isActive = router.asPath === href

  return (
    <NextLink href={href} passHref legacyBehavior>
      <NavigationMenuLink
        className={cn(
          "flex h-7 items-center justify-center rounded-full px-4 text-center text-sm transition-colors hover:text-primary",
          isActive
            ? "bg-muted font-medium text-primary"
            : "text-muted-foreground",
          props.className
        )}
        {...props}
      />
    </NextLink>
  )
}
