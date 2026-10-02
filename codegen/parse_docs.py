"""docs.slack.dev の Markdown 版メソッドリファレンスから spec/methods.json を作る。

使い方:
  python3 codegen/parse_docs.py methods <メソッドの .md を置いたディレクトリ> > codegen/spec/methods.json
  python3 codegen/parse_docs.py objects <オブジェクトの .md を置いたディレクトリ> > codegen/spec/objects.json
.md は https://docs.slack.dev/reference/methods.md の一覧にある各 URL をそのまま保存したもの。
"""
import json
import os
import re
import sys

ARG_RE = re.compile(r"^\*\*`([A-Za-z0-9_]+)`\*\*(?:`([a-z]+)`)?(Required|Optional)\s*$")
EXAMPLE_RE = re.compile(r"^_Example:_\s*`(.*)`\s*$")
DEFAULT_RE = re.compile(r"^_Default:_\s*`(.*)`\s*$")


def frontmatter(text):
    m = re.match(r"^---\n(.*?)\n---\n", text, re.S)
    meta = {}
    for line in m.group(1).splitlines():
        kv = re.match(r'^([a-z_]+):\s*"?(.*?)"?\s*$', line)
        if kv:
            meta[kv.group(1)] = kv.group(2)
    scopes = {}
    for kind in ("bot", "user"):
        s = re.search(r"^\s+%s:\s*\[(.*?)\]" % kind, m.group(1), re.M)
        if s:
            scopes[kind] = re.findall(r'"([^"]+)"', s.group(1))
    meta["scopes"] = scopes
    return meta


def section(text, heading):
    m = re.search(r"^##{1,2} %s.*?$(.*?)(?=^## |\Z)" % re.escape(heading), text, re.S | re.M)
    return m.group(1) if m else ""


def parse_args(text):
    lines = section(text, "Arguments").splitlines()
    args = []
    cur = None
    for line in lines:
        m = ARG_RE.match(line)
        if m:
            cur = {
                "name": m.group(1),
                "type": m.group(2),
                "required": m.group(3) == "Required",
                "description": "",
                "example": None,
                "default": None,
            }
            args.append(cur)
            continue
        if cur is None or line.startswith("#"):
            continue
        e = EXAMPLE_RE.match(line)
        if e:
            cur["example"] = mask_tokens(e.group(1))
            continue
        d = DEFAULT_RE.match(line)
        if d:
            cur["default"] = d.group(1)
            continue
        if line.strip() and not cur["description"]:
            cur["description"] = line.strip()
    return [a for a in args if a["name"] != "token"]


def parse_json_loose(src):
    try:
        return json.loads(src)
    except ValueError:
        pass
    # ドキュメントの例の崩れ（末尾カンマ・省略記号 ...・シングルクォート・// コメント）を直してから読む
    fixed = re.sub(r"^\s*//.*$", "", src, flags=re.M)
    fixed = re.sub(r"(?<![\"\w])\.\.\.(?![\"\w])", "", fixed)
    fixed = re.sub(r"'([^'\"]*)'", r'"\1"', fixed)
    fixed = re.sub(r",\s*([}\]])", r"\1", fixed)
    # 閉じ括弧が欠けている例がある
    for tail in ("", "}", "}}"):
        try:
            return json.loads(fixed + tail)
        except ValueError:
            pass
    return None


def unwrap_example(v):
    # {"url": "", "description": "...", "example": {...}} の形で載っているページがある
    if isinstance(v, dict) and "example" in v and "description" in v:
        return v["example"]
    return v


# 応答例に載っている見本のトークン。本物ではないが、GitHub の push protection がシークレットとして弾くので置き換える
TOKEN_RE = re.compile(r"\b(xox[a-z])-[A-Za-z0-9-]+")
SALESFORCE_TOKEN_RE = re.compile(r"\b00D[A-Za-z0-9]{12}![A-Za-z0-9_.]+")


def mask_tokens(value):
    if isinstance(value, str):
        return SALESFORCE_TOKEN_RE.sub("00D-EXAMPLE-TOKEN", TOKEN_RE.sub(r"\1-EXAMPLE", value))
    if isinstance(value, list):
        return [mask_tokens(v) for v in value]
    if isinstance(value, dict):
        return {k: mask_tokens(v) for k, v in value.items()}
    return value


def parse_responses(text):
    body = section(text, "Response")
    out = []
    for m in re.finditer(r"```[a-z]*\n(.*?)```", body, re.S):
        v = unwrap_example(parse_json_loose(m.group(1)))
        if isinstance(v, dict) and v.get("ok") is True:
            out.append(mask_tokens(v))
    return out


def deprecation(text):
    head = re.search(r"^# .*?$(.*?)^## Facts", text, re.S | re.M)
    if not head:
        return None
    for para in head.group(1).split("\n\n"):
        if re.search(r"deprecat|sunset|retired", para, re.I):
            return " ".join(para.split())
    return None


def main(root):
    methods = []
    for name in sorted(os.listdir(root)):
        if not name.endswith(".md"):
            continue
        text = open(os.path.join(root, name), encoding="utf-8").read()
        meta = frontmatter(text)
        methods.append({
            "name": meta["method_name"],
            "summary": meta.get("summary", ""),
            "http_method": meta.get("http_method"),
            "rate_limit": meta.get("rate_limit"),
            "scopes": meta["scopes"],
            "deprecated": deprecation(text),
            "args": parse_args(text),
            "responses": parse_responses(text),
        })
    methods.sort(key=lambda m: m["name"])
    json.dump(methods, sys.stdout, ensure_ascii=False, indent=1)
    sys.stdout.write("\n")




# オブジェクトのページ（reference/objects/*.md）→ 型名ごとの JSON 例
OBJECT_PAGES = {
    "channel-object.md": ("Conversation", "channel"),
    "conversation-object.md": ("Conversation", "channel"),
    "group-object.md": ("Conversation", None),
    "im-object.md": ("Conversation", None),
    "mpim-object.md": ("Conversation", None),
    "file-object.md": ("File", None),
    "user-object.md": ("User", "user"),
    "usergroup-object.md": ("Usergroup", None),
}


def objects_main(root):
    out = {}
    for page, (type_name, wrapper) in sorted(OBJECT_PAGES.items()):
        text = open(os.path.join(root, page), encoding="utf-8").read()
        for m in re.finditer(r"```[a-z]*\n(.*?)```", text, re.S):
            v = unwrap_example(parse_json_loose(m.group(1)))
            if not isinstance(v, dict):
                continue
            if wrapper and wrapper in v:
                v = v[wrapper]
            # file-object の2つ目の例は {"files": [...]} の形
            samples = v.get("files") if type_name == "File" and "files" in v and "id" not in v else [v]
            out.setdefault(type_name, []).extend(samples)
    json.dump(out, sys.stdout, ensure_ascii=False, indent=1)
    sys.stdout.write("\n")


if __name__ == "__main__":
    {"methods": main, "objects": objects_main}[sys.argv[1]](sys.argv[2])
