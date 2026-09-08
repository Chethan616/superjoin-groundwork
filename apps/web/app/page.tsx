export default function HomePage() {
  return (
    <main
      style={{
        minHeight: '100vh',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        gap: 12,
        textAlign: 'center',
        padding: 24,
      }}
    >
      <span
        style={{
          fontFamily: 'var(--font-body)',
          fontWeight: 700,
          fontSize: 11,
          letterSpacing: '.08em',
          textTransform: 'uppercase',
          color: 'var(--muted)',
        }}
      >
        Milestone 1 — skeleton
      </span>
      <h1
        style={{
          fontFamily: 'var(--font-display)',
          fontWeight: 600,
          fontSize: 'var(--fs-display-hero)',
          letterSpacing: 'var(--tracking-display-tight)',
          margin: 0,
          color: 'var(--ink)',
        }}
      >
        Groundwork
      </h1>
      <p style={{ color: 'var(--ink-2)', maxWidth: 460, margin: 0 }}>
        The chat workspace, document ingestion, and evidence viewer land in the milestones ahead.
        This page confirms the frontend, design tokens, and dark theme are wired up.
      </p>
    </main>
  );
}
