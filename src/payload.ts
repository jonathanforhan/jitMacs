/**
 * data model of our server responses
 */

// pty server response
export type PtyPayload = {
    res: string,
    fd: number,
    status: number
}