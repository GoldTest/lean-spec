import { useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { ChevronDown, ChevronUp, Pause, Play, Square } from 'lucide-react';
import { Badge, Button } from '@/library';
import { useCurrentProject } from '../../../hooks/useProjectQuery';
import { useSessions, useSessionMutations } from '../../../hooks/useSessionsQuery';
import type { Session } from '../../../types/api';
import { sessionStatusConfig } from '../../../lib/session-utils';
import { SessionCreateDialog } from '../session-create-dialog';
import { SessionLogsPanel } from '../session-logs-panel';

const BAR_HEIGHT_PX = 48;

function isActiveSession(session: Session): boolean {
  return session.status === 'running' || session.status === 'pending' || session.status === 'paused';
}

export function SessionsBottomBar() {
  const { t } = useTranslation('common');
  const navigate = useNavigate();
  const { currentProject } = useCurrentProject();
  const { data: sessions = [] } = useSessions(currentProject?.id ?? null);
  const { startSession, pauseSession, resumeSession, stopSession } = useSessionMutations(currentProject?.id ?? null);

  const [expanded, setExpanded] = useState(false);
  const [createOpen, setCreateOpen] = useState(false);
  const [activeLogSessionId, setActiveLogSessionId] = useState<string | null>(null);

  const activeSessions = useMemo(
    () => sessions.filter(isActiveSession).sort((a, b) => new Date(b.startedAt).getTime() - new Date(a.startedAt).getTime()),
    [sessions]
  );

  const runningCount = useMemo(() => activeSessions.filter((session) => session.status === 'running').length, [activeSessions]);
  const pendingCount = useMemo(() => activeSessions.filter((session) => session.status === 'pending').length, [activeSessions]);

  const openHub = () => {
    if (!currentProject?.id) return;
    navigate(`/projects/${currentProject.id}/sessions`);
  };

  const onToggleExpanded = () => {
    setExpanded((prev) => !prev);
    setActiveLogSessionId(null);
  };

  return (
    <>
      <div className="fixed bottom-0 left-0 right-0 z-40 border-t bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/85" style={{ height: BAR_HEIGHT_PX }}>
        <div className="mx-auto flex h-full w-full items-center justify-between gap-3 px-3">
          <div className="flex min-w-0 items-center gap-3 text-sm">
            <span className="inline-flex items-center gap-1.5 whitespace-nowrap">
              <span className="h-2 w-2 rounded-full bg-green-500" />
              {runningCount} {t('sessions.status.running')}
            </span>
            <span className="inline-flex items-center gap-1.5 whitespace-nowrap text-muted-foreground">
              <span className="h-2 w-2 rounded-full bg-amber-500" />
              {pendingCount} {t('sessions.status.pending')}
            </span>
          </div>

          <div className="flex items-center gap-2">
            <Button size="sm" variant="secondary" className="h-8" onClick={() => setCreateOpen(true)}>
              {t('sessions.actions.new')}
            </Button>
            <Button size="icon" variant="ghost" className="h-8 w-8" onClick={onToggleExpanded}>
              {expanded ? <ChevronDown className="h-4 w-4" /> : <ChevronUp className="h-4 w-4" />}
            </Button>
          </div>
        </div>
      </div>

      {expanded && (
        <div className="fixed bottom-12 left-0 right-0 z-40 border-t bg-background shadow-[0_-8px_24px_-16px_rgba(0,0,0,0.25)]">
          <div className="flex max-h-[50vh] min-h-[220px] w-full flex-col md:flex-row">
            <div className="w-full border-b p-3 md:w-[420px] md:border-b-0 md:border-r md:p-4">
              <div className="mb-3 flex items-center justify-between">
                <span className="text-sm font-semibold">{t('sessions.title')}</span>
                <Button size="sm" variant="link" className="h-7 px-0" onClick={openHub}>
                  {t('sessions.actions.viewAll')}
                </Button>
              </div>

              <div className="max-h-[34vh] space-y-2 overflow-y-auto pr-1">
                {activeSessions.length === 0 ? (
                  <div className="rounded-md border border-dashed p-3 text-xs text-muted-foreground">
                    {t('sessions.empty')}
                  </div>
                ) : (
                  activeSessions.map((session) => {
                    const statusMeta = sessionStatusConfig[session.status];
                    const StatusIcon = statusMeta.icon;
                    const selected = activeLogSessionId === session.id;

                    return (
                      <button
                        key={session.id}
                        type="button"
                        className={`w-full rounded-md border p-2 text-left transition-colors ${selected ? 'border-primary bg-primary/5' : 'hover:bg-muted/50'}`}
                        onClick={() => setActiveLogSessionId(session.id)}
                      >
                        <div className="mb-1 flex items-center justify-between gap-2">
                          <span className="truncate text-xs font-medium">{session.prompt?.trim() || session.id.slice(0, 8)}</span>
                          <Badge variant="outline" className={statusMeta.className}>
                            <StatusIcon className="mr-1 h-3 w-3" />
                            {t(`sessions.status.${session.status}`)}
                          </Badge>
                        </div>
                        <div className="mb-2 truncate text-[11px] text-muted-foreground">
                          {session.specIds.length > 0 ? session.specIds.join(', ') : t('sessions.select.empty')}
                        </div>
                        <div className="flex items-center gap-1">
                          {session.status === 'pending' && (
                            <Button size="sm" variant="secondary" className="h-6 px-2 text-xs" onClick={(event) => {
                              event.stopPropagation();
                              void startSession(session.id);
                            }}>
                              <Play className="mr-1 h-3 w-3" />
                              {t('sessions.actions.start')}
                            </Button>
                          )}
                          {session.status === 'running' && (
                            <>
                              <Button size="sm" variant="secondary" className="h-6 px-2 text-xs" onClick={(event) => {
                                event.stopPropagation();
                                void pauseSession(session.id);
                              }}>
                                <Pause className="mr-1 h-3 w-3" />
                                {t('sessions.actions.pause')}
                              </Button>
                              <Button size="sm" variant="destructive" className="h-6 px-2 text-xs" onClick={(event) => {
                                event.stopPropagation();
                                void stopSession(session.id);
                              }}>
                                <Square className="mr-1 h-3 w-3" />
                                {t('sessions.actions.stop')}
                              </Button>
                            </>
                          )}
                          {session.status === 'paused' && (
                            <>
                              <Button size="sm" variant="secondary" className="h-6 px-2 text-xs" onClick={(event) => {
                                event.stopPropagation();
                                void resumeSession(session.id);
                              }}>
                                <Play className="mr-1 h-3 w-3" />
                                {t('sessions.actions.resume')}
                              </Button>
                              <Button size="sm" variant="destructive" className="h-6 px-2 text-xs" onClick={(event) => {
                                event.stopPropagation();
                                void stopSession(session.id);
                              }}>
                                <Square className="mr-1 h-3 w-3" />
                                {t('sessions.actions.stop')}
                              </Button>
                            </>
                          )}
                        </div>
                      </button>
                    );
                  })
                )}
              </div>
            </div>

            <div className="min-h-[220px] flex-1 p-3 md:p-4">
              {activeLogSessionId ? (
                <SessionLogsPanel
                  sessionId={activeLogSessionId}
                  onBack={() => setActiveLogSessionId(null)}
                />
              ) : (
                <div className="flex h-full items-center justify-center rounded-md border border-dashed text-xs text-muted-foreground">
                  {t('sessions.labels.logs')}
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      <SessionCreateDialog
        open={createOpen}
        onOpenChange={setCreateOpen}
        projectPath={currentProject?.path}
      />
    </>
  );
}
