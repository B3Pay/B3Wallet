import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "lib/utils"

const badgeVariants = cva(
  "inline-flex items-center rounded-md border border-slate-200 px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-slate-950 focus:ring-offset-2 dark:border-slate-800 dark:focus:ring-slate-300",
  {
    variants: {
      variant: {
        default:
          "border-transparent bg-slate-900 text-slate-50 shadow hover:bg-slate-900/80 dark:bg-slate-50 dark:text-slate-900 dark:hover:bg-slate-50/80",
        secondary:
          "border-transparent bg-slate-100 text-slate-900 hover:bg-slate-100/80 dark:bg-slate-800 dark:text-slate-50 dark:hover:bg-slate-800/80",
        destructive:
          "border-transparent bg-red-500 text-slate-50 shadow hover:bg-red-500/80 dark:bg-red-900 dark:text-slate-50 dark:hover:bg-red-900/80",
        outline: "text-slate-950 dark:text-slate-50"
      },
      color: {
        primary: "bg-primary/75 border-primary text-primary",
        secondary: "bg-secondary/75 border-secondary text-secondary",
        error: "bg-error/75 border-error text-error",
        success: "bg-success/75 border-success text-success",
        warning: "bg-warning/75 border-warning text-warning",
        info: "bg-info/75 border-info text-info",
        muted: "bg-gray-400/75 border-gray-500"
      },
      size: {
        none: "w-auto h-auto",
        xs: "w-5 h-5",
        sm: "w-8 h-8",
        md: "w-9 h-9",
        lg: "w-10 h-10",
        xl: "w-12 h-12"
      },
      asIcon: {
        true: "rounded-full"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "muted",
      size: "none",
      asIcon: false
    }
  }
)

export interface BadgeProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof badgeVariants> {
  asIcon?: boolean
}

function Badge({
  className,
  color,
  size,
  asIcon,
  variant,
  ...props
}: BadgeProps) {
  return (
    <div
      className={cn(
        badgeVariants({ variant, color, size, asIcon }),
        "flex justify-center items-center",
        className
      )}
      {...props}
    />
  )
}

export { Badge, badgeVariants }
