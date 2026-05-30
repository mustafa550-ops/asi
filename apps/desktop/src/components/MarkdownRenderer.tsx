import { useMemo } from "react";

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function renderInline(text: string): string {
  return text
    .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
    .replace(/\*(.+?)\*/g, "<em>$1</em>")
    .replace(/`(.+?)`/g, "<code>$1</code>")
    .replace(/\[(.+?)\]\((.+?)\)/g, '<a href="$2" target="_blank">$1</a>');
}

function renderMarkdown(md: string): string {
  const lines = md.split("\n");
  const html: string[] = [];
  let inCode = false;
  let codeLang = "";
  let codeBuf: string[] = [];

  for (const line of lines) {
    if (line.startsWith("```")) {
      if (inCode) {
        html.push(
          `<pre class="md-code-block"><code class="md-code lang-${escapeHtml(codeLang)}">${escapeHtml(codeBuf.join("\n"))}</code></pre>`,
        );
        codeBuf = [];
        inCode = false;
        codeLang = "";
      } else {
        inCode = true;
        codeLang = line.slice(3).trim();
      }
      continue;
    }
    if (inCode) {
      codeBuf.push(line);
      continue;
    }
    if (line.startsWith("### ")) {
      html.push(
        `<h3 class="md-h3">${renderInline(escapeHtml(line.slice(4)))}</h3>`,
      );
    } else if (line.startsWith("## ")) {
      html.push(
        `<h2 class="md-h2">${renderInline(escapeHtml(line.slice(3)))}</h2>`,
      );
    } else if (line.startsWith("# ")) {
      html.push(
        `<h1 class="md-h1">${renderInline(escapeHtml(line.slice(2)))}</h1>`,
      );
    } else if (line.startsWith("- ") || line.startsWith("* ")) {
      html.push(
        `<li class="md-li">${renderInline(escapeHtml(line.slice(2)))}</li>`,
      );
    } else if (/^\d+\.\s/.test(line)) {
      html.push(
        `<li class="md-li">${renderInline(escapeHtml(line.replace(/^\d+\.\s/, "")))}</li>`,
      );
    } else if (line.trim() === "") {
      html.push("</ul><br/>");
    } else {
      html.push(`<p class="md-p">${renderInline(escapeHtml(line))}</p>`);
    }
  }
  if (inCode) {
    html.push(
      `<pre class="md-code-block"><code>${escapeHtml(codeBuf.join("\n"))}</code></pre>`,
    );
  }
  return html.join("\n");
}

interface MarkdownRendererProps {
  content: string;
}

export function MarkdownRenderer({ content }: MarkdownRendererProps) {
  const html = useMemo(() => renderMarkdown(content), [content]);
  return (
    <div className="md-render" dangerouslySetInnerHTML={{ __html: html }} />
  );
}
