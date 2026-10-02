"""codegen/spec/*.json から src/api/*.rs・src/objects.rs・tests/generated_*.rs を生成する。

使い方: python3 codegen/generate.py && cargo fmt

型の決め方
- リクエスト: ドキュメントの引数の型（string / boolean / integer / number / array / object）。
  blocks・attachments・view・metadata は Block Kit の型。配列は例が `[...]` なら JSON、それ以外はカンマ区切りで送る
- レスポンス: ドキュメントの成功例を全部重ねて推定する。Slack の主要オブジェクト（Message・Conversation・
  User・File …）は、どのメソッドの例に出てきたものも1つの型に寄せる（CORE_TYPES）。
  項目は全部省略可能（Option / 空 Vec）。スカラーは crate::de のゆるい読み取りを通す
"""
import json
import keyword
import os
import re
import sys
from collections import OrderedDict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC = os.path.join(ROOT, "codegen", "spec")

# 動かなくなったメソッド（2025-11-12 に停止）。files.getUploadURLExternal + files.completeUploadExternal を使う
SKIP_METHODS = {"files.upload"}
# JSON ではなくファイルそのもの（gzip）を返す
BYTES_METHODS = {"admin.analytics.getFile"}

RUST_KEYWORDS = set(
    "as break const continue crate else enum extern false fn for if impl in let loop match mod move mut "
    "pub ref return self Self static struct super trait true type unsafe use where while async await dyn "
    "abstract become box do final macro override priv typeof unsized virtual yield try gen".split()
)
NOT_RAW = {"self", "Self", "super", "crate"}

# ---------------------------------------------------------------- 名前


def snake(name):
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = re.sub(r"[^A-Za-z0-9]+", "_", s).strip("_").lower()
    s = re.sub(r"_+", "_", s)
    if not s or s[0].isdigit():
        s = "_" + s
    return s


def pascal(name):
    return "".join(p[:1].upper() + p[1:] for p in snake(name).split("_") if p)


def method_snake(method):
    return "_".join(snake(p) for p in method.split("."))


def method_pascal(method):
    return "".join(pascal(p) for p in method.split("."))


def field_ident(key):
    """JSON キー → (Rust のフィールド名, rename が要るか)"""
    if key.startswith("https://slack.com/"):
        base = "slack_" + snake(key[len("https://slack.com/"):])
    else:
        base = snake(key)
    if base in NOT_RAW:
        base = base + "_"
    elif base in RUST_KEYWORDS:
        return "r#" + base, base != key
    return base, base != key


def doc_url(method):
    return "https://docs.slack.dev/reference/methods/" + method


def doc_line(text):
    return " ".join(text.split()).replace("<", "&lt;").replace(">", "&gt;")


# ---------------------------------------------------------------- 主要オブジェクト

def has(obj, *keys):
    return all(k in obj for k in keys)


def any_of(obj, *keys):
    return any(k in obj for k in keys)


# 型名 → (寄せるキー, その形か判定する関数)
CORE_TYPES = OrderedDict([
    ("Message", ({"message", "messages", "latest", "root", "previous_message", "thread_message"},
                 lambda o: any_of(o, "ts", "thread_ts") and any_of(o, "text", "type", "user", "blocks", "bot_id"))),
    ("Conversation", ({"channel", "channels", "conversation", "conversations", "group", "groups", "im", "ims",
                       "mpim", "mpims"},
                      lambda o: "id" in o and any_of(o, "is_channel", "is_im", "is_private", "is_group", "is_mpim",
                                                     "is_archived", "name_normalized"))),
    ("User", ({"user", "users", "members", "inviting_user", "accepting_user", "requester", "owner"},
              lambda o: "id" in o and any_of(o, "profile", "real_name", "is_bot", "tz", "is_admin", "deleted"))),
    ("UserProfile", ({"profile"},
                     lambda o: any_of(o, "display_name", "real_name", "avatar_hash", "image_24", "email",
                                      "status_text"))),
    ("File", ({"file", "files"}, lambda o: "id" in o and any_of(o, "mimetype", "filetype", "url_private", "pretty_type"))),
    ("Usergroup", ({"usergroup", "usergroups", "subteam"}, lambda o: "id" in o and "handle" in o)),
    ("Team", ({"team", "teams", "inviting_team", "accepting_team", "reviewing_team", "connected_teams"},
              lambda o: "id" in o and any_of(o, "domain", "icon", "email_domain", "is_verified"))),
    ("Bot", ({"bot", "bots", "bot_profile"}, lambda o: "id" in o and any_of(o, "app_id", "icons", "deleted") and "profile" not in o)),
    ("Reminder", ({"reminder", "reminders"}, lambda o: "id" in o and any_of(o, "recurring", "complete_ts"))),
    ("Bookmark", ({"bookmark", "bookmarks"}, lambda o: "id" in o and any_of(o, "link", "channel_id") and "title" in o)),
    ("ViewInfo", ({"view"}, lambda o: "id" in o and "type" in o and any_of(o, "blocks", "callback_id", "hash"))),
    ("Reaction", ({"reactions"}, lambda o: has(o, "name") and any_of(o, "count", "users"))),
    ("MessageAttachment", ({"attachments"}, lambda o: True)),
    ("Topic", ({"topic", "purpose"}, lambda o: has(o, "value"))),
    ("Icons", ({"icons", "icon"}, lambda o: any(k.startswith("image_") or k.startswith("emoji") for k in o))),
    ("Canvas", ({"canvas"}, lambda o: any_of(o, "file_id", "is_empty"))),
    ("Enterprise", ({"enterprise"}, lambda o: "id" in o and "name" in o)),
])
CORE_DOCS = {
    "Message": "A message (<https://docs.slack.dev/messaging/message-structure>).",
    "Conversation": "A channel, DM or multi-person DM (<https://docs.slack.dev/reference/objects/conversation-object>).",
    "User": "A user (<https://docs.slack.dev/reference/objects/user-object>).",
    "UserProfile": "A user's profile.",
    "File": "A file (<https://docs.slack.dev/reference/objects/file-object>).",
    "Usergroup": "A user group (<https://docs.slack.dev/reference/objects/usergroup-object>).",
    "Team": "A workspace.",
    "Bot": "A bot.",
    "Reminder": "A reminder.",
    "Bookmark": "A channel bookmark.",
    "ViewInfo": "A view as returned by Slack. To send a view, use [`crate::blocks::View`].",
    "Reaction": "An emoji reaction.",
    "MessageAttachment": "A legacy attachment on a received message. To send one, use [`crate::blocks::Attachment`].",
    "Topic": "A channel topic or purpose.",
    "Icons": "Icon image URLs.",
    "Canvas": "A canvas.",
    "Enterprise": "An Enterprise Grid organization.",
}
KEY_TO_CORE = {}
for _name, (_keys, _) in CORE_TYPES.items():
    for _k in _keys:
        KEY_TO_CORE.setdefault(_k, []).append(_name)

# 主要オブジェクトと同じキーでも形が違う（purpose・icon が文字列）ので専用の型にするもの: (応答の型名, キー)
CORE_EXCLUDE = {
    ("AdminConversationsSearchResponse", "conversations"),
    ("AdminTeamsSettingsInfoResponse", "team"),
}

ID_KEY = re.compile(r"^[A-Z][A-Za-z0-9]{5,}(__[a-z_]+)?$")


def core_for(key, obj):
    for name in KEY_TO_CORE.get(key, []):
        if CORE_TYPES[name][1](obj):
            return name
    return None


def is_id_map(obj):
    return bool(obj) and all(ID_KEY.match(k) for k in obj)


# ---------------------------------------------------------------- 型の推定

class Registry:
    def __init__(self):
        self.core_samples = OrderedDict((n, []) for n in CORE_TYPES)
        self.structs = OrderedDict()  # name -> (doc, fields)
        self.names = set()
        self.aliases = {}  # 型名 -> docs.rs の検索で引けるようにする Slack のメソッド名

    def collect_core(self, value, key=None, owner=None):
        """例の中の主要オブジェクトを型ごとに集める（中にある主要オブジェクトも再帰的に）"""
        if isinstance(value, dict):
            core = core_for(key, value) if key and (owner, key) not in CORE_EXCLUDE else None
            if core:
                self.core_samples[core].append(value)
            for k, v in value.items():
                self.collect_core(v, k, owner if key is None else None)
        elif isinstance(value, list):
            for v in value:
                self.collect_core(v, key, owner)


def scalar_kinds(values):
    kinds = set()
    for v in values:
        if isinstance(v, bool):
            kinds.add("bool")
        elif isinstance(v, int):
            kinds.add("int")
        elif isinstance(v, float):
            kinds.add("float")
        elif isinstance(v, str):
            kinds.add("str")
        elif isinstance(v, dict):
            kinds.add("obj")
        elif isinstance(v, list):
            kinds.add("arr")
    return kinds


SCALAR = {"str": ("String", "opt_string"), "float": ("f64", "opt_f64"), "int": ("i64", "opt_i64"),
          "bool": ("bool", "opt_bool")}


def lenient_ok(kind, v):
    """crate::de のゆるい読み取りで v を kind として読めるか"""
    if kind == "bool":
        return isinstance(v, bool) or (isinstance(v, str) and v in ("true", "false", "0", "1"))
    if kind == "int":
        if isinstance(v, bool) or isinstance(v, float):
            return False
        return isinstance(v, int) or (isinstance(v, str) and re.fullmatch(r"-?\d+", v) is not None)
    if kind == "float":
        if isinstance(v, bool):
            return False
        return isinstance(v, (int, float)) or (isinstance(v, str) and re.fullmatch(r"-?\d+(\.\d+)?", v) is not None)
    return True


def scalar_type(values):
    """全部の値を読める一番狭い型。ts のような小数点つきの文字列は数値にしない"""
    if all(isinstance(v, str) for v in values):
        return "str"
    for k in ("bool", "int", "float"):
        if all(lenient_ok(k, v) for v in values):
            return k
    return "str"


class Field:
    def __init__(self, key, rust_type, kind, de=None, doc=None):
        self.key = key
        self.ident, self.renamed = field_ident(key)
        self.rust_type = rust_type
        self.kind = kind  # scalar / vec / option / map
        self.de = de
        self.doc = doc


def infer_struct(reg, name, samples, doc, skip_keys=()):
    """samples（dict の列）から構造体を推定して登録する"""
    if name in reg.structs:
        raise SystemExit("duplicate struct name " + name)
    reg.structs[name] = None  # 再帰で同名を作らないよう先に予約
    keys = OrderedDict()
    for s in samples:
        for k, v in s.items():
            if k in skip_keys:
                continue
            keys.setdefault(k, []).append(v)
    fields = []
    seen = set()
    for key, values in keys.items():
        f = infer_field(reg, name, key, values)
        while f.ident in seen:
            f.ident += "_"
            f.renamed = True
        seen.add(f.ident)
        fields.append(f)
    reg.structs[name] = (doc, fields)
    return name


def infer_value_type(reg, owner, key, values):
    """値の列 → (Rust 型, 種別, de 関数)。種別は scalar / object / value"""
    values = [v for v in values if v is not None]
    kinds = scalar_kinds(values)
    if not values:
        return "serde_json::Value", "value", None
    if kinds == {"obj"}:
        if key == "response_metadata":
            return "ResponseMetadata", "object", None
        cores = {core_for(key, v) for v in values} - {None}
        if len(cores) == 1 and (owner, key) not in CORE_EXCLUDE:
            return cores.pop(), "object", None
        if all(not v for v in values):
            return "serde_json::Map<String, serde_json::Value>", "object", None
        if (owner, key) in MAP_KEYS or all(is_id_map(v) or not v for v in values):
            inner = [x for v in values for x in v.values()]
            t, kind, de = infer_value_type(reg, owner, key + "_value", inner)
            if kind == "scalar":
                t = SCALAR[de][0]
            return "std::collections::HashMap<String, %s>" % t, "object", None
        sub = owner + pascal(key)
        if sub in reg.structs or sub in reg.names:
            return sub, "object", None
        infer_struct(reg, sub, values, None)
        return sub, "object", None
    if kinds == {"arr"}:
        t, kind, de = infer_value_type(reg, owner, key, [x for v in values for x in v])
        if kind == "scalar":
            t = SCALAR[de][0]
        return "Vec<%s>" % t, "object", None
    if kinds <= {"str", "int", "float", "bool"}:
        k = scalar_type(values)
        return SCALAR[k][0], "scalar", k
    if "obj" in kinds and "arr" not in kinds:
        return infer_value_type(reg, owner, key, [v for v in values if isinstance(v, dict)])
    return "serde_json::Value", "value", None


FIELD_OVERRIDES = {
    ("ViewInfoState", "values"): "std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>",
    ("Message", "metadata"): "crate::blocks::MessageMetadata",
    # apps.datastore のレコードはアプリが決める項目なので固定の型にしない
    ("AppsDatastoreGetResponse", "item"): "serde_json::Map<String, serde_json::Value>",
    ("AppsDatastorePutResponse", "item"): "serde_json::Map<String, serde_json::Value>",
    ("AppsDatastoreUpdateResponse", "item"): "serde_json::Map<String, serde_json::Value>",
}
VEC_OVERRIDES = {
    ("AppsDatastoreQueryResponse", "items"): "serde_json::Map<String, serde_json::Value>",
    ("AppsDatastoreBulkGetResponse", "items"): "serde_json::Map<String, serde_json::Value>",
}
# キーが絵文字名などの利用者が決める値なので、ID の形でなくてもマップにする: (型名, キー)
MAP_KEYS = {
    ("AdminEmojiListResponse", "emoji"),
    ("EmojiListResponse", "emoji"),
}


def infer_field(reg, owner, key, values):
    if (owner, key) in FIELD_OVERRIDES:
        return Field(key, FIELD_OVERRIDES[(owner, key)], "option", de="opt_object")
    if (owner, key) in VEC_OVERRIDES:
        return Field(key, VEC_OVERRIDES[(owner, key)], "vec")
    present = [v for v in values if v is not None]
    kinds = scalar_kinds(present)
    if kinds == {"arr"}:
        items = [x for v in present for x in v]
        if key in ("blocks", "description_blocks"):
            return Field(key, "crate::blocks::Block", "vec")
        t, kind, de = infer_value_type(reg, owner, key, items)
        if kind == "scalar":
            t = SCALAR[de][0]
        return Field(key, t, "vec")
    t, kind, de = infer_value_type(reg, owner, key, present)
    if kind == "scalar":
        return Field(key, t, "scalar", de=SCALAR[de][1])
    if kind == "value":
        return Field(key, t, "option")
    return Field(key, t, "option", de="opt_object")


# ---------------------------------------------------------------- リクエスト引数

TYPED_ARGS = {
    "blocks": "Vec<crate::blocks::Block>",
    "user_auth_blocks": "Vec<crate::blocks::Block>",
    "description_blocks": "Vec<crate::blocks::Block>",
    "attachments": "Vec<crate::blocks::Attachment>",
    "view": "crate::blocks::View",
}
INT_ARGS = {"count", "page"}


def arg_type(method, a):
    """→ (Rust 型, JSON で送るか)"""
    name, t, ex = a["name"], a["type"], (a["example"] or "").strip()
    if name in TYPED_ARGS:
        # 空の Vec でも `[]` を送る（chat.update で blocks を消すときに要る）
        return TYPED_ARGS[name], TYPED_ARGS[name].startswith("Vec<")
    if name == "metadata" and method.startswith("chat."):
        return "crate::blocks::MessageMetadata", False
    if t == "boolean":
        return "bool", False
    if t == "integer":
        return "i64", False
    if t == "number":
        # ドキュメントで number になっているのは全部 limit（整数）
        return ("f64" if "." in ex else "i64"), False
    if t == "object":
        return "serde_json::Value", False
    if t == "array":
        if ex.startswith("[{") or ex.startswith("[ {") or (ex and not ex.startswith("[") and ex.startswith("{")):
            return "Vec<serde_json::Value>", True
        if name in ("items", "users", "changes", "term_clauses", "permissions", "initial_fields", "schema", "cells",
                    "chunks", "loading_messages", "comments", "workspace_filter", "slack_connect_pref_filter",
                    "restricted_subjects", "trusted_cidr", "trusted_asns") and not re.match(r"^[A-Za-z0-9_, ]+$", ex):
            return "Vec<serde_json::Value>", True
        return "Vec<String>", ex.startswith("[")
    if t is None and name in INT_ARGS:
        return "i64", False
    if t is None and ex in ("true", "false"):
        return "bool", False
    return "String", False


# ---------------------------------------------------------------- 出力

def rust_doc(lines, indent=""):
    return "".join("%s///%s\n" % (indent, (" " + l) if l else "") for l in lines)


def render_struct(name, doc, fields, alias=None):
    out = []
    if doc:
        out.append(rust_doc([doc]))
    if alias:
        out.append('#[doc(alias = "%s")]\n' % alias)
    out.append("#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]\n")
    out.append("pub struct %s {\n" % name)
    for f in fields:
        if f.kind == "flatten":
            out.append("    /// Fields returned by Slack. This method has no documented response example.\n")
            out.append("    #[serde(flatten)]\n    pub %s: %s,\n" % (f.ident, f.rust_type))
            continue
        attrs = []
        if f.renamed:
            attrs.append('rename = "%s"' % f.key.replace('"', '\\"'))
        if f.kind == "vec":
            attrs += ["default", 'deserialize_with = "crate::de::vec"', 'skip_serializing_if = "Vec::is_empty"']
            ty = "Vec<%s>" % f.rust_type
        else:
            attrs.append("default")
            if f.de:
                attrs.append('deserialize_with = "crate::de::%s"' % f.de)
            attrs.append('skip_serializing_if = "Option::is_none"')
            ty = "Option<%s>" % f.rust_type
        out.append("    #[serde(%s)]\n" % ", ".join(attrs))
        out.append("    pub %s: %s,\n" % (f.ident, ty))
    out.append("}\n\n")
    return "".join(out)


def param_type(ty):
    if ty == "String":
        return "impl Into<String>", ".into()"
    return ty, ""


def render_request(m, reg):
    name = method_pascal(m["name"]) + "Request"
    resp = method_pascal(m["name"]) + "Response"
    args = []
    seen = set()
    for a in m["args"]:
        ident, renamed = field_ident(a["name"])
        if ident in seen:
            continue
        seen.add(ident)
        ty, as_json = arg_type(m["name"], a)
        args.append((a, ident, renamed, ty, as_json))
    required = [x for x in args if x[0]["required"]]
    optional = [x for x in args if not x[0]["required"]]
    out = []
    out.append(rust_doc(["Arguments for the Slack Web API method [`%s`](%s): %s" % (m["name"], doc_url(m["name"]), doc_line(m["summary"])),
                         "", "Send it with [`SlackClient::%s`]." % method_snake(m["name"])]))
    out.append('#[doc(alias = "%s")]\n' % m["name"])
    derive = "Debug, Clone, PartialEq, Serialize" + (", Default" if not required else "")
    out.append("#[derive(%s)]\n#[non_exhaustive]\npub struct %s {\n" % (derive, name))
    for a, ident, renamed, ty, as_json in args:
        if a["description"]:
            out.append(rust_doc([doc_line(a["description"])], "    "))
        attrs = []
        if renamed:
            attrs.append('rename = "%s"' % a["name"])
        if not a["required"]:
            attrs.append('skip_serializing_if = "Option::is_none"')
        if as_json:
            attrs.append('serialize_with = "crate::form::as_json"')
        if attrs:
            out.append("    #[serde(%s)]\n" % ", ".join(attrs))
        out.append("    pub %s: %s,\n" % (ident, ty if a["required"] else "Option<%s>" % ty))
    out.append("}\n\n")

    out.append("impl %s {\n" % name)
    params = ", ".join("%s: %s" % (ident, param_type(ty)[0]) for _, ident, _, ty, _ in required)
    out.append("    pub fn new(%s) -> Self {\n        Self {\n" % params)
    for _, ident, _, ty, _ in required:
        conv = param_type(ty)[1]
        out.append("            %s%s,\n" % (ident, (": " + ident + conv) if conv else ""))
    for _, ident, _, _, _ in optional:
        out.append("            %s: None,\n" % ident)
    out.append("        }\n    }\n")
    for _, ident, _, ty, _ in optional:
        pty, conv = param_type(ty)
        out.append("\n    pub fn %s(mut self, %s: %s) -> Self {\n        self.%s = Some(%s%s);\n        self\n    }\n"
                   % (ident.replace("r#", "r#"), ident, pty, ident, ident, conv))
    out.append("}\n\n")

    if m["name"] not in BYTES_METHODS:
        out.append("impl SlackApiMethod for %s {\n    const METHOD: &'static str = \"%s\";\n    type Response = %s;\n}\n\n"
                   % (name, m["name"], resp))
    if any(ident == "cursor" for _, ident, _, _, _ in optional) and m["name"] not in BYTES_METHODS:
        out.append("impl CursorPaginated for %s {\n    fn set_cursor(&mut self, cursor: String) {\n"
                   "        self.cursor = Some(cursor);\n    }\n}\n\n" % name)
        if any("next_cursor" in r for r in m["responses"]):
            # admin.conversations.search などは next_cursor を応答の一番上に置く
            body = ("self.next_cursor.as_deref().filter(|c| !c.is_empty())\n"
                    "            .or_else(|| self.response_metadata.as_ref()?.next_cursor.as_deref())")
        else:
            body = "self.response_metadata.as_ref()?.next_cursor.as_deref()"
        out.append("impl NextCursor for %s {\n    fn next_cursor(&self) -> Option<&str> {\n"
                   "        %s\n    }\n}\n\n" % (resp, body))
    return "".join(out), required, optional


def render_response(m, reg):
    name = method_pascal(m["name"]) + "Response"
    infer_struct(reg, name, m["responses"] or [{}],
                 "Successful response of the Slack Web API method [`%s`](%s)." % (m["name"], doc_url(m["name"])),
                 skip_keys=("ok",))
    reg.aliases[name] = m["name"]
    doc, fields = reg.structs[name]
    keys = {f.key for f in fields}
    if not keys:
        # 応答例が無い（か ok だけ）ので項目が分からない。捨てずに extra に全部残す
        fields.append(Field("extra", "serde_json::Map<String, serde_json::Value>", "flatten"))
    if "response_metadata" not in keys:
        fields.append(Field("response_metadata", "ResponseMetadata", "option", de="opt_object"))
    if "warning" not in keys:
        fields.append(Field("warning", "String", "scalar", de="opt_string"))
    return name


def rate_limit_line(m):
    rl = m["rate_limit"]
    if not rl:
        return None
    if re.fullmatch(r"t[1-5]", rl):
        return "Rate limit: Tier %s (<https://docs.slack.dev/apis/web-api/rate-limits>)." % rl[1]
    return "Rate limit: %s" % doc_line(rl)


def scope_lines(m):
    labels = {"bot": "bot token", "user": "user token"}
    return ["- %s: %s" % (labels.get(k, k), ", ".join("`%s`" % s for s in v)) for k, v in m["scopes"].items() if v]


def render_client_fn(m):
    fn = method_snake(m["name"])
    req = method_pascal(m["name"]) + "Request"
    lines = ["Calls the Slack Web API method [`%s`](%s): %s" % (m["name"], doc_url(m["name"]), doc_line(m["summary"]))]
    scopes = scope_lines(m)
    if scopes:
        lines += ["", "Required scopes:", ""] + scopes
    if rate_limit_line(m):
        lines += ["", rate_limit_line(m)]
    if m["name"] in BYTES_METHODS:
        lines += ["", "Returns the raw file body (a gzip-compressed JSON Lines file), not JSON."]
    lines += ["", "# Errors", "",
              "Returns [`SlackError::Api`] when Slack answers `\"ok\": false`; see [`SlackClient::call`] for the other cases."]
    out = [rust_doc(lines, "    ")]
    out.append('    #[doc(alias = "%s")]\n' % m["name"])
    if m["deprecated"]:
        out.append('    #[deprecated(note = "%s")]\n' % doc_line(m["deprecated"]).replace('"', "'"))
    if m["name"] in BYTES_METHODS:
        out.append("    pub async fn %s(&self, request: &%s) -> Result<bytes::Bytes, SlackError> {\n"
                   "        self.call_bytes(\"%s\", request).await\n    }\n" % (fn, req, m["name"]))
    else:
        out.append("    pub async fn %s(&self, request: &%s) -> Result<%sResponse, SlackError> {\n"
                   "        self.call(request).await\n    }\n" % (fn, req, method_pascal(m["name"])))
    return "".join(out)


def module_of(method):
    parts = method.split(".")
    if parts[0] == "admin":
        return "admin_" + snake(parts[1])
    if parts[0] == "api":
        # api::api だと clippy::module_inception になる
        return "api_test"
    return snake(parts[0])


def category(method):
    parts = method.split(".")
    return parts[0] + "." + parts[1] if parts[0] == "admin" else parts[0]


def api_module_doc(methods):
    lines = [
        "Request and response types for every Slack Web API method, and the [`SlackClient`](crate::SlackClient)",
        "function that calls each one.",
        "",
        "Naming: the method `chat.postMessage` is called with",
        "[`SlackClient::chat_post_message`](crate::SlackClient::chat_post_message), which takes a",
        "[`ChatPostMessageRequest`] and returns a [`ChatPostMessageResponse`]. Searching docs.rs for the Slack method",
        "name (for example `conversations.history`) finds the function and its types.",
        "",
        "## All methods",
        "",
        "| Slack method | Function | Summary |",
        "|---|---|---|",
    ]
    for m in methods:
        summary = doc_line(m["summary"]).replace("|", "\\|")
        if len(summary) > 110:
            summary = summary[:107].rstrip() + "..."
        lines.append("| [`%s`](%s) | [`%s`](crate::SlackClient::%s) | %s |"
                     % (m["name"], doc_url(m["name"]), method_snake(m["name"]), method_snake(m["name"]), summary))
    return "".join("//!%s\n" % ((" " + l) if l else "") for l in lines) + "\n"


def write_llms_full(methods):
    out = ["# slack-web-api: all Slack Web API methods", "",
           "> Every Slack Web API method supported by the Rust crate `slack-web-api`, with the Rust function and types.",
           "> Call pattern: `client.<function>(&<Request>::new(<required args>).<optional_arg>(value)).await?` returns `<Response>`.",
           "> Required arguments are the parameters of `new`; every other argument is a builder method of the same name.", ""]
    current = None
    for m in methods:
        cat = category(m["name"])
        if cat != current:
            out += ["", "## " + cat, ""]
            current = cat
        p = method_pascal(m["name"])
        required = [field_ident(a["name"])[0] for a in m["args"] if a["required"]]
        ret = "bytes::Bytes" if m["name"] in BYTES_METHODS else p + "Response"
        out.append("- `%s` -> `client.%s(&%sRequest::new(%s))` -> `%s`: %s"
                   % (m["name"], method_snake(m["name"]), p, ", ".join(required), ret, doc_line(m["summary"])))
    with open(os.path.join(ROOT, "llms-full.txt"), "w") as f:
        f.write("\n".join(out) + "\n")


def write_readme_tables(methods):
    """README の <!-- methods:start --> 〜 <!-- methods:end --> をカテゴリ別の一覧で置き換える"""
    groups = OrderedDict()
    for m in methods:
        top = m["name"].split(".")[0]
        groups.setdefault(top, []).append(m["name"])
    rows = ["| Category | Methods | Examples |", "|---|---:|---|"]
    for top in sorted(groups, key=lambda t: (t == "admin", t)):
        names = groups[top]
        examples = ", ".join("`%s`" % n for n in names[:3]) + (", ..." if len(names) > 3 else "")
        rows.append("| `%s.*` | %d | %s |" % (top, len(names), examples))
    table = "\n".join(rows)
    for readme in ("README.md", "README.ja.md"):
        path = os.path.join(ROOT, readme)
        text = open(path, encoding="utf-8").read()
        text = re.sub(r"<!-- methods:start -->.*?<!-- methods:end -->",
                      lambda _: "<!-- methods:start -->\n" + table + "\n<!-- methods:end -->", text, flags=re.S)
        open(path, "w", encoding="utf-8").write(text)


HEADER = "// このファイルは codegen/generate.py が生成する。手で編集しない\n\n"


def main():
    methods = [m for m in json.load(open(os.path.join(SPEC, "methods.json"))) if m["name"] not in SKIP_METHODS]
    objects = json.load(open(os.path.join(SPEC, "objects.json")))
    supplements = json.load(open(os.path.join(SPEC, "supplements.json")))
    supplements.pop("_comment")
    extra_responses = supplements.pop("responses")
    for m in methods:
        m["responses"] = m["responses"] + extra_responses.get(m["name"], [])
    for name, samples in supplements.items():
        objects.setdefault(name, []).extend(samples)
    reg = Registry()
    for name in CORE_TYPES:
        reg.names.add(name)

    # 1. 主要オブジェクトの例を集める（オブジェクトのページの例 + 全メソッドの例の中に出てくるもの）
    for name, samples in objects.items():
        for s in samples:
            reg.core_samples[name].append(s)
            for k, v in s.items():
                reg.collect_core(v, k)
    for m in methods:
        for r in m["responses"]:
            reg.collect_core(r, owner=method_pascal(m["name"]) + "Response")

    # 2. 主要オブジェクトの型
    for name in CORE_TYPES:
        infer_struct(reg, name, reg.core_samples[name] or [{}], CORE_DOCS[name])
    core_struct_names = list(reg.structs.keys())

    # 3. メソッドごと
    modules = OrderedDict()
    for m in methods:
        mod = module_of(m["name"])
        before = set(reg.structs)
        req_src, required, optional = render_request(m, reg)
        resp = render_response(m, reg) if m["name"] not in BYTES_METHODS else None
        new_structs = [n for n in reg.structs if n not in before]
        modules.setdefault(mod, []).append((m, req_src, new_structs, required, optional))

    # objects.rs
    with open(os.path.join(ROOT, "src", "objects.rs"), "w") as f:
        f.write(HEADER)
        f.write("//! Core Slack objects shared by many responses, inferred from the documented examples of every method.\n\n")
        f.write("use serde::{Deserialize, Serialize};\n\n")
        for n in core_struct_names:
            doc, fields = reg.structs[n]
            f.write(render_struct(n, doc, fields))

    # api/*.rs
    api_dir = os.path.join(ROOT, "src", "api")
    os.makedirs(api_dir, exist_ok=True)
    for mod, entries in modules.items():
        with open(os.path.join(api_dir, mod + ".rs"), "w") as f:
            f.write(HEADER)
            f.write("#![allow(unused_imports)]\n\n")
            f.write("use crate::objects::*;\nuse crate::{CursorPaginated, NextCursor, ResponseMetadata, "
                    "SlackApiMethod, SlackClient, SlackError};\nuse serde::{Deserialize, Serialize};\n\n")
            for m, req_src, new_structs, _, _ in entries:
                f.write(req_src)
                for n in new_structs:
                    doc, fields = reg.structs[n]
                    f.write(render_struct(n, doc, fields, reg.aliases.get(n)))
            f.write("impl SlackClient {\n")
            f.write("\n".join(render_client_fn(m) for m, _, _, _, _ in entries))
            f.write("}\n")
    with open(os.path.join(api_dir, "mod.rs"), "w") as f:
        f.write(HEADER)
        f.write(api_module_doc(methods))
        for mod in sorted(modules):
            f.write("mod %s;\n" % mod)
        f.write("\n")
        for mod in sorted(modules):
            f.write("pub use %s::*;\n" % mod)

    write_llms_full(methods)
    write_readme_tables(methods)
    write_tests(methods, modules)
    print("methods=%d modules=%d structs=%d" % (len(methods), len(modules), len(reg.structs)), file=sys.stderr)


# ---------------------------------------------------------------- テスト

def sample_arg(ty, a):
    """引数の型に合う値の式（ドキュメントの例を優先）"""
    ex = a["example"]
    if ty == "String":
        return json.dumps(ex if ex else "x", ensure_ascii=False)
    if ty == "bool":
        return "true"
    if ty == "i64":
        return "1"
    if ty == "f64":
        return "1.5"
    if ty == "Vec<String>":
        return 'vec!["A1".to_string(), "A2".to_string()]'
    if ty == "Vec<serde_json::Value>":
        return 'vec![serde_json::json!({"k": "v"})]'
    if ty == "serde_json::Value":
        return 'serde_json::json!({"k": "v"})'
    if ty == "Vec<crate::blocks::Block>":
        return "vec![slack_web_api::blocks::Block::from(slack_web_api::blocks::DividerBlock::new())]"
    if ty == "Vec<crate::blocks::Attachment>":
        return "vec![slack_web_api::blocks::Attachment::new()]"
    if ty == "crate::blocks::View":
        return "slack_web_api::blocks::View::modal(slack_web_api::blocks::TextObject::plain(\"t\"), vec![])"
    if ty == "crate::blocks::MessageMetadata":
        return 'slack_web_api::blocks::MessageMetadata::new("e", serde_json::json!({}))'
    raise SystemExit("no sample for " + ty)


def write_tests(methods, modules):
    out = [HEADER, "//! 全メソッドを、ドキュメントの成功例を返すモックサーバーに対して呼ぶ\n\n",
           "use slack_web_api::api::*;\nuse slack_web_api::{CursorPaginated, NextCursor, ResponseMetadata, SlackClient};\n",
           "use wiremock::matchers::{method, path};\nuse wiremock::{Mock, MockServer, ResponseTemplate};\n\n"]
    out.append("""async fn setup(name: &str, body: &str) -> (MockServer, SlackClient) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/api/{name}")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body.to_owned(), "application/json"))
        .mount(&server)
        .await;
    let client = SlackClient::builder().token("xoxb-test").base_url(format!("{}/api", server.uri())).build();
    (server, client)
}

""")
    out.append(PRESERVED_HELPER)
    for m in methods:
        fn = method_snake(m["name"])
        req = method_pascal(m["name"]) + "Request"
        args = []
        seen = set()
        for a in m["args"]:
            ident, _ = field_ident(a["name"])
            if ident in seen:
                continue
            seen.add(ident)
            args.append((a, ident, arg_type(m["name"], a)[0]))
        req_expr = "%s::new(%s)" % (req, ", ".join(sample_arg(ty, a) for a, _, ty in args if a["required"]))
        for a, ident, ty in args:
            if not a["required"]:
                req_expr += "\n        .%s(%s)" % (ident, sample_arg(ty, a))
        out.append("#[tokio::test]\n#[allow(deprecated)]\nasync fn %s() {\n" % fn)
        out.append("    let req = %s;\n" % req_expr)
        if any(ident == "cursor" for a, ident, _ in args if not a["required"]):
            resp = method_pascal(m["name"]) + "Response"
            out.append("    let mut next = req.clone();\n    next.set_cursor(\"c2\".into());\n"
                       "    assert_eq!(next.cursor.as_deref(), Some(\"c2\"));\n")
            out.append("    assert_eq!(%s::default().next_cursor(), None);\n" % resp)
            out.append("    let page = %s { response_metadata: Some(ResponseMetadata { next_cursor: Some(\"n\".into()), "
                       "..Default::default() }), ..Default::default() };\n" % resp)
            out.append("    assert_eq!(page.next_cursor(), Some(\"n\"));\n")
            if any("next_cursor" in r for r in m["responses"]):
                out.append("    let top = %s { next_cursor: Some(\"t\".into()), ..Default::default() };\n" % resp)
                out.append("    assert_eq!(top.next_cursor(), Some(\"t\"));\n")
        if m["name"] in BYTES_METHODS:
            out.append("    let (_server, client) = setup(%s, \"gzip-bytes\").await;\n" % json.dumps(m["name"]))
            out.append("    assert_eq!(&client.%s(&req).await.unwrap()[..], b\"gzip-bytes\");\n}\n\n" % fn)
            continue
        for ex in (m["responses"] or [{"ok": True}]):
            body = rust_raw(json.dumps(ex, ensure_ascii=False))
            out.append("    {\n        let (_server, client) = setup(%s, %s).await;\n" % (json.dumps(m["name"]), body))
            out.append("        let res = client.%s(&req).await.expect(%s);\n" % (fn, json.dumps(m["name"])))
            out.append("        assert_preserved(%s, %s, &res);\n    }\n" % (json.dumps(m["name"]), body))
        out.append("}\n\n")
    with open(os.path.join(ROOT, "tests", "generated_methods.rs"), "w") as f:
        f.write("".join(out))


PRESERVED_HELPER = r"""/// 応答例の値が、型に読んで書き戻したあとも全部残っているか（null・空配列・空オブジェクトは除く）。
/// 数値と数字の文字列は同じとみなす（ゆるい読み取りで型が寄るため）
fn assert_preserved<T: serde::Serialize>(name: &str, original: &str, res: &T) {
    let original: serde_json::Value = serde_json::from_str(original).unwrap();
    let back = serde_json::to_value(res).unwrap();
    let mut missing = vec![];
    covers(&original, &back, String::new(), &mut missing);
    missing.retain(|p| p != ".ok");
    assert!(missing.is_empty(), "{name}: lost {missing:?}");
}

fn covers(a: &serde_json::Value, b: &serde_json::Value, at: String, missing: &mut Vec<String>) {
    use serde_json::Value::*;
    match (a, b) {
        (Null, _) => {}
        (Object(x), _) if x.is_empty() => {}
        (Array(x), _) if x.is_empty() => {}
        (Object(x), Object(y)) => {
            for (k, v) in x {
                match y.get(k) {
                    Some(w) => covers(v, w, format!("{at}.{k}"), missing),
                    None if v.is_null() || v.as_array().is_some_and(|a| a.is_empty()) || v.as_object().is_some_and(|o| o.is_empty()) => {}
                    None => missing.push(format!("{at}.{k}")),
                }
            }
        }
        (Array(x), Array(y)) if x.len() == y.len() => {
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                covers(v, w, format!("{at}[{i}]"), missing);
            }
        }
        (x, y) if x == y => {}
        (x, y) if scalar_text(x).is_some() && scalar_text(x) == scalar_text(y) => {}
        _ => missing.push(at),
    }
}

fn scalar_text(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => n.as_f64().map(|f| f.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

"""


def rust_raw(s):
    hashes = "#"
    while ('"' + hashes) in s:
        hashes += "#"
    return "r%s\"%s\"%s" % (hashes, s, hashes)


if __name__ == "__main__":
    main()
