/**
 * data model of our server responses
 */

export type InstancePayload = {
    args: string[],
    cwd: string,
}

// pty server response
export type PtyPayload = {
    res: string,
    fd: number,
    status: number
}