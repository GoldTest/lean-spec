export type RunnerDefinition = { id: string, name: string | null, command: string | null, args: Array<string>, env: { [key in string]?: string }, detection: DetectionConfig | null, symlink_file: string | null, 
/**
 * Controls how the session prompt is passed to the runner CLI.
 * - `Some(flag)` — prepend `flag` before the prompt value (e.g. `"--print"`).
 * - `Some("-")` — suppress prompt injection (runner doesn't accept a prompt arg).
 * - `None` — append the prompt as a positional argument.
 */
prompt_flag: string | null, };