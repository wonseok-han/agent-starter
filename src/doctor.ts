import type { MessageKey } from "./locales/ko";

export interface Diagnosis {
  titleKey: MessageKey;
  adviceKey: MessageKey;
}

// 백엔드가 Err(AppError)로 거절하면 invoke는 { kind, detail } 객체로 reject된다.
export interface AppError {
  kind: string;
  detail: string;
}

const KNOWN_KINDS = [
  "network",
  "checksum",
  "notfound",
  "permission",
  "disk",
  "generic",
];

// 백엔드 kind는 kebab-case(not-found), 프론트 키는 notfound로 정규화
function normalizeKind(kind: string): string {
  return kind.replace(/-/g, "");
}

/** invoke 실패값을 구조화된 AppError로 정규화한다(문자열/객체 모두 수용). */
export function toAppError(e: unknown): AppError {
  if (e && typeof e === "object" && "detail" in e) {
    const obj = e as { kind?: string; detail?: string };
    return {
      kind: normalizeKind(obj.kind ?? "generic"),
      detail: String(obj.detail ?? ""),
    };
  }
  return { kind: "generic", detail: String(e) };
}

/**
 * 에러를 진단으로 바꾼다. 백엔드 kind가 구체적이면 그대로 쓰고(정확),
 * generic이면 detail·로그를 정규식으로 재분류한다(보완).
 */
export function diagnoseError(err: AppError, log?: string[]): Diagnosis {
  const kind =
    err.kind && err.kind !== "generic" && KNOWN_KINDS.includes(err.kind)
      ? err.kind
      : diagnoseKind(err.detail, log);
  return {
    titleKey: `doctor.${kind}.title` as MessageKey,
    adviceKey: `doctor.${kind}.advice` as MessageKey,
  };
}

function diagnoseKind(...sources: Source[]): string {
  const text = flatten(sources);
  for (const p of PATTERNS) {
    if (p.match.test(text)) return p.kind;
  }
  return "generic";
}

// 에러 메시지·로그에서 흔한 실패 원인을 짚어 분류한다(백엔드 kind가 generic일 때 보완).
// 위에서부터 먼저 맞는 패턴을 쓴다 (구체적인 것 먼저).
const PATTERNS: { match: RegExp; kind: string }[] = [
  {
    match: /could not connect|connection refused|network|timed? ?out|resolve host|getaddrinfo|dns|offline/i,
    kind: "network",
  },
  { match: /checksum|verification failed|corrupt/i, kind: "checksum" },
  { match: /command not found|not recognized|no such file|enoent/i, kind: "notfound" },
  {
    match: /permission denied|eacces|not permitted|operation not permitted/i,
    kind: "permission",
  },
  { match: /no space|enospc|disk (is )?full/i, kind: "disk" },
];

type Source = string | string[] | null | undefined;

function flatten(sources: Source[]): string {
  return sources
    .flatMap((s) => (Array.isArray(s) ? s : [s]))
    .filter((s): s is string => Boolean(s))
    .join("\n");
}
