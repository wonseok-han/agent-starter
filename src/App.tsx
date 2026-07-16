import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface ToolInfo {
  path: string;
  version: string;
  inShellPath: boolean;
}

interface EnvironmentReport {
  os: string;
  arch: string;
  osVersion: string | null;
  claude: ToolInfo | null;
  node: ToolInfo | null;
  checkedPaths: string[];
}

const STEPS = ["진단", "설치", "로그인", "첫 프로젝트", "졸업식"] as const;

function osLabel(report: EnvironmentReport): string {
  if (report.os === "macos") {
    const chip = report.arch === "aarch64" ? "Apple Silicon" : "Intel";
    const ver = report.osVersion ? ` ${report.osVersion}` : "";
    return `macOS${ver} (${chip})`;
  }
  if (report.os === "windows") return "Windows";
  return `${report.os} (${report.arch})`;
}

function App() {
  const [step, setStep] = useState(0);
  const [report, setReport] = useState<EnvironmentReport | null>(null);

  return (
    <div className="app">
      <header className="header">
        <h1>Agent Starter</h1>
        <p>클로드 코드를 쓸 수 있는 상태까지, 차근차근 같이 갈게요.</p>
      </header>

      <ol className="steps">
        {STEPS.map((name, i) => (
          <li
            key={name}
            className={i === step ? "active" : i < step ? "done" : ""}
          >
            <span className="step-dot">{i < step ? "✓" : i + 1}</span>
            {name}
          </li>
        ))}
      </ol>

      <main className="panel">
        {step === 0 ? (
          <DiagnosisStep
            report={report}
            onReport={setReport}
            onNext={() => setStep(1)}
          />
        ) : (
          <PlaceholderStep name={STEPS[step]} onBack={() => setStep(0)} />
        )}
      </main>
    </div>
  );
}

function DiagnosisStep({
  report,
  onReport,
  onNext,
}: {
  report: EnvironmentReport | null;
  onReport: (r: EnvironmentReport) => void;
  onNext: () => void;
}) {
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function run() {
    setRunning(true);
    setError(null);
    try {
      onReport(await invoke<EnvironmentReport>("detect_environment"));
    } catch (e) {
      setError(String(e));
    } finally {
      setRunning(false);
    }
  }

  if (!report) {
    return (
      <div className="center">
        <h2>먼저 내 컴퓨터 상태를 확인할게요</h2>
        <p className="muted">
          어떤 컴퓨터인지, 클로드 코드가 이미 설치되어 있는지 살펴봐요.
          <br />
          컴퓨터의 파일을 바꾸지 않으니 안심하세요.
        </p>
        {error && <p className="error">확인 중 문제가 생겼어요: {error}</p>}
        <button className="primary" onClick={run} disabled={running}>
          {running ? "확인하는 중…" : "내 컴퓨터 확인하기"}
        </button>
      </div>
    );
  }

  const claude = report.claude;
  return (
    <div>
      <h2>확인이 끝났어요</h2>
      <ul className="results">
        <li className="result ok">
          <span className="result-icon">💻</span>
          <div>
            <strong>{osLabel(report)}</strong>
            <div className="muted">이 컴퓨터에서 사용할 수 있어요.</div>
          </div>
        </li>
        <li className={`result ${claude ? "ok" : "todo"}`}>
          <span className="result-icon">{claude ? "✅" : "📦"}</span>
          <div>
            <strong>
              {claude
                ? `클로드 코드가 이미 설치되어 있어요 (버전 ${claude.version.split(" ")[0]})`
                : "클로드 코드가 아직 없어요"}
            </strong>
            <div className="muted">
              {claude
                ? claude.inShellPath
                  ? "터미널에서도 바로 쓸 수 있는 상태예요."
                  : "설치는 되어 있지만 터미널이 아직 위치를 몰라요. 나중에 자동으로 잡아드릴게요."
                : "다음 단계에서 자동으로 설치해 드릴게요."}
            </div>
          </div>
        </li>
        <li className="result ok">
          <span className="result-icon">{report.node ? "🟢" : "⚪️"}</span>
          <div>
            <strong>
              Node.js {report.node ? `${report.node.version} 있음` : "없음"}
            </strong>
            <div className="muted">
              {report.node
                ? "부가 도구를 쓸 때 도움이 돼요."
                : "없어도 클로드 코드 사용에는 문제 없어요."}
            </div>
          </div>
        </li>
      </ul>
      <div className="actions">
        <button className="ghost" onClick={run} disabled={running}>
          다시 확인
        </button>
        <button className="primary" onClick={onNext}>
          {claude ? "설치는 건너뛰고 다음으로" : "다음: 설치하러 가기"}
        </button>
      </div>
    </div>
  );
}

function PlaceholderStep({
  name,
  onBack,
}: {
  name: string;
  onBack: () => void;
}) {
  return (
    <div className="center">
      <h2>{name} 단계는 준비 중이에요 🚧</h2>
      <p className="muted">
        M0 기술 검증이 진행되는 동안 순서대로 열릴 예정이에요.
      </p>
      <button className="ghost" onClick={onBack}>
        진단으로 돌아가기
      </button>
    </div>
  );
}

export default App;
