export type SessionConfig = { 
/**
 * Project path
 */
project_path: string, 
/**
 * Specs attached as context (zero or more)
 */
spec_ids: Array<string>, 
/**
 * Optional custom prompt/instructions for the session
 */
prompt: string | null, 
/**
 * AI runner to use
 */
runner: string, 
/**
 * Session mode
 */
mode: SessionMode, 
/**
 * Maximum iterations for Ralph mode
 */
max_iterations: number | null, 
/**
 * Working directory
 */
working_dir: string | null, 
/**
 * Environment variables
 */
env_vars: { [key in string]?: string }, 
/**
 * Additional arguments for the runner
 */
runner_args: Array<string>, };