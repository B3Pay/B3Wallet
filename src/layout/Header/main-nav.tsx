import Link from "next/link"

import { cn } from "@src/lib/utils"

export function MainNav({
  className,
  ...props
}: React.HTMLAttributes<HTMLElement>) {
  return (
    <nav
      className={cn("flex items-center space-x-4 lg:space-x-6", className)}
      {...props}
    >
      <Link
        href="/system"
        className="text-sm font-medium transition-colors hover:text-primary"
      >
        System
      </Link>
      <Link
        href="/wallet"
        className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Wallet
      </Link>
      <Link
        href="/candid"
        className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Candid
      </Link>
      <Link
        href="/dashboard"
        className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Dashboard
      </Link>
    </nav>
  )
}
