import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const iconVariants = cva(
  "inline-flex items-center justify-center text-inherit p-1",
  {
    compoundVariants: [
      {
        asButton: true,
        className: "cursor-pointer"
      }
    ],
    variants: {
      asButton: {
        true: "p-0"
      },
      color: {
        primary: "text-primary",
        secondary: "text-secondary",
        error: "text-error",
        success: "text-success",
        warning: "text-warning",
        info: "text-info",
        muted: "text-gray-500"
      },
      size: {
        xs: "w-5 h-5",
        sm: "w-8 h-8 p-1.5",
        md: "w-9 h-9 p-1.5",
        lg: "w-10 h-10 p-2",
        xl: "w-12 h-12 p-2"
      }
    },
    defaultVariants: {
      color: "muted",
      size: "md"
    }
  }
)

export interface IconProps
  extends Omit<React.HTMLAttributes<HTMLSpanElement>, "color">,
    VariantProps<typeof iconVariants> {
  asButton?: boolean
}

const Icon = React.forwardRef<HTMLSpanElement, IconProps>(
  ({ children, asButton, className, color, size, ...props }, ref) => {
    Slot

    return (
      <Slot
        ref={ref}
        className={cn(iconVariants({ size, color, asButton }), className)}
        {...props}
      >
        {children}
      </Slot>
    )
  }
)

Icon.displayName = "Icon"
Icon.defaultProps = {
  size: "md"
}

export { Icon }
