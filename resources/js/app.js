import './bootstrap'

import * as Sentry from "@sentry/browser"

Sentry.init({
    dsn: import.meta.env.VITE_SENTRY_DSN,
    environment: import.meta.env.VITE_SENTRY_ENVIRONMENT,
    release: import.meta.env.VITE_SENTRY_RELEASE,
    integrations: [
        new Sentry.BrowserTracing()
    ],
    tracesSampleRate: import.meta.env.VITE_SENTRY_TRACES_SAMPLE_RATE,
})
