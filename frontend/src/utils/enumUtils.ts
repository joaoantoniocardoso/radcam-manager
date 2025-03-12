/**
 * Converts a numeric enum into an array of option objects,
 * each containing a numeric value and its corresponding enum key as text.
 * This works for TypeScript numeric enums due to the reverse mapping.
 */
export function enumToOptions<E extends Record<string, number | string>>(enumObj: E): { value: number; text: string }[] {
    return Object.values(enumObj)
        .filter((v): v is number => typeof v === 'number')
        .map((value) => ({
            value,
            text: formatEnumValue(enumObj[value as unknown as keyof E] as string),
        }))
}

export function formatEnumValue(value: string): string {
    return value
        .replace(/([A-Z])/g, " $1")
        .trim()
        .toLowerCase()
        .replace("bit rate", "bitrate")
        .replace(/^./, (match) => match.toUpperCase())
}