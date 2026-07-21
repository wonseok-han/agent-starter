const steps = [
  {
    number: "01",
    title: "에이전트 선택",
    description: "Claude Code와 Codex 중 지금 쓰는 구독에 맞춰 골라요.",
  },
  {
    number: "02",
    title: "컴퓨터 진단",
    description: "설치 상태와 운영체제를 안전하게 확인해요. 파일은 바꾸지 않아요.",
  },
  {
    number: "03",
    title: "자동 설치",
    description: "공식 설치 파일을 받고 복잡한 터미널 설정까지 대신 정리해요.",
  },
  {
    number: "04",
    title: "브라우저 로그인",
    description: "비밀번호는 공식 사이트에만 입력하고, 완료 여부는 앱이 확인해요.",
  },
  {
    number: "05",
    title: "첫 프로젝트",
    description: "다른 파일과 분리된 안전한 작업 폴더를 만들어요.",
  },
  {
    number: "06",
    title: "첫 대화",
    description: "에이전트의 첫 답장을 확인하면 모든 준비가 끝나요.",
  },
] as const;

const faqs = [
  {
    question: "코딩을 전혀 몰라도 쓸 수 있나요?",
    answer:
      "네. 터미널이나 명령어를 몰라도 괜찮아요. 화면에 나오는 쉬운 한국어 안내를 따라 버튼만 누르면 됩니다.",
  },
  {
    question: "어떤 계정이 필요한가요?",
    answer:
      "Claude Code는 Claude 구독을, Codex는 ChatGPT 계정을 사용할 수 있어요. 로그인 단계에서 내 상황에 맞는 방식을 설명해 드립니다.",
  },
  {
    question: "이미 설치한 사람도 쓸 수 있나요?",
    answer:
      "물론이에요. 먼저 컴퓨터를 확인한 뒤 이미 끝난 단계는 건너뛰고 필요한 설정만 도와드려요.",
  },
  {
    question: "내 비밀번호나 대화가 앱에 저장되나요?",
    answer:
      "아니요. 로그인은 Claude와 ChatGPT의 공식 화면에서 진행하며 Hello, Agent는 비밀번호나 인증 토큰을 저장하지 않아요.",
  },
] as const;

function BrandMark() {
  return (
    <span className="brand-mark" aria-hidden="true">
      <span />
      <span />
    </span>
  );
}

function ProductPreview() {
  return (
    <div className="product-preview" aria-label="Hello, Agent 앱 화면 미리보기">
      <div className="window-bar">
        <div className="window-dots" aria-hidden="true">
          <i />
          <i />
          <i />
        </div>
        <span>Hello, Agent</span>
        <span className="window-spacer" />
      </div>
      <div className="preview-body">
        <p className="preview-eyebrow">코딩 에이전트 시작 도우미</p>
        <h2>어떤 코딩 에이전트를 시작할까요?</h2>
        <p>이미 쓰고 있는 구독에 맞춰 고르면 돼요.</p>
        <div className="preview-progress" aria-hidden="true">
          {["에이전트", "진단", "설치", "로그인", "프로젝트", "완료"].map(
            (label, index) => (
              <div className={index === 0 ? "current" : ""} key={label}>
                <span>{index + 1}</span>
                <small>{label}</small>
              </div>
            ),
          )}
        </div>
        <div className="agent-options">
          <div className="agent-option recommended">
            <span className="option-badge">추천</span>
            <strong>클로드 코드</strong>
            <small>Anthropic</small>
            <p>Claude 구독으로 시작해요.</p>
          </div>
          <div className="agent-option">
            <strong>코덱스</strong>
            <small>OpenAI</small>
            <p>ChatGPT 계정으로 시작해요.</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default function Home() {
  return (
    <main>
      <header className="site-header">
        <a className="brand" href="#top" aria-label="Hello, Agent 처음으로">
          <BrandMark />
          <span>Hello, Agent</span>
        </a>
        <nav aria-label="주요 메뉴">
          <a href="#how">어떻게 작동하나요?</a>
          <a href="#safety">안전한가요?</a>
          <a href="#faq">자주 묻는 질문</a>
        </nav>
        <a className="header-cta" href="#download">
          출시 소식 보기
        </a>
      </header>

      <section className="hero" id="top">
        <div className="hero-copy">
          <p className="section-kicker">
            <span className="status-dot" /> macOS · Windows 베타 준비 중
          </p>
          <h1>
            터미널을 몰라도,
            <br />
            <em>코딩 에이전트</em>는 쓸 수 있어요.
          </h1>
          <p className="hero-description">
            설치부터 로그인, 안전한 첫 프로젝트까지. Hello, Agent가 어려운
            설정을 대신하고 첫 대화까지 차근차근 안내합니다.
          </p>
          <div className="hero-actions">
            <a className="button primary" href="#download">
              베타 출시 소식 받기
              <span aria-hidden="true">→</span>
            </a>
            <a
              className="button secondary"
              href="https://github.com/wonseok-han/hello-agent"
              target="_blank"
              rel="noreferrer"
            >
              GitHub에서 보기
            </a>
          </div>
          <p className="hero-note">무료 오픈소스 · 한국어 지원 · 터미널 노출 없음</p>
        </div>
        <div className="hero-visual">
          <div className="coral-orbit orbit-one" aria-hidden="true" />
          <div className="coral-orbit orbit-two" aria-hidden="true" />
          <ProductPreview />
          <div className="trust-chip chip-left">
            <span>✓</span>
            공식 설치 파일만 사용
          </div>
          <div className="trust-chip chip-right">
            <span>✓</span>
            비밀번호 저장 안 함
          </div>
        </div>
      </section>

      <section className="problem-strip" aria-label="Hello, Agent가 해결하는 문제">
        <p>명령어 복사</p>
        <span aria-hidden="true">×</span>
        <p>PATH 설정</p>
        <span aria-hidden="true">×</span>
        <p>검은 터미널 창</p>
        <span aria-hidden="true">×</span>
        <p>혼자 해결하는 오류</p>
      </section>

      <section className="section how-section" id="how">
        <div className="section-heading">
          <p className="section-kicker">버튼을 따라가는 6단계</p>
          <h2>첫 대화까지, 길을 잃지 않도록</h2>
          <p>
            지금 무엇을 하고 있는지 쉬운 말로 보여주고, 이미 끝난 일은 알아서
            건너뜁니다.
          </p>
        </div>
        <ol className="steps-grid">
          {steps.map((step) => (
            <li key={step.number}>
              <span className="step-number">{step.number}</span>
              <div className="step-icon" aria-hidden="true">
                {step.number === "01"
                  ? "◎"
                  : step.number === "02"
                    ? "⌁"
                    : step.number === "03"
                      ? "↓"
                      : step.number === "04"
                        ? "↗"
                        : step.number === "05"
                          ? "□"
                          : "✓"}
              </div>
              <h3>{step.title}</h3>
              <p>{step.description}</p>
            </li>
          ))}
        </ol>
      </section>

      <section className="section safety-section" id="safety">
        <div className="safety-card">
          <div className="safety-copy">
            <p className="section-kicker light">처음이니까 더 조심스럽게</p>
            <h2>내 파일과 계정을 먼저 생각했어요.</h2>
            <p>
              코딩 에이전트는 강력한 도구입니다. 그래서 Hello, Agent는 처음부터
              안전한 범위 안에서 시작하도록 기본 설정을 챙깁니다.
            </p>
            <ul>
              <li>
                <span>01</span>
                다른 문서와 분리된 전용 프로젝트 폴더
              </li>
              <li>
                <span>02</span>
                위험한 작업은 실행 전에 꼭 확인
              </li>
              <li>
                <span>03</span>
                비밀번호와 인증 토큰은 앱에 저장하지 않음
              </li>
              <li>
                <span>04</span>
                Claude와 OpenAI의 공식 설치 경로만 사용
              </li>
            </ul>
          </div>
          <div className="safety-visual" aria-hidden="true">
            <div className="folder-back" />
            <div className="folder-front">
              <span className="shield">✓</span>
              <strong>내 첫 프로젝트</strong>
              <small>안전 설정 적용됨</small>
            </div>
            <div className="safe-file file-one">AGENTS.md</div>
            <div className="safe-file file-two">CLAUDE.md</div>
          </div>
        </div>
      </section>

      <section className="section agents-section">
        <div className="section-heading compact">
          <p className="section-kicker">내가 이미 쓰는 계정으로</p>
          <h2>Claude Code도, Codex도</h2>
        </div>
        <div className="agent-showcase">
          <article>
            <div className="agent-letter claude">C</div>
            <div>
              <p className="agent-company">ANTHROPIC</p>
              <h3>Claude Code</h3>
              <p>Claude Pro·Max 등 구독 계정으로 시작할 수 있어요.</p>
            </div>
          </article>
          <article>
            <div className="agent-letter codex">O</div>
            <div>
              <p className="agent-company">OPENAI</p>
              <h3>Codex</h3>
              <p>사용 중인 ChatGPT 계정으로 자연스럽게 이어져요.</p>
            </div>
          </article>
        </div>
      </section>

      <section className="section faq-section" id="faq">
        <div className="section-heading compact">
          <p className="section-kicker">궁금한 점부터 천천히</p>
          <h2>자주 묻는 질문</h2>
        </div>
        <div className="faq-list">
          {faqs.map((faq, index) => (
            <details key={faq.question} open={index === 0}>
              <summary>
                {faq.question}
                <span aria-hidden="true">+</span>
              </summary>
              <p>{faq.answer}</p>
            </details>
          ))}
        </div>
      </section>

      <section className="download-section" id="download">
        <div>
          <p className="section-kicker light">Hello, Agent 0.1</p>
          <h2>첫 번째 베타를 준비하고 있어요.</h2>
          <p>
            macOS와 Windows에서 안심하고 시작할 수 있도록 마지막 확인을 하고
            있습니다. GitHub에서 개발 소식을 가장 먼저 만나보세요.
          </p>
        </div>
        <a
          className="button download-button"
          href="https://github.com/wonseok-han/hello-agent"
          target="_blank"
          rel="noreferrer"
        >
          GitHub에서 진행 상황 보기
          <span aria-hidden="true">↗</span>
        </a>
      </section>

      <footer>
        <a className="brand footer-brand" href="#top">
          <BrandMark />
          <span>Hello, Agent</span>
        </a>
        <p>코딩 에이전트와의 첫 만남을 더 쉽게.</p>
        <a
          href="https://github.com/wonseok-han/hello-agent"
          target="_blank"
          rel="noreferrer"
        >
          GitHub
        </a>
      </footer>
    </main>
  );
}
