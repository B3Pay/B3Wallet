import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const containerVariants = cva("max-w-7xl mx-auto px-4 sm:px-6 lg:px-8", {
  variants: {
    padding: {
      none: "p-0",
      small: "p-2",
      medium: "p-4",
      large: "p-8"
    },
    margin: {
      none: "m-0",
      small: "m-2",
      medium: "m-4",
      large: "m-8"
    }
  },
  defaultVariants: {
    padding: "medium",
    margin: "none"
  }
})

export interface ContainerProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof containerVariants> {
  asChild?: boolean
}

const Container = React.forwardRef<HTMLDivElement, ContainerProps>(
  ({ asChild, children, className, padding, margin, ...props }, ref) => {
    const Comp = asChild ? Slot : "div"

    return (
      <Comp
        ref={ref}
        className={cn(containerVariants({ padding, margin }), className)}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)

Container.displayName = "Container"

export { Container }
