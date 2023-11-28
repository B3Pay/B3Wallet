import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const iconVariants = cva(
  "inline-flex items-center justify-center text-inherit p-1",
  {
    variants: {
      size: {
        xs: "w-6 h-6",
        sm: "w-8 h-8 p-1.5",
        md: "w-9 h-9 p-1.5",
        lg: "w-10 h-10 p-2",
        xl: "w-12 h-12 p-2"
      }
    },
    defaultVariants: {
      size: "md"
    }
  }
)

export interface IconProps
  extends React.HTMLAttributes<HTMLSpanElement>,
    VariantProps<typeof iconVariants> {}

const Icon = React.forwardRef<HTMLSpanElement, IconProps>(
  ({ children, className, size, ...props }, ref) => {
    return (
      <Slot
        className={cn(iconVariants({ size }), className)}
        ref={ref}
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
