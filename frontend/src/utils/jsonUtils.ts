export function applyNonNull<T extends object>(target: T, incoming: Partial<T>): void {
    for (const key of Object.keys(incoming) as (keyof T)[]) {
        const value = incoming[key]
        if (value !== null) {
            target[key] = value as T[keyof T]
        }
    }
}