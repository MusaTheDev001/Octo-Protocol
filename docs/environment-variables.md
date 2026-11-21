# Environment Variables

## Webhooks

| Variable | Values | Scope |
|---|---|---|
| `OCTO_ALLOW_LOCAL_WEBHOOKS` | `1` | Development and tests only. Allows webhook delivery to localhost and other otherwise-blocked local targets. |

Do not set `OCTO_ALLOW_LOCAL_WEBHOOKS=1` on mainnet. Server startup rejects this configuration when `NETWORK` is `mainnet` or `public`.
