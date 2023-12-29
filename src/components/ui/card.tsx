import * as React from "react"
import { VariantProps, cva } from "class-variance-authority"
import { cn } from "lib/utils"
import { Box, colorVariants, marginVariants, paddingVariants } from "./box"

const cardVariants = cva("shadow", {
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      md: "text-base",
      lg: "text-lg",
      xl: "text-xl"
    },
    roundSize: {
      none: "rounded-none",
      xs: "rounded-xs",
      sm: "rounded-sm",
      md: "rounded-md",
      lg: "rounded-lg",
      xl: "rounded-xl",
      "2xl": "rounded-2xl",
      "3xl": "rounded-3xl"
    }
  },
  defaultVariants: {
    size: "md",
    roundSize: "md"
  }
})

export interface CardProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof colorVariants>,
    VariantProps<typeof cardVariants>,
    VariantProps<typeof paddingVariants>,
    VariantProps<typeof marginVariants> {
  dashedBorder?: boolean
  icon?: React.ReactNode
  asChild?: boolean
  noRadius?: boolean
  noShadow?: boolean
  border?: 0 | 1 | 2 | 3 | 4
  action?: React.ReactNode
}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      size,
      noShadow,
      noRadius,
      color,
      border = 2,
      dashedBorder,
      margin,
      padding,
      icon,
      title,
      roundSize,
      action,
      children,
      ...props
    },
    ref
  ) => (
    <div
      className={cn(
        "relative",
        paddingVariants({ padding }),
        marginVariants({ margin }),
        colorVariants({ color }),
        cardVariants({ size, roundSize })
      )}
    >
      <div className="flex justify-between w-full items-stretch">
        <div className="flex-none">{icon}</div>
        <Box
          color={color}
          size={size}
          className={cn(
            "flex-1 pl-2 flex items-center",
            `border-t-${border}`,
            dashedBorder && "border-dashed",
            "border-b-1 border-b-gray-200 dark:border-b-gray-700",
            noShadow && "shadow-none"
          )}
        >
          {title}
        </Box>
        <div className="flex items-center justify-between">{action}</div>
      </div>
      <div
        ref={ref}
        className={cn(
          colorVariants({ color }),
          cardVariants({ roundSize: "none" }),
          noShadow && "shadow-none",
          noRadius ? "rounded-none" : "rounded-b-lg",
          `border-${border}`,
          "border-t-0",
          dashedBorder && "border-dashed",
          className
        )}
        {...props}
      >
        {children}
      </div>
    </div>
  )
)
Card.displayName = "Card"

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col space-y-1.5 p-4", className)}
    {...props}
  />
))
CardHeader.displayName = "CardHeader"

export interface CardActionProps extends React.HTMLAttributes<HTMLDivElement> {
  icon?: React.ReactNode
}

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
  <div ref={ref} className={cn("p-4 py-2", className)} {...props} />
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

export { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle }
