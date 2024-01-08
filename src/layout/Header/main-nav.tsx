import Link from "next/link"
import { cn } from "@src/lib/utils"
import { usePathname } from "next/navigation"

const navItems = [
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
  },
  {
    title: "Dashboard",
    href: "/dashboard"
  }
]

export function MainNav({
  className,
  ...props
}: React.HTMLAttributes<HTMLElement>) {
  const pathname = usePathname()

  return (
    <nav
      className={cn("flex items-center space-x-4 lg:space-x-6", className)}
      {...props}
    >
      {navItems.map((item, index) => (
        <Link
          href={item.href}
          key={index}
          className={cn(
            "text-sm font-medium",
            pathname === item.href ? "text-gray-900" : "text-gray-500"
          )}
        >
          {item.title}
        </Link>
      ))}
    </nav>
  )
}
