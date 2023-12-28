import * as React from "react"
import { VariantProps, cva } from "class-variance-authority"
import { cn } from "lib/utils"

const cardVariants = cva("shadow", {
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      md: "text-base",
      lg: "text-lg",
      xl: "text-xl"
    },
    padding: {
      none: "p-0",
      xs: "p-1",
      sm: "p-2",
      md: "p-4",
      lg: "p-8",
      xl: "p-10"
    },
    margin: {
      none: "m-0",
      xs: "m-1",
      sm: "m-2",
      md: "m-4",
      lg: "m-8",
      xl: "m-10"
    },
    color: {
      primary: "bg-primary/5 border-primary",
      secondary: "bg-secondary/5 border-secondary",
      error: "bg-error/5 border-error",
      success: "bg-success/5 border-success",
      warning: "bg-warning/5 border-warning",
      info: "bg-info/5 border-info",
      muted: "bg-gray-400/5 border-gray-500"
    },
    border: {
      0: "border-0",
      1: "border-1",
      2: "border-2",
      3: "border-3",
      4: "border-4"
    },
    round: {
      none: "rounded-none",
      both: "rounded-md",
      left: "rounded-l-md",
      right: "rounded-r-md",
      t: "rounded-t-md",
      b: "rounded-b-md",
      tl: "rounded-tl-md",
      tr: "rounded-tr-md",
      bl: "rounded-bl-md",
      br: "rounded-br-md"
    }
  },
  defaultVariants: {
    color: "muted",
    size: "md",
    border: 1,
    round: "none",
    padding: "none",
    margin: "xs"
  }
})

export interface CardProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof cardVariants> {
  asChild?: boolean
  noShadow?: boolean
}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      size,
      noShadow,
      color,
      border,
      round,
      padding,
      margin,
      ...props
    },
    ref
  ) => (
    <div
      ref={ref}
      className={cn(
        cardVariants({ size, round, border, color, padding, margin }),
        noShadow && "shadow-none",
        size && size !== "xs" && `rounded-${round}-${size}`,
        className
      )}
      {...props}
    />
  )
)
Card.displayName = "Card"

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col space-y-1.5 p-6", className)}
    {...props}
  />
))
CardHeader.displayName = "CardHeader"

export interface CardActionProps extends React.HTMLAttributes<HTMLDivElement> {
  icon?: React.ReactNode
}

const CardAction = React.forwardRef<HTMLDivElement, CardActionProps>(
  ({ className, children, title, icon, ...props }, ref) => (
    <div
      ref={ref}
      className="flex items-center justify-between mb-2"
      {...props}
    >
      <div className="flex-none">{icon}</div>
      <CardTitle className="flex-1 pl-2">{title}</CardTitle>
      <div className="flex items-center justify-between">{children}</div>
    </div>
  )
)
CardAction.displayName = "CardAction"

const CardTitle = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h3
    ref={ref}
    className={cn("font-semibold leading-none tracking-tight", className)}
    {...props}
  />
))
CardTitle.displayName = "CardTitle"

const CardDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn("text-sm text-slate-500 dark:text-slate-400", className)}
    {...props}
  />
))
CardDescription.displayName = "CardDescription"

const CardContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div ref={ref} className={cn("p-6 pt-0", className)} {...props} />
))
CardContent.displayName = "CardContent"

const CardFooter = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex items-center p-6 pt-0", className)}
    {...props}
  />
))
CardFooter.displayName = "CardFooter"

export {
  Card,
  CardContent,
  CardAction,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
}
