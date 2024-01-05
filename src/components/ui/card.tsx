import * as React from "react"
import { VariantProps, cva } from "class-variance-authority"
import { cn } from "lib/utils"
import {
  BgColorVariant,
  MarginVariant,
  PaddingVariant,
  bgColorVariants,
  marginVariants,
  paddingVariants
} from "lib/variants"
import { Icon, IconProps } from "./icon"
import { Box } from "./box"

const cardVariants = cva(
  "bg-card transition-card-height transition-border-radius shadow",
  {
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
  }
)

export interface CardProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof cardVariants>,
    VariantProps<BgColorVariant>,
    VariantProps<PaddingVariant>,
    VariantProps<MarginVariant> {
  dashedBorder?: boolean
  icon?: React.ReactNode
  asChild?: boolean
  noRadius?: boolean
  noShadow?: boolean
  border?: 0 | 1 | 2 | 3 | 4
  action?: React.ReactNode
  iconProps?: IconProps
}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      size,
      noShadow,
      noRadius,
      bgColor: color,
      border = 0,
      dashedBorder,
      iconProps,
      padding,
      paddingBottom,
      paddingLeft,
      paddingRight,
      paddingTop,
      margin,
      marginBottom,
      marginLeft,
      marginRight,
      marginTop,
      icon,
      title,
      roundSize,
      action,
      children,
      ...props
    },
    ref
  ) => {
    border = size === "xl" ? 3 : border

    return (
      <div
        className={cn(
          paddingVariants({
            padding,
            paddingBottom,
            paddingLeft,
            paddingRight,
            paddingTop
          }),
          marginVariants({
            margin,
            marginBottom,
            marginLeft,
            marginRight,
            marginTop
          }),
          bgColorVariants()({ bgColor: color }),
          cardVariants({ size, roundSize })
        )}
      >
        <div className="flex justify-between w-full items-stretch">
          {icon && (
            <Icon
              {...iconProps}
              className={cn(
                "flex-none flex items-center justify-center",
                `border-${border}`,
                dashedBorder ? "border-dashed" : "shadow-button-inner"
              )}
            >
              {icon}
            </Icon>
          )}
          <Box
            bgColor={color}
            size={size}
            className={cn(
              "flex-1 pl-2 flex items-center font-semibold leading-none tracking-tight",
              dashedBorder && "border-dashed",
              `border-t-${border}`
            )}
          >
            {title}
          </Box>
          {action}
        </div>
        <div
          ref={ref}
          className={cn(
            bgColorVariants()({ bgColor: color }),
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
  }
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
  // dont allow to words overflow the card
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
