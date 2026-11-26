// Loads the native module built by `rust-anything build`.
const native = require('./twake_domain_ref.node')

// Structured values cross as JSON text. JSON has no `undefined`, NaN, -0, bigint, Date, Map, Set
// or bytes, and Rust strings no lone surrogates: those travel as one-key marker objects
// (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$map": [[k, v]]}`, `{"$bytes": [...]}`, ...),
// the same markers recordings use.
const replacer = function (k, v) {
  const raw = this[k]
  if (raw instanceof Date) return { $date: Number.isNaN(raw.getTime()) ? 'Invalid' : raw.toISOString() }
  if (typeof Buffer !== 'undefined' && Buffer.isBuffer(raw)) return { $buffer: Array.from(raw) }
  if (raw instanceof Uint8Array) return { $bytes: Array.from(raw) }
  if (raw instanceof Map) return { $map: [...raw] }
  if (raw instanceof Set) return { $set: [...raw] }
  return v === undefined ? { $undefined: true }
    : typeof v === 'bigint' ? { $bigint: String(v) }
    : typeof v === 'number' ? (Number.isFinite(v) && !Object.is(v, -0) ? v : { $num: Object.is(v, -0) ? '-0' : String(v) })
    : typeof v === 'string' && !v.isWellFormed() ? { $utf16: Array.from({ length: v.length }, (_, i) => v.charCodeAt(i)) }
    : v
}
const encode = v => JSON.stringify(v, replacer)
const single = (v, key) => Object.keys(v).length === 1 && key in v
const revive = v => Array.isArray(v) ? v.map(revive)
  : v !== null && typeof v === 'object'
    ? (single(v, '$undefined') ? undefined
      : single(v, '$utf16') ? fromUnits(v.$utf16)
      : single(v, '$num') ? Number(v.$num)
      : single(v, '$bigint') ? BigInt(v.$bigint)
      : single(v, '$date') ? new Date(v.$date)
      : single(v, '$bytes') ? Uint8Array.from(v.$bytes)
      : single(v, '$buffer') ? Buffer.from(v.$buffer)
      : single(v, '$map') ? new Map(v.$map.map(([k, x]) => [revive(k), revive(x)]))
      : single(v, '$set') ? new Set(v.$set.map(revive))
      : Object.fromEntries(Object.entries(v).map(([k, x]) => [k, revive(x)])))
    : v
// Most results carry no marker: skip the walk unless the text has one.
const decode = text => {
  const value = JSON.parse(text)
  return text.includes('"$') ? revive(value) : value
}
const fromUnits = (units) => {
  let out = ''
  for (let i = 0; i < units.length; i += 8192)
    out += String.fromCharCode.apply(null, Array.prototype.slice.call(units, i, i + 8192))
  return out
}

module.exports = { ...native }
const specs = {
  buildActivity: [[1, 1, 1, 1], 'json', false],
  buildFeed: [[1, 1, 1], 'json', false],
  filterFeed: [[1, 0], 'json', false],
  searchFeed: [[1, 0], 'json', false],
  groupByDay: [[1, 0], 'json', false],
  assistantConsentStatus: [[1, 0, 0], 'json', false],
  requestsWaitingFor: [[1, 0, 0], 'json', false],
  peopleOf: [[1, 1], 'raw', false],
  presentRequest: [[1, 1], 'json', false],
  findOpenQuestion: [[1, 1, 0, 0], 'json', false],
  computeWaveform: [[1, 1], 'raw', false],
  isPlayableAudio: [[1], 'raw', false],
  isViewablePdf: [[1, 0], 'raw', false],
  isPreviewable: [[1], 'raw', false],
  isGif: [[1], 'raw', false],
  svgCanvasSize: [[0, 0], 'json', false],
  thumbnailSize: [[1, 1], 'json', false],
  botOf: [[1], 'json', false],
  findCommandQuery: [[0, 0], 'json', false],
  matchCommands: [[1, 0], 'json', false],
  findLocalCommand: [[0], 'json', false],
  prepareCommandText: [[0, 1], 'json', false],
  prepareOutgoing: [[0, 1, 1], 'json', false],
  shouldMirror: [[1], 'json', false],
  currentDeviceId: [[1, 0], 'raw', false],
  countCallPeople: [[1], 'raw', false],
  ringAsks: [[1, 1, 0, 0, 0], 'raw', false],
  isRinging: [[1, 0, 0, 0, 0], 'raw', false],
  findCallThread: [[1, 0], 'json', false],
  toCallLine: [[1, 0], 'json', false],
  fitWindow: [[1, 1], 'json', false],
  defaultCallWindow: [[1], 'json', false],
  moveWindow: [[1, 0, 0, 1], 'json', false],
  placeInCorner: [[1, 0, 1], 'json', false],
  resizeWindow: [[1, 0, 0, 0, 1], 'json', false],
  parseCallWindow: [[0], 'json', false],
  placeCall: [[1, 1], 'json', false],
  isCallFloating: [[1, 0], 'raw', false],
  parseClientConfig: [[1], 'json', false],
  readComposerStyle: [[1], 'json', false],
  parseInvitationTarget: [[0], 'json', false],
  groupByInitial: [[1], 'json', false],
  parseContactsVisibility: [[1], 'raw', false],
  emptyCardOf: [[1], 'json', false],
  footerOf: [[1], 'json', false],
  timelineHeadOf: [[1], 'json', false],
  headOverWindow: [[1, 0], 'json', false],
  isLandingRoom: [[1, 0], 'raw', false],
  mayPostIn: [[1], 'raw', false],
  isDirectRoom: [[1, 0], 'raw', false],
  peerStateOf: [[1], 'json', false],
  splitsCallChat: [[1], 'json', false],
  isEncryptedConversation: [[1], 'raw', false],
  isAwaitingRoom: [[1], 'raw', false],
  roomNameOf: [[1], 'raw', false],
  assistantOfferIn: [[1], 'raw', false],
  hasAssistantIn: [[1, 1, 1], 'raw', false],
  offersAddContact: [[1], 'raw', false],
  matchPlaceOf: [[0, 0], 'json', false],
  readDeletedChats: [[1], 'json', false],
  isChatHidden: [[1, 0, 0], 'raw', false],
  withoutDeleted: [[1, 0, 1], 'json', false],
  isAfterDeletion: [[1, 0, 0], 'raw', false],
  readDismissedQuestions: [[1], 'json', false],
  withDismissedQuestion: [[1, 0, 0, 0], 'json', false],
  findEmojiQuery: [[0, 0], 'json', false],
  isEmojiDrawable: [[0, 1, 0], 'raw', false],
  hasInk: [[1], 'raw', false],
  hasColour: [[1], 'raw', false],
  isDrawn: [[1, 1], 'raw', false],
  isNotificationService: [[1], 'raw', false],
  readFolders: [[1], 'json', false],
  writeFolders: [[1], 'json', false],
  withHiddenBuiltins: [[1], 'json', false],
  listFolders: [[1, 1], 'json', false],
  folderChatTypeOf: [[1], 'raw', false],
  isInFolder: [[1, 1], 'raw', false],
  isRecommendationTaken: [[0, 1], 'raw', false],
  fitPreview: [[1, 0, 0], 'json', false],
  isRecord: [[1], 'raw', false],
  readText: [[1, 0], 'raw', false],
  readTexts: [[1, 0], 'raw', false],
  readBoolean: [[1, 0, 0], 'raw', false],
  autoTranslationOf: [[1, 0], 'raw', false],
  parseGeoUri: [[1], 'json', false],
  openStreetMapUrl: [[1], 'raw', false],
  pickAttendees: [[1, 1, 0], 'json', false],
  readCreatedMeeting: [[1], 'json', false],
  meetingMessageText: [[1, 0, 0], 'raw', false],
  describePeople: [[0, 1], 'raw', false],
  matchMembers: [[1, 0, 0, 0], 'json', false],
  mentionText: [[1], 'raw', false],
  keepPresentMentions: [[0, 1], 'json', false],
  matchRooms: [[1, 0, 0], 'json', false],
  roomMentionRef: [[1], 'raw', false],
  roomMentionText: [[1], 'raw', false],
  isRoomMention: [[1], 'raw', false],
  isEveryoneMention: [[1], 'raw', false],
  countsAsUnread: [[1], 'raw', false],
  findFirstUnreadId: [[1, 0], 'raw', false],
  isSelectable: [[1], 'raw', false],
  startsGroup: [[1, 0], 'raw', false],
  groupAlbums: [[1, 0, 0], 'json', false],
  startsDay: [[1, 0], 'raw', false],
  isTranslatable: [[1], 'raw', false],
  isEditable: [[1], 'raw', false],
  shouldNotify: [[1, 0, 1], 'raw', false],
  parsePermalink: [[0, 0], 'json', false],
  parseRoomAddress: [[0, 0], 'json', false],
  readSeenSteps: [[1], 'raw', false],
  unionSteps: [[1], 'raw', false],
  tallyPoll: [[1, 1, 0, 0], 'json', false],
  pressAnswer: [[1, 0], 'json', false],
  readNewPoll: [[0, 0], 'json', false],
  computeRoomUnread: [[1, 1], 'json', false],
  computeSilentUnread: [[1, 1], 'raw', false],
  isUnread: [[1], 'raw', false],
  computeTotalUnread: [[1], 'json', false],
  computeReceiptTargets: [[1], 'json', false],
  isRoomUnread: [[1, 0], 'raw', false],
  shownUnreadCount: [[1], 'raw', false],
  emptyReadState: [[0], 'json', false],
  makeReadRoomState: [[1], 'json', false],
  hasGifsOff: [[1], 'raw', false],
  offersScribe: [[1], 'raw', false],
  spaceOfRoom: [[1], 'raw', false],
  canBeListenedTo: [[1], 'raw', false],
  computeRoomKind: [[1], 'raw', false],
  canPost: [[1, 0], 'raw', false],
  findSpaceGeneral: [[1, 0], 'json', false],
  readArchive: [[1], 'json', false],
  withArchived: [[1, 0, 1], 'json', false],
  toRoomLines: [[1, 0, 0, 1, 0], 'json', false],
  folderView: [[1], 'json', false],
  isInView: [[1, 1, 1, 1], 'raw', false],
  isRoomOwner: [[1], 'raw', false],
  canManageMember: [[1, 1, 0], 'raw', false],
  canChangeRole: [[1, 1, 0], 'raw', false],
  assignableRoles: [[1, 1, 0], 'raw', false],
  canBanMember: [[1, 1, 0], 'raw', false],
  canSetReadOnly: [[1, 1, 0], 'raw', false],
  ownershipAction: [[1], 'raw', false],
  canTakeOwnership: [[1, 1, 0], 'raw', false],
  canTransferTo: [[1, 1, 0], 'raw', false],
  hasStaffRole: [[1], 'raw', false],
  permissionRoleOf: [[1, 1], 'json', false],
  givenRoles: [[1], 'raw', false],
  canHavePublicPage: [[1], 'raw', false],
  publicPagePath: [[1], 'raw', false],
  roomIdOfRoute: [[0, 1], 'raw', false],
  readSavedRefs: [[1], 'json', false],
  withSaved: [[1, 0, 0, 0, 1], 'json', false],
  readAutoTranslate: [[1], 'raw', false],
  toIndexedMessage: [[0, 1], 'json', false],
  isEmptyFilter: [[1], 'raw', false],
  matchesFilter: [[1, 1], 'raw', false],
  computeSharedContent: [[1], 'json', false],
  roomBadgeCount: [[1], 'raw', false],
  computeSpaceBadges: [[1], 'json', false],
  canMoveToSpace: [[1, 1], 'raw', false],
  arrangeRooms: [[1, 0], 'json', false],
  staysInTree: [[1], 'raw', false],
  parentSpaceOf: [[1, 1], 'json', false],
  presenceOfPreset: [[1], 'raw', false],
  sortThreadsByActivity: [[1], 'json', false],
  readThreadPrefs: [[1], 'json', false],
  withThreadPref: [[1, 0, 0, 0], 'json', false],
  resolveThreadFollow: [[0, 0], 'json', false],
  timelineWindowOf: [[1, 1, 1, 1], 'json', false],
  olderStartOf: [[1, 0], 'json', false],
  commonSettingsUrlFor: [[1, 0], 'raw', false],
  findWebLinks: [[0], 'json', false],
}
const classes = {

}
const convert = result => result === 'json' ? decode : result === 'units' ? fromUnits : x => x
// An omitted structured argument is `undefined`, not a missing value.
const prepare = (a, structured) => Array.from({ length: Math.max(a.length, structured.length) }, (_, i) => structured[i] ? encode(a[i]) : a[i])
// Functions: [structured arguments, result kind, async].
for (const [name, [structured, result, isAsync]] of Object.entries(specs)) {
  const call = native[name]
  const out = convert(result)
  module.exports[name] = isAsync
    ? async (...a) => out(await call(...prepare(a, structured)))
    : (...a) => out(call(...prepare(a, structured)))
}
// Classes: a JS class around the native one, converting arguments and results.
const INNER = Symbol('native')
for (const [name, [ctor, methods]] of Object.entries(classes)) {
  const Native = native[name]
  const Wrapped = { [name]: class { constructor(...a) { this[INNER] = new Native(...prepare(a, ctor)) } } }[name]
  for (const [method, [structured, result]] of Object.entries(methods)) {
    const out = convert(result)
    Wrapped.prototype[method] = function (...a) { return out(this[INNER][method](...prepare(a, structured))) }
  }
  module.exports[name] = Wrapped
}

// The binary wire (see rust-anything's `wire.rs`): arguments are written into `wIn`, results read
// from `wOut`; both are registered with the native module once.
let wIn = new Uint8Array(1 << 16), wOut = new Uint8Array(1 << 16)
let iv = new DataView(wIn.buffer), ov = new DataView(wOut.buffer), at = 0, p = 0
native.rustAnythingWireInit(wIn, wOut)
const regrow = () => native.rustAnythingWireInit(wIn, wOut)
// Runs a wire function; a result larger than `wOut` is fetched once `wOut` has grown.
const wireCall = fn => {
  const n = fn(at)
  if (Number.isNaN(n)) return false
  if (n < 0) {
    wOut = new Uint8Array(Math.max(wOut.length * 2, -n))
    ov = new DataView(wOut.buffer)
    native.rustAnythingWireInit(wIn, wOut)
    native.rustAnythingWireFlush()
  }
  p = 0
  return true
}

const need = n => {
  if (at + n <= wIn.length) return
  const grown = new Uint8Array(Math.max(wIn.length * 2, at + n))
  grown.set(wIn)
  wIn = grown
  iv = new DataView(wIn.buffer)
  regrow()
}
const wU8 = v => { need(1); iv.setUint8(at++, v) }
const wU32 = v => { need(4); iv.setUint32(at, v, true); at += 4 }
const wF64 = v => { need(8); iv.setFloat64(at, v, true); at += 8 }
const wUnits = s => {
  need(4 + 2 * s.length)
  iv.setUint32(at, s.length, true)
  at += 4
  for (let i = 0; i < s.length; i++) { iv.setUint16(at, s.charCodeAt(i), true); at += 2 }
}
const wStr = s => { wU8(4); wUnits(s) }
const wDyn = v => {
  switch (typeof v) {
    case 'undefined': wU8(0); return true
    case 'boolean': wU8(v ? 3 : 2); return true
    case 'number': wU8(4); wF64(v); return true
    case 'string': wU8(5); wUnits(v); return true
    case 'object':
      if (v === null) { wU8(1); return true }
      if (Array.isArray(v)) {
        wU8(6); wU32(v.length)
        for (let i = 0; i < v.length; i++) if (!wDyn(v[i])) return false
        return true
      }
      if (Object.getPrototypeOf(v) !== Object.prototype && Object.getPrototypeOf(v) !== null) return false
      {
        const keys = Object.keys(v)
        wU8(7); wU32(keys.length)
        for (const k of keys) { wUnits(k); if (!wDyn(v[k])) return false }
        return true
      }
  }
  return false
}
const rU8 = () => ov.getUint8(p++)
const rU32 = () => { const v = ov.getUint32(p, true); p += 4; return v }
const rF64 = () => { const v = ov.getFloat64(p, true); p += 8; return v }
// ignoreBOM: a leading U+FEFF is part of the string, not a byte-order mark.
const utf16 = new TextDecoder('utf-16le', { ignoreBOM: true })
const rUnits = (wellFormed) => {
  const n = rU32()
  let s
  if (n > 24 && wellFormed) s = utf16.decode(wOut.subarray(p, p + 2 * n))
  else {
    s = ''
    for (let i = 0; i < n; i++) s += String.fromCharCode(ov.getUint16(p + 2 * i, true))
  }
  p += 2 * n
  return s
}
// Tag 1: a string from Rust (well-formed); 4: raw code units, lone surrogates included.
const rStr = () => rUnits(rU8() === 1)
const rDyn = () => {
  switch (rU8()) {
    case 0: return undefined
    case 1: return null
    case 2: return false
    case 3: return true
    case 4: return rF64()
    case 5: return rUnits(false)
    case 6: { const n = rU32(); const a = new Array(n); for (let i = 0; i < n; i++) a[i] = rDyn(); return a }
    case 7: { const n = rU32(); const o = {}; for (let i = 0; i < n; i++) { const k = rUnits(false); o[k] = rDyn() } return o }
  }
}

// `buildActivity`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.buildActivity
const call = native.buildActivityWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "message": {
wU8(2)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderId": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderAvatarUrl": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isFromBot": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "text": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "html": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "undecryptable": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "notice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "attachment": {
wU8(9)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "location": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "lat": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "lon": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "poll": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "question": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "answers": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "maxSelections": {
wU8(2)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isDisclosed": {
wU8(3)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isAnonymous": {
wU8(4)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "endedAt": {
wU8(5)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "votes": {
wU8(6)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "voters": {
wU8(7)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "totalVotes": {
wU8(8)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "ownAnswerIds": {
wU8(9)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canVote": {
wU8(10)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canEnd": {
wU8(11)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canEdit": {
wU8(12)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "question": {
wU8(12)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEmote": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "direction": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isVideo": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "outcome": {
wU8(2)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "durationMs": {
wU8(3)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sentAt": {
wU8(17)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isOwn": {
wU8(18)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
}
break
}
case "thread": {
wU8(21)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "replyTo": {
wU8(22)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEdited": {
wU8(23)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "sendStatus": {
wU8(25)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 13) return false
iv.setUint8(at2, n2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "thread": {
wU8(2)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
r2++
const f2 = f1[k2]
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "unreadCount": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "mentionCount": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (a2 === undefined) {
wU8(0)
} else if (a2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "key": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderName": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "reactedAt": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "message": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderId": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderAvatarUrl": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isFromBot": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "text": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "html": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "undecryptable": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "notice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "attachment": {
wU8(9)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "location": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "lat": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "lon": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "poll": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "question": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "answers": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "maxSelections": {
wU8(2)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isDisclosed": {
wU8(3)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isAnonymous": {
wU8(4)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "endedAt": {
wU8(5)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "votes": {
wU8(6)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "voters": {
wU8(7)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "totalVotes": {
wU8(8)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "ownAnswerIds": {
wU8(9)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canVote": {
wU8(10)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canEnd": {
wU8(11)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "canEdit": {
wU8(12)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "question": {
wU8(12)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEmote": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f2 === null || typeof f2 !== 'object' || Array.isArray(f2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in f2) {
switch (k3) {
case "direction": {
wU8(0)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "isVideo": {
wU8(1)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "outcome": {
wU8(2)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
case "durationMs": {
wU8(3)
const f3 = f2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sentAt": {
wU8(17)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isOwn": {
wU8(18)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
}
break
}
case "thread": {
wU8(21)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "replyTo": {
wU8(22)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEdited": {
wU8(23)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "sendStatus": {
wU8(25)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 13) return false
iv.setUint8(at2, n2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
}
if (a3 === undefined) {
wU8(0)
} else if (a3 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(a3)) return false
wU8(1)
wU32(a3.length)
for (let i0 = 0; i0 < a3.length; i0++) {
const v0 = a3[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "eventId": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "callerId": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "callerName": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "callerAvatarUrl": {
wU8(5)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "calledAt": {
wU8(6)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "canCallBack": {
wU8(7)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 8) return false
iv.setUint8(at1, n1)
}
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["roomId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["roomName"] = v2
break
}
case 2: {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["id"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["senderId"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["senderName"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["senderAvatarUrl"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["isFromBot"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["text"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["html"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["undecryptable"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["notice"] = v3
break
}
case 9: {
let v3
v3 = rDyn()
o3["attachment"] = v3
break
}
case 10: {
let v3
v3 = rDyn()
o3["location"] = v3
break
}
case 11: {
let v3
v3 = rDyn()
o3["poll"] = v3
break
}
case 12: {
let v3
v3 = rDyn()
o3["question"] = v3
break
}
case 13: {
let v3
v3 = rDyn()
o3["isEmote"] = v3
break
}
case 14: {
let v3
v3 = rDyn()
o3["mentionsRoom"] = v3
break
}
case 15: {
let v3
v3 = rDyn()
o3["isCallThreadRoot"] = v3
break
}
case 16: {
let v3
v3 = rDyn()
o3["call"] = v3
break
}
case 17: {
let v3
v3 = rDyn()
o3["sentAt"] = v3
break
}
case 18: {
let v3
v3 = rDyn()
o3["isOwn"] = v3
break
}
case 19: {
let v3
v3 = rDyn()
o3["isReadByOthers"] = v3
break
}
case 20: {
let v3
v3 = rDyn()
o3["lastReadBy"] = v3
break
}
case 21: {
let v3
v3 = rDyn()
o3["thread"] = v3
break
}
case 22: {
let v3
v3 = rDyn()
o3["replyTo"] = v3
break
}
case 23: {
let v3
v3 = rDyn()
o3["isEdited"] = v3
break
}
case 24: {
let v3
v3 = rDyn()
o3["reactions"] = v3
break
}
case 25: {
let v3
v3 = rDyn()
o3["sendStatus"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2["message"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["mentions"] = v0
break
}
case 1: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["roomId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["roomName"] = v2
break
}
case 2: {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["threadId"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["title"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["replyCount"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["lastReply"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["unread"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["isParticipating"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["isMuted"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["isResolved"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["lastOwnReplyAt"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2["thread"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["threads"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["roomId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["roomName"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["key"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["senderId"] = v2
break
}
case 4: {
let v2
v2 = rStr()
o2["senderName"] = v2
break
}
case 5: {
let v2
v2 = rF64()
o2["reactedAt"] = v2
break
}
case 6: {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["id"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["senderId"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["senderName"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["senderAvatarUrl"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["isFromBot"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["text"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["html"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["undecryptable"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["notice"] = v3
break
}
case 9: {
let v3
v3 = rDyn()
o3["attachment"] = v3
break
}
case 10: {
let v3
v3 = rDyn()
o3["location"] = v3
break
}
case 11: {
let v3
v3 = rDyn()
o3["poll"] = v3
break
}
case 12: {
let v3
v3 = rDyn()
o3["question"] = v3
break
}
case 13: {
let v3
v3 = rDyn()
o3["isEmote"] = v3
break
}
case 14: {
let v3
v3 = rDyn()
o3["mentionsRoom"] = v3
break
}
case 15: {
let v3
v3 = rDyn()
o3["isCallThreadRoot"] = v3
break
}
case 16: {
let v3
v3 = rDyn()
o3["call"] = v3
break
}
case 17: {
let v3
v3 = rDyn()
o3["sentAt"] = v3
break
}
case 18: {
let v3
v3 = rDyn()
o3["isOwn"] = v3
break
}
case 19: {
let v3
v3 = rDyn()
o3["isReadByOthers"] = v3
break
}
case 20: {
let v3
v3 = rDyn()
o3["lastReadBy"] = v3
break
}
case 21: {
let v3
v3 = rDyn()
o3["thread"] = v3
break
}
case 22: {
let v3
v3 = rDyn()
o3["replyTo"] = v3
break
}
case 23: {
let v3
v3 = rDyn()
o3["isEdited"] = v3
break
}
case 24: {
let v3
v3 = rDyn()
o3["reactions"] = v3
break
}
case 25: {
let v3
v3 = rDyn()
o3["sendStatus"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2["message"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["reactions"] = v0
break
}
case 3: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["roomId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["roomName"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["eventId"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["callerId"] = v2
break
}
case 4: {
let v2
v2 = rStr()
o2["callerName"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["callerAvatarUrl"] = v2
break
}
case 6: {
let v2
v2 = rF64()
o2["calledAt"] = v2
break
}
case 7: {
let v2
v2 = rU8() === 1
o2["canCallBack"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["missedCalls"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.buildActivity = function buildActivity (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `buildFeed`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.buildFeed
const call = native.buildFeedWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === undefined) {
wU8(0)
} else if (a0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "mentions": {
wU8(0)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "roomId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "roomName": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "message": {
wU8(2)
const f2 = v1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "threads": {
wU8(1)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "roomId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "roomName": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "thread": {
wU8(2)
const f2 = v1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "reactions": {
wU8(2)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "roomId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "roomName": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "key": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderId": {
wU8(3)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderName": {
wU8(4)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "reactedAt": {
wU8(5)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "message": {
wU8(6)
const f2 = v1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "missedCalls": {
wU8(3)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "roomId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "roomName": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "eventId": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "callerId": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "callerName": {
wU8(4)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "callerAvatarUrl": {
wU8(5)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "calledAt": {
wU8(6)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "canCallBack": {
wU8(7)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 8) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "unreadCount": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastMessage": {
wU8(4)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEmote": {
wU8(1)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderId": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderName": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isOwn": {
wU8(6)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "sentAt": {
wU8(7)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "tick": {
wU8(8)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 4) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomName": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "message": {
wU8(1)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderId": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderAvatarUrl": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isFromBot": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "text": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "html": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "undecryptable": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "notice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "attachment": {
wU8(9)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "location": {
wU8(10)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "poll": {
wU8(11)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "question": {
wU8(12)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEmote": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "sentAt": {
wU8(17)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isOwn": {
wU8(18)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
}
break
}
case "thread": {
wU8(21)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "replyTo": {
wU8(22)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isEdited": {
wU8(23)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "sendStatus": {
wU8(25)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 12) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["kind"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["roomName"] = v1
break
}
case 4: {
let v1
v1 = rF64()
o1["at"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorName"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorAvatarUrl"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["quote"] = v1
break
}
case 8: {
let v1
v1 = rF64()
o1["count"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isMarkedUnread"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["threadTitle"] = v1
break
}
case 12: {
let v1
v1 = rU8() === 1
o1["isResolved"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["canCallBack"] = v1
break
}
case 14: {
let v1
v1 = rDyn()
o1["target"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.buildFeed = function buildFeed (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `filterFeed`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.filterFeed
const call = native.filterFeedWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomId": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "at": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "actorName": {
wU8(5)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "actorAvatarUrl": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "quote": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "count": {
wU8(8)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(9)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(10)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "threadTitle": {
wU8(11)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isResolved": {
wU8(12)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canCallBack": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "target": {
wU8(14)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 12) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["kind"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["roomName"] = v1
break
}
case 4: {
let v1
v1 = rF64()
o1["at"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorName"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorAvatarUrl"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["quote"] = v1
break
}
case 8: {
let v1
v1 = rF64()
o1["count"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isMarkedUnread"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["threadTitle"] = v1
break
}
case 12: {
let v1
v1 = rU8() === 1
o1["isResolved"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["canCallBack"] = v1
break
}
case 14: {
let v1
v1 = rDyn()
o1["target"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.filterFeed = function filterFeed (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `searchFeed`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.searchFeed
const call = native.searchFeedWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomId": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "at": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "actorName": {
wU8(5)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "actorAvatarUrl": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "quote": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "count": {
wU8(8)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(9)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(10)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "threadTitle": {
wU8(11)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isResolved": {
wU8(12)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canCallBack": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "target": {
wU8(14)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 12) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["kind"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["roomName"] = v1
break
}
case 4: {
let v1
v1 = rF64()
o1["at"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorName"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["actorAvatarUrl"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["quote"] = v1
break
}
case 8: {
let v1
v1 = rF64()
o1["count"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isMarkedUnread"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["threadTitle"] = v1
break
}
case 12: {
let v1
v1 = rU8() === 1
o1["isResolved"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["canCallBack"] = v1
break
}
case 14: {
let v1
v1 = rDyn()
o1["target"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.searchFeed = function searchFeed (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `groupByDay`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.groupByDay
const call = native.groupByDayWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomId": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "roomName": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "at": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "actorName": {
wU8(5)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "actorAvatarUrl": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "quote": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "count": {
wU8(8)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(9)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(10)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "threadTitle": {
wU8(11)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isResolved": {
wU8(12)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canCallBack": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "target": {
wU8(14)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 12) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rDyn()
o1["day"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["id"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["kind"] = v3
break
}
case 2: {
let v3
v3 = rStr()
o3["roomId"] = v3
break
}
case 3: {
let v3
v3 = rStr()
o3["roomName"] = v3
break
}
case 4: {
let v3
v3 = rF64()
o3["at"] = v3
break
}
case 5: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["actorName"] = v3
break
}
case 6: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["actorAvatarUrl"] = v3
break
}
case 7: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["quote"] = v3
break
}
case 8: {
let v3
v3 = rF64()
o3["count"] = v3
break
}
case 9: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["isMarkedUnread"] = v3
break
}
case 10: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["emoji"] = v3
break
}
case 11: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["threadTitle"] = v3
break
}
case 12: {
let v3
v3 = rU8() === 1
o3["isResolved"] = v3
break
}
case 13: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["canCallBack"] = v3
break
}
case 14: {
let v3
v3 = rDyn()
o3["target"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["items"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.groupByDay = function groupByDay (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `assistantConsentStatus`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.assistantConsentStatus
const call = native.assistantConsentStatusWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "requests": {
wU8(0)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "requesterId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "at": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "answers": {
wU8(1)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "userId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "accepted": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "at": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.assistantConsentStatus = function assistantConsentStatus (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `requestsWaitingFor`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.requestsWaitingFor
const call = native.requestsWaitingForWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "requests": {
wU8(0)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "requesterId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "at": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "answers": {
wU8(1)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "userId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "accepted": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "at": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["assistantId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["requesterId"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["at"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.requestsWaitingFor = function requestsWaitingFor (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `peopleOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.peopleOf
const call = native.peopleOfWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isBot": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "requests": {
wU8(0)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "requesterId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "at": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "answers": {
wU8(1)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "userId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "accepted": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "at": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.peopleOf = function peopleOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `presentRequest`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.presentRequest
const call = native.presentRequestWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "requests": {
wU8(0)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "requesterId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "at": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "answers": {
wU8(1)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "userId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "accepted": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "at": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isInvited": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["assistantId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["requesterId"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["at"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.presentRequest = function presentRequest (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findOpenQuestion`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findOpenQuestion
const call = native.findOpenQuestionWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "senderId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "text": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "sentAt": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "unread": {
wU8(4)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "mentionCount": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
if (typeof a3 !== 'number') return false
wF64(a3)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["messageId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["questionId"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["closesAt"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["text"] = v1
break
}
case 4: {
let v1
v1 = rF64()
o1["sentAt"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["title"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["roomId"] = v3
break
}
case 1: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v3 = a4
}
o3["withIds"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["startTs"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["endTs"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["meeting"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.findOpenQuestion = function findOpenQuestion (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeWaveform`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeWaveform
const call = native.computeWaveformWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rF64()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.computeWaveform = function computeWaveform (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isPlayableAudio`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isPlayableAudio
const call = native.isPlayableAudioWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "kind": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "mimeType": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "size": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "isEncrypted": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "width": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "height": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "duration": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "hasThumbnail": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "waveform": {
wU8(0)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'number') return false
wF64(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
break
}
case "batch": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 7) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isPlayableAudio = function isPlayableAudio (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isViewablePdf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isViewablePdf
const call = native.isViewablePdfWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "kind": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "mimeType": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "size": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "isEncrypted": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "width": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "height": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "duration": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "hasThumbnail": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "waveform": {
wU8(0)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'number') return false
wF64(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
break
}
case "batch": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 7) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'boolean') return false
wU8(a1 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isViewablePdf = function isViewablePdf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isPreviewable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isPreviewable
const call = native.isPreviewableWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "kind": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "mimeType": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "size": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "isEncrypted": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "width": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "height": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "duration": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "hasThumbnail": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "waveform": {
wU8(0)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'number') return false
wF64(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
break
}
case "batch": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 7) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isPreviewable = function isPreviewable (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isGif`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isGif
const call = native.isGifWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "mimeType": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isGif = function isGif (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `svgCanvasSize`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.svgCanvasSize
const call = native.svgCanvasSizeWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'number') return false
wF64(a0)
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.svgCanvasSize = function svgCanvasSize (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `thumbnailSize`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.thumbnailSize
const call = native.thumbnailSizeWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "mimeType": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "size": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["width"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["height"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.thumbnailSize = function thumbnailSize (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `botOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.botOf
const call = native.botOfWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["userId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["deviceId"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["masterKey"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.botOf = function botOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findCommandQuery`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findCommandQuery
const call = native.findCommandQueryWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["start"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["query"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.findCommandQuery = function findCommandQuery (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `matchCommands`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.matchCommands
const call = native.matchCommandsWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "botUserId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "syntax": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "description": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "argumentNames": {
wU8(4)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "isLastArgumentVariadic": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["botUserId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["syntax"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["description"] = v1
break
}
case 4: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["argumentNames"] = v1
break
}
case 5: {
let v1
v1 = rU8() === 1
o1["isLastArgumentVariadic"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.matchCommands = function matchCommands (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findLocalCommand`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findLocalCommand
const call = native.findLocalCommandWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.findLocalCommand = function findLocalCommand (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `prepareCommandText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.prepareCommandText
const call = native.prepareCommandTextWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "botUserId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "syntax": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "description": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "argumentNames": {
wU8(4)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "isLastArgumentVariadic": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["text"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["botUserId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["syntax"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["description"] = v2
break
}
case 4: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v2 = a3
}
o2["argumentNames"] = v2
break
}
case 5: {
let v2
v2 = rU8() === 1
o2["isLastArgumentVariadic"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["command"] = v0
break
}
case 2: {
let v0
v0 = rU8() === 1
o0["isEmote"] = v0
break
}
case 3: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["unknownCommand"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.prepareCommandText = function prepareCommandText (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `prepareOutgoing`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.prepareOutgoing
const call = native.prepareOutgoingWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "botUserId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "syntax": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "description": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "argumentNames": {
wU8(4)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "isLastArgumentVariadic": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["error"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["text"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["userId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["mentions"] = v0
break
}
case 3: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["botUserId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["syntax"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["description"] = v2
break
}
case 4: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v2 = a3
}
o2["argumentNames"] = v2
break
}
case 5: {
let v2
v2 = rU8() === 1
o2["isLastArgumentVariadic"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["command"] = v0
break
}
case 4: {
let v0
v0 = rU8() === 1
o0["isEmote"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.prepareOutgoing = function prepareOutgoing (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `shouldMirror`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.shouldMirror
const call = native.shouldMirrorWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.shouldMirror = function shouldMirror (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `currentDeviceId`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.currentDeviceId
const call = native.currentDeviceIdWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "label": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (a1 === undefined) {
return false
wU8(0)
} else if (a1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.currentDeviceId = function currentDeviceId (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `countCallPeople`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.countCallPeople
const call = native.countCallPeopleWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "deviceId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "joinedAt": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.countCallPeople = function countCallPeople (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `ringAsks`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.ringAsks
const call = native.ringAsksWire
const encode = (a0, a1, a2, a3, a4) => {
at = 0
if (a0 === undefined) {
wU8(0)
} else if (a0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "eventId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "until": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isDeclined": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "deviceId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "joinedAt": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (typeof a2 !== 'string') return false
wStr(a2)
if (typeof a3 !== 'boolean') return false
wU8(a3 ? 1 : 0)
if (typeof a4 !== 'number') return false
wF64(a4)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.ringAsks = function ringAsks (...a) {
  if (a.length <= 5 && encode(a[0], a[1], a[2], a[3], a[4]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRinging`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRinging
const call = native.isRingingWire
const encode = (a0, a1, a2, a3, a4) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "deviceId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "joinedAt": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
if (typeof a3 !== 'boolean') return false
wU8(a3 ? 1 : 0)
if (a4 === undefined) {
wU8(0)
} else if (a4 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a4 !== 'number') return false
wF64(a4)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isRinging = function isRinging (...a) {
  if (a.length <= 5 && encode(a[0], a[1], a[2], a[3], a[4]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findCallThread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findCallThread
const call = native.findCallThreadWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (!wDyn(v0)) return false
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.findCallThread = function findCallThread (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `toCallLine`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.toCallLine
const call = native.toCallLineWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'boolean') return false
wU8(a1 ? 1 : 0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["direction"] = v1
break
}
case 1: {
let v1
v1 = rU8() === 1
o1["isVideo"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["outcome"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["durationMs"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.toCallLine = function toCallLine (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `fitWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.fitWindow
const call = native.fitWindowWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "x": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "y": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "width": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["x"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["y"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 3: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.fitWindow = function fitWindow (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `defaultCallWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.defaultCallWindow
const call = native.defaultCallWindowWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["x"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["y"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 3: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.defaultCallWindow = function defaultCallWindow (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `moveWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.moveWindow
const call = native.moveWindowWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "x": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "y": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "width": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'number') return false
wF64(a1)
if (typeof a2 !== 'number') return false
wF64(a2)
if (a3 === null || typeof a3 !== 'object' || Array.isArray(a3)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a3) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a3[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a3[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["x"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["y"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 3: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.moveWindow = function moveWindow (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `placeInCorner`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.placeInCorner
const call = native.placeInCornerWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "x": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "y": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "width": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (a2 === null || typeof a2 !== 'object' || Array.isArray(a2)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a2) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["x"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["y"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 3: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.placeInCorner = function placeInCorner (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `resizeWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.resizeWindow
const call = native.resizeWindowWire
const encode = (a0, a1, a2, a3, a4) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "x": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "y": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "width": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'number') return false
wF64(a2)
if (typeof a3 !== 'number') return false
wF64(a3)
if (a4 === null || typeof a4 !== 'object' || Array.isArray(a4)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a4) {
switch (k0) {
case "width": {
wU8(0)
r0++
const f0 = a4[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "height": {
wU8(1)
r0++
const f0 = a4[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["x"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["y"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 3: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.resizeWindow = function resizeWindow (...a) {
  if (a.length <= 5 && encode(a[0], a[1], a[2], a[3], a[4]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseCallWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseCallWindow
const call = native.parseCallWindowWire
const encode = (a0) => {
at = 0
if (a0 === undefined) {
return false
wU8(0)
} else if (a0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a0 !== 'string') return false
wStr(a0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["x"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["y"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["width"] = v1
break
}
case 3: {
let v1
v1 = rF64()
o1["height"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.parseCallWindow = function parseCallWindow (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `placeCall`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.placeCall
const call = native.placeCallWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "detachedRoomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "shownRoomId": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "shownRoomId": {
wU8(0)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "callRoomId": {
wU8(1)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["detachedRoomId"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["shownRoomId"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.placeCall = function placeCall (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isCallFloating`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isCallFloating
const call = native.isCallFloatingWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "detachedRoomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "shownRoomId": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (a1 === undefined) {
return false
wU8(0)
} else if (a1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isCallFloating = function isCallFloating (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseClientConfig`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseClientConfig
const call = native.parseClientConfigWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["applicationName"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["welcomeMessage"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["homeserverUrl"] = v0
break
}
case 3: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["twakeWorkplaceHomeserverUrl"] = v0
break
}
case 4: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["privacyUrl"] = v0
break
}
case 5: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["registrationUrl"] = v0
break
}
case 6: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["supportUrl"] = v0
break
}
case 7: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["qrCodeDownloadUrl"] = v0
break
}
case 8: {
let v0
v0 = rU8() === 1
o0["renderHtml"] = v0
break
}
case 9: {
let v0
v0 = rU8() === 1
o0["hideRedactedEvents"] = v0
break
}
case 10: {
let v0
v0 = rU8() === 1
o0["hideUnknownEvents"] = v0
break
}
case 11: {
let v0
v0 = rU8() === 1
o0["appGridAvailable"] = v0
break
}
case 12: {
let v0
v0 = rU8() === 1
o0["enableLogs"] = v0
break
}
case 13: {
let v0
v0 = rU8() === 1
o0["webPushEnabled"] = v0
break
}
case 14: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["vapidPublicKey"] = v0
break
}
case 15: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["pushGatewayUrl"] = v0
break
}
case 16: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["pushAppId"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.parseClientConfig = function parseClientConfig (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readComposerStyle`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readComposerStyle
const call = native.readComposerStyleWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.readComposerStyle = function readComposerStyle (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseInvitationTarget`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseInvitationTarget
const call = native.parseInvitationTargetWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["contact"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["medium"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.parseInvitationTarget = function parseInvitationTarget (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `groupByInitial`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.groupByInitial
const call = native.groupByInitialWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "displayName": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["letter"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["displayName"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["people"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.groupByInitial = function groupByInitial (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseContactsVisibility`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseContactsVisibility
const call = native.parseContactsVisibilityWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.parseContactsVisibility = function parseContactsVisibility (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `emptyCardOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.emptyCardOf
const call = native.emptyCardOfWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isLanding": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isWaiting": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "hasView": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isEmpty": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "room": {
wU8(4)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmbedded": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isAssistantRoom": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isPeerBlocked": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "mayPost": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 8) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.emptyCardOf = function emptyCardOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `footerOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.footerOf
const call = native.footerOfWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "hasView": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isAbsent": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isLanding": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isPeerGone": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isPeerBlocked": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "peerId": {
wU8(5)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "mayPost": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isRecording": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "replacedBy": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 9) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.footerOf = function footerOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `timelineHeadOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.timelineHeadOf
const call = native.timelineHeadOfWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isLanding": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "history": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "hasOlderMessages": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "deletedBefore": {
wU8(1)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "messageCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "predecessor": {
wU8(2)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isAbsent": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isSettled": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOutside": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isWaiting": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "hasTimedOut": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 8) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rU8() === 1
o0["hasOlder"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rF64()
}
}
o0["deletedBefore"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["predecessor"] = v0
break
}
case 3: {
let v0
v0 = rU8() === 1
o0["isUnavailable"] = v0
break
}
case 4: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["loading"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.timelineHeadOf = function timelineHeadOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `headOverWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.headOverWindow
const call = native.headOverWindowWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "hasOlder": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "deletedBefore": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "predecessor": {
wU8(2)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isUnavailable": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "loading": {
wU8(4)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 5) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rU8() === 1
o0["hasOlder"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rF64()
}
}
o0["deletedBefore"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["predecessor"] = v0
break
}
case 3: {
let v0
v0 = rU8() === 1
o0["isUnavailable"] = v0
break
}
case 4: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["loading"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.headOverWindow = function headOverWindow (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isLandingRoom`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isLandingRoom
const call = native.isLandingRoomWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'boolean') return false
wU8(a1 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isLandingRoom = function isLandingRoom (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `mayPostIn`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.mayPostIn
const call = native.mayPostInWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "hasView": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isLanding": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "replacedBy": {
wU8(2)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "room": {
wU8(3)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isSpace": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hasName": {
wU8(5)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "alias": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isEncrypted": {
wU8(10)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "gifsOff": {
wU8(16)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isPeerGone": {
wU8(20)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "parentSpaceIds": {
wU8(22)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
break
}
case "ownPowerLevel": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.mayPostIn = function mayPostIn (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isDirectRoom`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isDirectRoom
const call = native.isDirectRoomWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isDirectRoom = function isDirectRoom (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `peerStateOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.peerStateOf
const call = native.peerStateOfWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isDirect": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "members": {
wU8(1)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (!wDyn(v1)) return false
}
break
}
case "hasMembers": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownUserId": {
wU8(3)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "ignoredUsers": {
wU8(4)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rDyn()
o0["peer"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["peerId"] = v0
break
}
case 2: {
let v0
v0 = rU8() === 1
o0["isPeerBlocked"] = v0
break
}
case 3: {
let v0
v0 = rU8() === 1
o0["isPeerGone"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.peerStateOf = function peerStateOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `splitsCallChat`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.splitsCallChat
const call = native.splitsCallChatWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isInCall": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isDirect": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isVoice": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "mayPost": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.splitsCallChat = function splitsCallChat (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isEncryptedConversation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isEncryptedConversation
const call = native.isEncryptedConversationWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isEncryptedConversation = function isEncryptedConversation (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isAwaitingRoom`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isAwaitingRoom
const call = native.isAwaitingRoomWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isOutside": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isPending": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "hasRoom": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isSettled": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "messageCount": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isSyncReady": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isLanding": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 7) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isAwaitingRoom = function isAwaitingRoom (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `roomNameOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.roomNameOf
const call = native.roomNameOfWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === undefined) {
wU8(0)
} else if (v0 === null) {
wU8(2)
} else {
wU8(1)
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 0) return false
iv.setUint8(at1, n1)
}
}
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.roomNameOf = function roomNameOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `assistantOfferIn`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.assistantOfferIn
const call = native.assistantOfferInWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isEncrypted": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "myBotId": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "permissions": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "canInvite": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "defaultLevel": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "canRedact": {
wU8(6)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "canMentionEveryone": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 16) return false
iv.setUint8(at1, n1)
}
}
break
}
case "members": {
wU8(3)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isBot": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isInvited": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "consent": {
wU8(4)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "requests": {
wU8(0)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "assistantId": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "requesterId": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "at": {
wU8(2)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "assistantId": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "userId": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "accepted": {
wU8(2)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "at": {
wU8(3)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.assistantOfferIn = function assistantOfferIn (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `hasAssistantIn`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hasAssistantIn
const call = native.hasAssistantInWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "requests": {
wU8(0)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "requesterId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "at": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "answers": {
wU8(1)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "assistantId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "userId": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "accepted": {
wU8(2)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "at": {
wU8(3)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isInvited": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.hasAssistantIn = function hasAssistantIn (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `offersAddContact`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.offersAddContact
const call = native.offersAddContactWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isDirect": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "peerId": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "ownUserId": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerBlocked": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isEmbedded": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isAssistantRoom": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "hasView": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.offersAddContact = function offersAddContact (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `matchPlaceOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.matchPlaceOf
const call = native.matchPlaceOfWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
if (a1 === undefined) {
return false
wU8(0)
} else if (a1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["index"] = v0
break
}
case 1: {
let v0
v0 = rU8() === 1
o0["hasOlder"] = v0
break
}
case 2: {
let v0
v0 = rU8() === 1
o0["hasNewer"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.matchPlaceOf = function matchPlaceOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readDeletedChats`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readDeletedChats
const call = native.readDeletedChatsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.readDeletedChats = function readDeletedChats (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isChatHidden`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isChatHidden
const call = native.isChatHiddenWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'number') return false
wF64(a2)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isChatHidden = function isChatHidden (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withoutDeleted`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withoutDeleted
const call = native.withoutDeletedWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["senderId"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["senderName"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["senderAvatarUrl"] = v1
break
}
case 4: {
let v1
v1 = rU8() === 1
o1["isFromBot"] = v1
break
}
case 5: {
let v1
v1 = rStr()
o1["text"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["html"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["undecryptable"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["notice"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["kind"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["name"] = v3
break
}
case 2: {
let v3
v3 = rStr()
o3["mimeType"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["size"] = v3
break
}
case 4: {
let v3
v3 = rU8() === 1
o3["isEncrypted"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["dimensions"] = v3
break
}
case 6: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["duration"] = v3
break
}
case 7: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["hasThumbnail"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["voice"] = v3
break
}
case 9: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["batch"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["attachment"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rF64()
o3["lat"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["lon"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["location"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["question"] = v3
break
}
case 1: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rDyn()
a4[i4] = v4
}
v3 = a4
}
o3["answers"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["maxSelections"] = v3
break
}
case 3: {
let v3
v3 = rU8() === 1
o3["isDisclosed"] = v3
break
}
case 4: {
let v3
v3 = rU8() === 1
o3["isAnonymous"] = v3
break
}
case 5: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["endedAt"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["votes"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["voters"] = v3
break
}
case 8: {
let v3
v3 = rF64()
o3["totalVotes"] = v3
break
}
case 9: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v3 = a4
}
o3["ownAnswerIds"] = v3
break
}
case 10: {
let v3
v3 = rU8() === 1
o3["canVote"] = v3
break
}
case 11: {
let v3
v3 = rU8() === 1
o3["canEnd"] = v3
break
}
case 12: {
let v3
v3 = rU8() === 1
o3["canEdit"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["poll"] = v1
break
}
case 12: {
let v1
v1 = rDyn()
o1["question"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isEmote"] = v1
break
}
case 14: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["mentionsRoom"] = v1
break
}
case 15: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isCallThreadRoot"] = v1
break
}
case 16: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["direction"] = v3
break
}
case 1: {
let v3
v3 = rU8() === 1
o3["isVideo"] = v3
break
}
case 2: {
let v3
v3 = rStr()
o3["outcome"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["durationMs"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["call"] = v1
break
}
case 17: {
let v1
v1 = rF64()
o1["sentAt"] = v1
break
}
case 18: {
let v1
v1 = rU8() === 1
o1["isOwn"] = v1
break
}
case 19: {
let v1
v1 = rU8() === 1
o1["isReadByOthers"] = v1
break
}
case 20: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["userId"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["name"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rStr()
}
}
o4["avatarUrl"] = v4
break
}
case 3: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["readAt"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["lastReadBy"] = v1
break
}
case 21: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["threadId"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["title"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["replyCount"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["lastReply"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["unread"] = v3
break
}
case 5: {
let v3
v3 = rU8() === 1
o3["isParticipating"] = v3
break
}
case 6: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["isMuted"] = v3
break
}
case 7: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["isResolved"] = v3
break
}
case 8: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["lastOwnReplyAt"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["thread"] = v1
break
}
case 22: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["messageId"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["senderName"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["senderId"] = v3
break
}
case 3: {
let v3
v3 = rStr()
o3["text"] = v3
break
}
case 4: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["isDeleted"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["replyTo"] = v1
break
}
case 23: {
let v1
v1 = rU8() === 1
o1["isEdited"] = v1
break
}
case 24: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["key"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["count"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["ownReactionId"] = v3
break
}
case 3: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rDyn()
a4[i4] = v4
}
v3 = a4
}
o3["senders"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["reactions"] = v1
break
}
case 25: {
let v1
v1 = rStr()
o1["sendStatus"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.withoutDeleted = function withoutDeleted (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isAfterDeletion`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isAfterDeletion
const call = native.isAfterDeletionWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'number') return false
wF64(a2)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isAfterDeletion = function isAfterDeletion (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readDismissedQuestions`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readDismissedQuestions
const call = native.readDismissedQuestionsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.readDismissedQuestions = function readDismissedQuestions (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withDismissedQuestion`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withDismissedQuestion
const call = native.withDismissedQuestionWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'number') return false
wF64(a2)
if (typeof a3 !== 'number') return false
wF64(a3)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.withDismissedQuestion = function withDismissedQuestion (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findEmojiQuery`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findEmojiQuery
const call = native.findEmojiQueryWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["start"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["query"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.findEmojiQuery = function findEmojiQuery (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isEmojiDrawable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isEmojiDrawable
const call = native.isEmojiDrawableWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!wDyn(a1)) return false
if (a2 === undefined) {
wU8(0)
} else if (a2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isEmojiDrawable = function isEmojiDrawable (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `hasInk`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hasInk
const call = native.hasInkWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.hasInk = function hasInk (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `hasColour`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hasColour
const call = native.hasColourWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.hasColour = function hasColour (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isDrawn`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isDrawn
const call = native.isDrawnWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isDrawn = function isDrawn (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isNotificationService`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isNotificationService
const call = native.isNotificationServiceWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isNotificationService = function isNotificationService (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readFolders`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readFolders
const call = native.readFoldersWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 1: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["builtin"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hidden"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["color"] = v1
break
}
case 5: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["include"] = v1
break
}
case 6: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["exclude"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readFolders = function readFolders (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `writeFolders`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.writeFolders
const call = native.writeFoldersWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "builtin": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hidden": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "color": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "include": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "exclude": {
wU8(6)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rDyn()
a1[i1] = v1
}
v0 = a1
}
o0["folders"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.writeFolders = function writeFolders (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withHiddenBuiltins`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withHiddenBuiltins
const call = native.withHiddenBuiltinsWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "builtin": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hidden": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "color": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "include": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "exclude": {
wU8(6)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 1: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["builtin"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hidden"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["color"] = v1
break
}
case 5: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["include"] = v1
break
}
case 6: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["exclude"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.withHiddenBuiltins = function withHiddenBuiltins (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `listFolders`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.listFolders
const call = native.listFoldersWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "builtin": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hidden": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "color": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "include": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "exclude": {
wU8(6)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "room": {
wU8(0)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "folders": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 0) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 1: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["builtin"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hidden"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["emoji"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["color"] = v1
break
}
case 5: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["include"] = v1
break
}
case 6: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["exclude"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.listFolders = function listFolders (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `folderChatTypeOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.folderChatTypeOf
const call = native.folderChatTypeOfWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.folderChatTypeOf = function folderChatTypeOf (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isInFolder`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isInFolder
const call = native.isInFolderWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "room": {
wU8(0)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "folders": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 0) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "name": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "builtin": {
wU8(1)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hidden": {
wU8(2)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "color": {
wU8(4)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "include": {
wU8(5)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
case "exclude": {
wU8(6)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isInFolder = function isInFolder (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRecommendationTaken`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRecommendationTaken
const call = native.isRecommendationTakenWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "builtin": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hidden": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "color": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "include": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "exclude": {
wU8(6)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isRecommendationTaken = function isRecommendationTaken (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `fitPreview`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.fitPreview
const call = native.fitPreviewWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'number') return false
wF64(a1)
if (a2 === undefined) {
wU8(0)
} else if (a2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'number') return false
wF64(a2)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["width"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["height"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.fitPreview = function fitPreview (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRecord`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRecord
const call = native.isRecordWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.isRecord = function isRecord (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readText
const call = native.readTextWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const k0 = Object.keys(a0)
wU8(1)
wU32(k0.length)
for (const key0 of k0) {
wStr(key0)
const v0 = a0[key0]
if (!wDyn(v0)) return false
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.readText = function readText (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readTexts`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readTexts
const call = native.readTextsWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const k0 = Object.keys(a0)
wU8(1)
wU32(k0.length)
for (const key0 of k0) {
wStr(key0)
const v0 = a0[key0]
if (!wDyn(v0)) return false
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readTexts = function readTexts (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readBoolean`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readBoolean
const call = native.readBooleanWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const k0 = Object.keys(a0)
wU8(1)
wU32(k0.length)
for (const key0 of k0) {
wStr(key0)
const v0 = a0[key0]
if (!wDyn(v0)) return false
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.readBoolean = function readBoolean (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `autoTranslationOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.autoTranslationOf
const call = native.autoTranslationOfWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
if (a1 === undefined) {
return false
wU8(0)
} else if (a1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.autoTranslationOf = function autoTranslationOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseGeoUri`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseGeoUri
const call = native.parseGeoUriWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["lat"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["lon"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.parseGeoUri = function parseGeoUri (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `openStreetMapUrl`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.openStreetMapUrl
const call = native.openStreetMapUrlWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "lat": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "lon": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.openStreetMapUrl = function openStreetMapUrl (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `pickAttendees`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.pickAttendees
const call = native.pickAttendeesWire
const encode = (a0, a1, a2) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isBot": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "displayName": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "firstName": {
wU8(2)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "lastName": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "emails": {
wU8(4)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "phones": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "avatarUrl": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "contactId": {
wU8(7)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
if (typeof a2 !== 'string') return false
wStr(a2)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
v0 = a1
}
o0["emails"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["missingCount"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.pickAttendees = function pickAttendees (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readCreatedMeeting`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readCreatedMeeting
const call = native.readCreatedMeetingWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["uid"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["title"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["start"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["end"] = v1
break
}
case 4: {
let v1
v1 = rU8() === 1
o1["allDay"] = v1
break
}
case 5: {
let v1
v1 = rStr()
o1["timeZone"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["videoConferenceUrl"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["url"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.readCreatedMeeting = function readCreatedMeeting (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `meetingMessageText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.meetingMessageText
const call = native.meetingMessageTextWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "uid": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "title": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "start": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "end": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "allDay": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "timeZone": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "videoConferenceUrl": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "url": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 8) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.meetingMessageText = function meetingMessageText (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `describePeople`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.describePeople
const call = native.describePeopleWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.describePeople = function describePeople (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `matchMembers`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.matchMembers
const call = native.matchMembersWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "matrixName": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (a3 === undefined) {
wU8(0)
} else if (a3 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a3 !== 'number') return false
wF64(a3)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["userId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["matrixName"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.matchMembers = function matchMembers (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `mentionText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.mentionText
const call = native.mentionTextWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "name": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.mentionText = function mentionText (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `keepPresentMentions`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.keepPresentMentions
const call = native.keepPresentMentionsWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "userId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["userId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.keepPresentMentions = function keepPresentMentions (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `matchRooms`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.matchRooms
const call = native.matchRoomsWire
const encode = (a0, a1, a2) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "alias": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (a2 === undefined) {
wU8(0)
} else if (a2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'number') return false
wF64(a2)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["alias"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.matchRooms = function matchRooms (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `roomMentionRef`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.roomMentionRef
const call = native.roomMentionRefWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "alias": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.roomMentionRef = function roomMentionRef (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `roomMentionText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.roomMentionText
const call = native.roomMentionTextWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "alias": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.roomMentionText = function roomMentionText (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRoomMention`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRoomMention
const call = native.isRoomMentionWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isRoomMention = function isRoomMention (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isEveryoneMention`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isEveryoneMention
const call = native.isEveryoneMentionWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isEveryoneMention = function isEveryoneMention (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `countsAsUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.countsAsUnread
const call = native.countsAsUnreadWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.countsAsUnread = function countsAsUnread (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findFirstUnreadId`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findFirstUnreadId
const call = native.findFirstUnreadIdWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.findFirstUnreadId = function findFirstUnreadId (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isSelectable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isSelectable
const call = native.isSelectableWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isSelectable = function isSelectable (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `startsGroup`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.startsGroup
const call = native.startsGroupWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.startsGroup = function startsGroup (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `groupAlbums`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.groupAlbums
const call = native.groupAlbumsWire
const encode = (a0, a1, a2) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
if (a2 === undefined) {
wU8(0)
} else if (a2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["id"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["senderId"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["senderName"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["senderAvatarUrl"] = v2
break
}
case 4: {
let v2
v2 = rU8() === 1
o2["isFromBot"] = v2
break
}
case 5: {
let v2
v2 = rStr()
o2["text"] = v2
break
}
case 6: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["html"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["undecryptable"] = v2
break
}
case 8: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["notice"] = v2
break
}
case 9: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["kind"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["name"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["mimeType"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["size"] = v4
break
}
case 4: {
let v4
v4 = rDyn()
o4["isEncrypted"] = v4
break
}
case 5: {
let v4
v4 = rDyn()
o4["dimensions"] = v4
break
}
case 6: {
let v4
v4 = rDyn()
o4["duration"] = v4
break
}
case 7: {
let v4
v4 = rDyn()
o4["hasThumbnail"] = v4
break
}
case 8: {
let v4
v4 = rDyn()
o4["voice"] = v4
break
}
case 9: {
let v4
v4 = rDyn()
o4["batch"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["attachment"] = v2
break
}
case 10: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rF64()
o4["lat"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["lon"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["location"] = v2
break
}
case 11: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["question"] = v4
break
}
case 1: {
let v4
if (rU8() === 0) v4 = null
else {
const n5 = rU32()
const a5 = new Array(n5)
for (let i5 = 0; i5 < n5; i5++) {
let v5
v5 = rDyn()
a5[i5] = v5
}
v4 = a5
}
o4["answers"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["maxSelections"] = v4
break
}
case 3: {
let v4
v4 = rU8() === 1
o4["isDisclosed"] = v4
break
}
case 4: {
let v4
v4 = rU8() === 1
o4["isAnonymous"] = v4
break
}
case 5: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["endedAt"] = v4
break
}
case 6: {
let v4
v4 = rDyn()
o4["votes"] = v4
break
}
case 7: {
let v4
v4 = rDyn()
o4["voters"] = v4
break
}
case 8: {
let v4
v4 = rF64()
o4["totalVotes"] = v4
break
}
case 9: {
let v4
if (rU8() === 0) v4 = null
else {
const n5 = rU32()
const a5 = new Array(n5)
for (let i5 = 0; i5 < n5; i5++) {
let v5
v5 = rDyn()
a5[i5] = v5
}
v4 = a5
}
o4["ownAnswerIds"] = v4
break
}
case 10: {
let v4
v4 = rU8() === 1
o4["canVote"] = v4
break
}
case 11: {
let v4
v4 = rU8() === 1
o4["canEnd"] = v4
break
}
case 12: {
let v4
v4 = rU8() === 1
o4["canEdit"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["poll"] = v2
break
}
case 12: {
let v2
v2 = rDyn()
o2["question"] = v2
break
}
case 13: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isEmote"] = v2
break
}
case 14: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["mentionsRoom"] = v2
break
}
case 15: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isCallThreadRoot"] = v2
break
}
case 16: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["direction"] = v4
break
}
case 1: {
let v4
v4 = rU8() === 1
o4["isVideo"] = v4
break
}
case 2: {
let v4
v4 = rStr()
o4["outcome"] = v4
break
}
case 3: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["durationMs"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["call"] = v2
break
}
case 17: {
let v2
v2 = rF64()
o2["sentAt"] = v2
break
}
case 18: {
let v2
v2 = rU8() === 1
o2["isOwn"] = v2
break
}
case 19: {
let v2
v2 = rU8() === 1
o2["isReadByOthers"] = v2
break
}
case 20: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rDyn()
o5["userId"] = v5
break
}
case 1: {
let v5
v5 = rDyn()
o5["name"] = v5
break
}
case 2: {
let v5
v5 = rDyn()
o5["avatarUrl"] = v5
break
}
case 3: {
let v5
v5 = rDyn()
o5["readAt"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v2 = a4
}
}
}
o2["lastReadBy"] = v2
break
}
case 21: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["threadId"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["title"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["replyCount"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["lastReply"] = v4
break
}
case 4: {
let v4
v4 = rDyn()
o4["unread"] = v4
break
}
case 5: {
let v4
v4 = rDyn()
o4["isParticipating"] = v4
break
}
case 6: {
let v4
v4 = rDyn()
o4["isMuted"] = v4
break
}
case 7: {
let v4
v4 = rDyn()
o4["isResolved"] = v4
break
}
case 8: {
let v4
v4 = rDyn()
o4["lastOwnReplyAt"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["thread"] = v2
break
}
case 22: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["messageId"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["senderName"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["senderId"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["text"] = v4
break
}
case 4: {
let v4
v4 = rDyn()
o4["isDeleted"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["replyTo"] = v2
break
}
case 23: {
let v2
v2 = rU8() === 1
o2["isEdited"] = v2
break
}
case 24: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rDyn()
o4["key"] = v4
break
}
case 1: {
let v4
v4 = rDyn()
o4["count"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["ownReactionId"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["senders"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v2 = a3
}
o2["reactions"] = v2
break
}
case 25: {
let v2
v2 = rStr()
o2["sendStatus"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.groupAlbums = function groupAlbums (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `startsDay`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.startsDay
const call = native.startsDayWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "lat": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lon": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "question": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "answers": {
wU8(1)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
case "maxSelections": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isDisclosed": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "votes": {
wU8(6)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "voters": {
wU8(7)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "totalVotes": {
wU8(8)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownAnswerIds": {
wU8(9)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
case "canVote": {
wU8(10)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 11) return false
iv.setUint8(at2, n2)
}
}
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "direction": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isVideo": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "durationMs": {
wU8(3)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "title": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "replyCount": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "lastReply": {
wU8(3)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "unread": {
wU8(4)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isParticipating": {
wU8(5)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 18) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.startsDay = function startsDay (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isTranslatable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isTranslatable
const call = native.isTranslatableWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isTranslatable = function isTranslatable (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isEditable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isEditable
const call = native.isEditableWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a0[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isEditable = function isEditable (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `shouldNotify`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.shouldNotify
const call = native.shouldNotifyWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isOwn": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isDirect": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isMention": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (a2 === null || typeof a2 !== 'object' || Array.isArray(a2)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a2) {
switch (k0) {
case "directMessages": {
wU8(0)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "groups": {
wU8(1)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "mentionsOfMe": {
wU8(2)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "invitations": {
wU8(3)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "memberEvents": {
wU8(4)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "notices": {
wU8(5)
r0++
const f0 = a2[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.shouldNotify = function shouldNotify (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parsePermalink`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parsePermalink
const call = native.parsePermalinkWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.parsePermalink = function parsePermalink (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parseRoomAddress`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseRoomAddress
const call = native.parseRoomAddressWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["via"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.parseRoomAddress = function parseRoomAddress (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readSeenSteps`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readSeenSteps
const call = native.readSeenStepsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readSeenSteps = function readSeenSteps (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `tallyPoll`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tallyPoll
const call = native.tallyPollWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "senderId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answerIds": {
wU8(1)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "sentAt": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (a2 === undefined) {
return false
wU8(0)
} else if (a2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'number') return false
wF64(a2)
}
if (typeof a3 !== 'string') return false
wStr(a3)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.tallyPoll = function tallyPoll (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `pressAnswer`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.pressAnswer
const call = native.pressAnswerWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
if (rU8() === 0) t = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
t = a1
}
}
}
return t
}
module.exports.pressAnswer = function pressAnswer (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readNewPoll`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readNewPoll
const call = native.readNewPollWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["question"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["answers"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.readNewPoll = function readNewPoll (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeRoomUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeRoomUnread
const call = native.computeRoomUnreadWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "main": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "latestEventId": {
wU8(0)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "requestCount": {
wU8(1)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
break
}
case "threads": {
wU8(2)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isParticipating": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(2)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "notificationLevel": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isMarkedUnread": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "tag": {
wU8(2)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "folders": {
wU8(3)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["unreadCount"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["mentionCount"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["isMarkedUnread"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.computeRoomUnread = function computeRoomUnread (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeSilentUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeSilentUnread
const call = native.computeSilentUnreadWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "main": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "latestEventId": {
wU8(0)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "requestCount": {
wU8(1)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
break
}
case "threads": {
wU8(2)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isParticipating": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(2)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "notificationLevel": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isMarkedUnread": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "tag": {
wU8(2)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "folders": {
wU8(3)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
}
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.computeSilentUnread = function computeSilentUnread (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isUnread
const call = native.isUnreadWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "unreadCount": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "mentionCount": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isMarkedUnread": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isUnread = function isUnread (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeTotalUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeTotalUnread
const call = native.computeTotalUnreadWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "unreadCount": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "mentionCount": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rF64()
o0["unreadCount"] = v0
break
}
case 1: {
let v0
v0 = rF64()
o0["mentionCount"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["isMarkedUnread"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.computeTotalUnread = function computeTotalUnread (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeReceiptTargets`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeReceiptTargets
const call = native.computeReceiptTargetsWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "main": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "latestEventId": {
wU8(0)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "requestCount": {
wU8(1)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
break
}
case "threads": {
wU8(2)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isParticipating": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(2)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["threadId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["eventId"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.computeReceiptTargets = function computeReceiptTargets (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRoomUnread`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRoomUnread
const call = native.isRoomUnreadWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "unreadCount": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "mentionCount": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isMarkedUnread": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'number') return false
wF64(a1)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isRoomUnread = function isRoomUnread (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `shownUnreadCount`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.shownUnreadCount
const call = native.shownUnreadCountWire
const encode = (a0) => {
at = 0
if (a0 === undefined) {
wU8(0)
} else if (a0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "unread": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "unreadCount": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "mentionCount": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
break
}
case "silentCount": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
}
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.shownUnreadCount = function shownUnreadCount (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `emptyReadState`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.emptyReadState
const call = native.emptyReadStateWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["roomId"] = v0
break
}
case 1: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["latestEventId"] = v1
break
}
case 1: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["requestCount"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
o0["main"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["threadId"] = v2
break
}
case 1: {
let v2
v2 = rU8() === 1
o2["isParticipating"] = v2
break
}
case 2: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isMuted"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["threads"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.emptyReadState = function emptyReadState (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `makeReadRoomState`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.makeReadRoomState
const call = native.makeReadRoomStateWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "main": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "latestEventId": {
wU8(0)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "requestCount": {
wU8(1)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
break
}
case "threads": {
wU8(2)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "threadId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isParticipating": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "isMuted": {
wU8(2)
const f2 = v1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["roomId"] = v0
break
}
case 1: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["latestEventId"] = v1
break
}
case 1: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["requestCount"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
o0["main"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["threadId"] = v2
break
}
case 1: {
let v2
v2 = rU8() === 1
o2["isParticipating"] = v2
break
}
case 2: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isMuted"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["threads"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.makeReadRoomState = function makeReadRoomState (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `hasGifsOff`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hasGifsOff
const call = native.hasGifsOffWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.hasGifsOff = function hasGifsOff (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `offersScribe`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.offersScribe
const call = native.offersScribeWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.offersScribe = function offersScribe (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `spaceOfRoom`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.spaceOfRoom
const call = native.spaceOfRoomWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.spaceOfRoom = function spaceOfRoom (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canBeListenedTo`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canBeListenedTo
const call = native.canBeListenedToWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canBeListenedTo = function canBeListenedTo (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeRoomKind`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeRoomKind
const call = native.computeRoomKindWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "isDirect": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "postPowerLevel": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultUserPowerLevel": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.computeRoomKind = function computeRoomKind (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canPost`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canPost
const call = native.canPostWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canPost = function canPost (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findSpaceGeneral`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findSpaceGeneral
const call = native.findSpaceGeneralWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isSpace": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hasName": {
wU8(5)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "alias": {
wU8(9)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isEncrypted": {
wU8(10)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "gifsOff": {
wU8(16)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isPeerGone": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "parentSpaceIds": {
wU8(22)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["kind"] = v1
break
}
case 3: {
let v1
v1 = rU8() === 1
o1["isSpace"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["directUserId"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hasName"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isVoice"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isPage"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["avatarUrl"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["alias"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isEncrypted"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isWorldReadable"] = v1
break
}
case 12: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isOpenToAll"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isAlone"] = v1
break
}
case 14: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hasAssistant"] = v1
break
}
case 15: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["createdAt"] = v1
break
}
case 16: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["gifsOff"] = v1
break
}
case 17: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["scribeOn"] = v1
break
}
case 18: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hasPublicPage"] = v1
break
}
case 19: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["replacedBy"] = v1
break
}
case 20: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isPeerGone"] = v1
break
}
case 21: {
let v1
v1 = rF64()
o1["postPowerLevel"] = v1
break
}
case 22: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["parentSpaceIds"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.findSpaceGeneral = function findSpaceGeneral (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readArchive`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readArchive
const call = native.readArchiveWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rU8() === 1
o1["isDirect"] = v1
break
}
case 3: {
let v1
v1 = rF64()
o1["leftAt"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["avatarUrl"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["by"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["reason"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["ban"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readArchive = function readArchive (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withArchived`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withArchived
const call = native.withArchivedWire
const encode = (a0, a1, a2) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDirect": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "leftAt": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "avatarUrl": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "ban": {
wU8(5)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "by": {
wU8(0)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "reason": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 4) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rU8() === 1
o1["isDirect"] = v1
break
}
case 3: {
let v1
v1 = rF64()
o1["leftAt"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["avatarUrl"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["by"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["reason"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["ban"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.withArchived = function withArchived (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `toRoomLines`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.toRoomLines
const call = native.toRoomLinesWire
const encode = (a0, a1, a2, a3, a4) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "at": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "actorId": {
wU8(3)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "actorName": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "targetId": {
wU8(5)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "targetName": {
wU8(6)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "name": {
wU8(7)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (typeof v0 !== 'number') return false
wF64(v0)
}
if (a2 === undefined) {
return false
wU8(0)
} else if (a2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof a2 !== 'string') return false
wStr(a2)
}
if (!wDyn(a3)) return false
if (typeof a4 !== 'string') return false
wStr(a4)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rDyn()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.toRoomLines = function toRoomLines (...a) {
  if (a.length <= 5 && encode(a[0], a[1], a[2], a[3], a[4]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `folderView`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.folderView
const call = native.folderViewWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "name": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "builtin": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hidden": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "color": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "include": {
wU8(5)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
case "exclude": {
wU8(6)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 3) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.folderView = function folderView (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isInView`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isInView
const call = native.isInViewWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "room": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isSpace": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hasName": {
wU8(5)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "alias": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isEncrypted": {
wU8(10)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "gifsOff": {
wU8(16)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isPeerGone": {
wU8(20)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "parentSpaceIds": {
wU8(22)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
break
}
case "unread": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "unreadCount": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "mentionCount": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isMarkedUnread": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
break
}
case "tag": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "folders": {
wU8(3)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
if (!wDyn(a1)) return false
if (!Array.isArray(a2)) return false
wU8(1)
wU32(a2.length)
for (let i0 = 0; i0 < a2.length; i0++) {
const v0 = a2[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "room": {
wU8(0)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "kind": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isSpace": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "hasName": {
wU8(5)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "alias": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isEncrypted": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "gifsOff": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isPeerGone": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "parentSpaceIds": {
wU8(22)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 6) return false
iv.setUint8(at2, n2)
}
break
}
case "unread": {
wU8(1)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "mentionCount": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
break
}
case "tag": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "folders": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (a3 === undefined) {
wU8(0)
} else if (a3 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(a3)) return false
wU8(1)
wU32(a3.length)
for (let i0 = 0; i0 < a3.length; i0++) {
const v0 = a3[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "name": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "builtin": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hidden": {
wU8(2)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "emoji": {
wU8(3)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "color": {
wU8(4)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "include": {
wU8(5)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "exclude": {
wU8(6)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isInView = function isInView (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isRoomOwner`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isRoomOwner
const call = native.isRoomOwnerWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isRoomOwner = function isRoomOwner (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canManageMember`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canManageMember
const call = native.canManageMemberWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canManageMember = function canManageMember (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canChangeRole`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canChangeRole
const call = native.canChangeRoleWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canChangeRole = function canChangeRole (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `assignableRoles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.assignableRoles
const call = native.assignableRolesWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.assignableRoles = function assignableRoles (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canBanMember`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canBanMember
const call = native.canBanMemberWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canBanMember = function canBanMember (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canSetReadOnly`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canSetReadOnly
const call = native.canSetReadOnlyWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canSetReadOnly = function canSetReadOnly (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `ownershipAction`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.ownershipAction
const call = native.ownershipActionWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.ownershipAction = function ownershipAction (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canTakeOwnership`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canTakeOwnership
const call = native.canTakeOwnershipWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canTakeOwnership = function canTakeOwnership (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canTransferTo`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canTransferTo
const call = native.canTransferToWire
const encode = (a0, a1, a2) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "canInvite": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canRemove": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canBan": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetLevels": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "readOnlyLevel": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "defaultLevel": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canRedact": {
wU8(6)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEditSettings": {
wU8(7)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canEncrypt": {
wU8(8)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canSetJoinRules": {
wU8(9)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddChildren": {
wU8(10)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "ownPowerLevel": {
wU8(11)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "canMentionEveryone": {
wU8(12)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isManagedBySpace": {
wU8(13)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canTransferOwnership": {
wU8(14)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "canAddOwner": {
wU8(15)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 16) return false
iv.setUint8(at0, n0)
}
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canTransferTo = function canTransferTo (...a) {
  if (a.length <= 3 && encode(a[0], a[1], a[2]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `hasStaffRole`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hasStaffRole
const call = native.hasStaffRoleWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.hasStaffRole = function hasStaffRole (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `permissionRoleOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.permissionRoleOf
const call = native.permissionRoleOfWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "userId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "matrixName": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "avatarUrl": {
wU8(3)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "powerLevel": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "role": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isInvited": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isBot": {
wU8(7)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadOnly": {
wU8(8)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isCreator": {
wU8(9)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isOwner": {
wU8(10)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 10) return false
iv.setUint8(at0, n0)
}
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.permissionRoleOf = function permissionRoleOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `givenRoles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.givenRoles
const call = native.givenRolesWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rDyn()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.givenRoles = function givenRoles (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canHavePublicPage`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canHavePublicPage
const call = native.canHavePublicPageWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canHavePublicPage = function canHavePublicPage (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `publicPagePath`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.publicPagePath
const call = native.publicPagePathWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "name": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "kind": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "isSpace": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "hasName": {
wU8(5)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "alias": {
wU8(9)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isEncrypted": {
wU8(10)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "gifsOff": {
wU8(16)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isPeerGone": {
wU8(20)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "parentSpaceIds": {
wU8(22)
r0++
const f0 = a0[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (typeof v1 !== 'string') return false
wStr(v1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 6) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.publicPagePath = function publicPagePath (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `roomIdOfRoute`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.roomIdOfRoute
const call = native.roomIdOfRouteWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "alias": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.roomIdOfRoute = function roomIdOfRoute (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readSavedRefs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readSavedRefs
const call = native.readSavedRefsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["eventId"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["savedAt"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readSavedRefs = function readSavedRefs (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withSaved`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withSaved
const call = native.withSavedWire
const encode = (a0, a1, a2, a3, a4) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "eventId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "savedAt": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (typeof a3 !== 'boolean') return false
wU8(a3 ? 1 : 0)
if (typeof a4 !== 'number') return false
wF64(a4)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["eventId"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["savedAt"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.withSaved = function withSaved (...a) {
  if (a.length <= 5 && encode(a[0], a[1], a[2], a[3], a[4]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readAutoTranslate`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readAutoTranslate
const call = native.readAutoTranslateWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.readAutoTranslate = function readAutoTranslate (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `toIndexedMessage`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.toIndexedMessage
const call = native.toIndexedMessageWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "id": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(2)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderAvatarUrl": {
wU8(3)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "isFromBot": {
wU8(4)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "text": {
wU8(5)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "html": {
wU8(6)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "undecryptable": {
wU8(7)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "notice": {
wU8(8)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "attachment": {
wU8(9)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "kind": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "mimeType": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "size": {
wU8(3)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "isEncrypted": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "width": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "height": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "duration": {
wU8(6)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "hasThumbnail": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "waveform": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
}
break
}
case "batch": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 7) return false
iv.setUint8(at1, n1)
}
}
break
}
case "location": {
wU8(10)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "lat": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lon": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
break
}
case "poll": {
wU8(11)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "question": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "answers": {
wU8(1)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "id": {
wU8(0)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
case "text": {
wU8(1)
const f3 = v2[k3]
if (!wDyn(f3)) return false
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 0) return false
iv.setUint8(at3, n3)
}
}
break
}
case "maxSelections": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isDisclosed": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isAnonymous": {
wU8(4)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "endedAt": {
wU8(5)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "votes": {
wU8(6)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "voters": {
wU8(7)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "totalVotes": {
wU8(8)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "ownAnswerIds": {
wU8(9)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
case "canVote": {
wU8(10)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEnd": {
wU8(11)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "canEdit": {
wU8(12)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 11) return false
iv.setUint8(at1, n1)
}
}
break
}
case "question": {
wU8(12)
const f0 = a1[k0]
if (!wDyn(f0)) return false
break
}
case "isEmote": {
wU8(13)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "direction": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isVideo": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "outcome": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "durationMs": {
wU8(3)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "sentAt": {
wU8(17)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isOwn": {
wU8(18)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f0 = a1[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "userId": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "avatarUrl": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "readAt": {
wU8(3)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
}
break
}
case "thread": {
wU8(21)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
const f1 = f0[k1]
if (!wDyn(f1)) return false
break
}
case "unread": {
wU8(4)
r1++
const f1 = f0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "mentionCount": {
wU8(1)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 0) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 5) return false
iv.setUint8(at1, n1)
}
}
break
}
case "replyTo": {
wU8(22)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "messageId": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(1)
r1++
const f1 = f0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "senderId": {
wU8(2)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "text": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isDeleted": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
break
}
case "isEdited": {
wU8(23)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r0++
const f0 = a1[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === null || typeof v1 !== 'object' || Array.isArray(v1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in v1) {
switch (k2) {
case "key": {
wU8(0)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "count": {
wU8(1)
r2++
const f2 = v1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "ownReactionId": {
wU8(2)
r2++
const f2 = v1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senders": {
wU8(3)
r2++
const f2 = v1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (!wDyn(v3)) return false
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "sendStatus": {
wU8(25)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 18) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["messageId"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["senderId"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["senderName"] = v1
break
}
case 4: {
let v1
v1 = rStr()
o1["text"] = v1
break
}
case 5: {
let v1
v1 = rF64()
o1["sentAt"] = v1
break
}
case 6: {
let v1
v1 = rU8() === 1
o1["hasFile"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.toIndexedMessage = function toIndexedMessage (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `isEmptyFilter`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isEmptyFilter
const call = native.isEmptyFilterWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "query": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "since": {
wU8(2)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "hasFile": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isEmptyFilter = function isEmptyFilter (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `matchesFilter`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.matchesFilter
const call = native.matchesFilterWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "roomId": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "messageId": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(2)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderName": {
wU8(3)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "text": {
wU8(4)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "sentAt": {
wU8(5)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "hasFile": {
wU8(6)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 7) return false
iv.setUint8(at0, n0)
}
if (a1 === null || typeof a1 !== 'object' || Array.isArray(a1)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a1) {
switch (k0) {
case "query": {
wU8(0)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'string') return false
wStr(f0)
break
}
case "senderId": {
wU8(1)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
case "since": {
wU8(2)
r0++
const f0 = a1[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
case "hasFile": {
wU8(3)
r0++
const f0 = a1[k0]
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 4) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.matchesFilter = function matchesFilter (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeSharedContent`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeSharedContent
const call = native.computeSharedContentWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderName": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "senderAvatarUrl": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isFromBot": {
wU8(4)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "text": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "html": {
wU8(6)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "undecryptable": {
wU8(7)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "notice": {
wU8(8)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "attachment": {
wU8(9)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "kind": {
wU8(0)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "mimeType": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "size": {
wU8(3)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "isEncrypted": {
wU8(4)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "dimensions": {
wU8(5)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "duration": {
wU8(6)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "hasThumbnail": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "voice": {
wU8(8)
const f2 = f1[k2]
if (!wDyn(f2)) return false
break
}
case "batch": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 5) return false
iv.setUint8(at2, n2)
}
}
break
}
case "location": {
wU8(10)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "poll": {
wU8(11)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "question": {
wU8(12)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "isEmote": {
wU8(13)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "mentionsRoom": {
wU8(14)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isCallThreadRoot": {
wU8(15)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "call": {
wU8(16)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "sentAt": {
wU8(17)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "isOwn": {
wU8(18)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isReadByOthers": {
wU8(19)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "lastReadBy": {
wU8(20)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "userId": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "name": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "avatarUrl": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "readAt": {
wU8(3)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'number') return false
wF64(f3)
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
}
break
}
case "thread": {
wU8(21)
const f1 = v0[k1]
if (!wDyn(f1)) return false
break
}
case "replyTo": {
wU8(22)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "messageId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (f2 === undefined) {
return false
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "senderId": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "text": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isDeleted": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 3) return false
iv.setUint8(at2, n2)
}
}
break
}
case "isEdited": {
wU8(23)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "reactions": {
wU8(24)
r1++
const f1 = v0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (v2 === null || typeof v2 !== 'object' || Array.isArray(v2)) return false
{
const at3 = at
wU8(0)
let n3 = 0, r3 = 0
for (const k3 in v2) {
switch (k3) {
case "key": {
wU8(0)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'string') return false
wStr(f3)
break
}
case "count": {
wU8(1)
r3++
const f3 = v2[k3]
if (typeof f3 !== 'number') return false
wF64(f3)
break
}
case "ownReactionId": {
wU8(2)
r3++
const f3 = v2[k3]
if (f3 === undefined) {
return false
wU8(0)
} else if (f3 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f3 !== 'string') return false
wStr(f3)
}
break
}
case "senders": {
wU8(3)
r3++
const f3 = v2[k3]
if (!Array.isArray(f3)) return false
wU8(1)
wU32(f3.length)
for (let i4 = 0; i4 < f3.length; i4++) {
const v4 = f3[i4]
if (!wDyn(v4)) return false
}
break
}
default: return false
}
if (++n3 > 255) return false
}
if (r3 !== 4) return false
iv.setUint8(at3, n3)
}
}
break
}
case "sendStatus": {
wU8(25)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 16) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["messageId"] = v2
break
}
case 1: {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["kind"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["name"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["mimeType"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["size"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["isEncrypted"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["dimensions"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["duration"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["hasThumbnail"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["voice"] = v3
break
}
case 9: {
let v3
v3 = rDyn()
o3["batch"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2["attachment"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["senderName"] = v2
break
}
case 3: {
let v2
v2 = rF64()
o2["sentAt"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["media"] = v0
break
}
case 1: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["messageId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["url"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["senderName"] = v2
break
}
case 3: {
let v2
v2 = rF64()
o2["sentAt"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["links"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["messageId"] = v2
break
}
case 1: {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rDyn()
o3["kind"] = v3
break
}
case 1: {
let v3
v3 = rDyn()
o3["name"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["mimeType"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["size"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["isEncrypted"] = v3
break
}
case 5: {
let v3
v3 = rDyn()
o3["dimensions"] = v3
break
}
case 6: {
let v3
v3 = rDyn()
o3["duration"] = v3
break
}
case 7: {
let v3
v3 = rDyn()
o3["hasThumbnail"] = v3
break
}
case 8: {
let v3
v3 = rDyn()
o3["voice"] = v3
break
}
case 9: {
let v3
v3 = rDyn()
o3["batch"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2["attachment"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["senderName"] = v2
break
}
case 3: {
let v2
v2 = rF64()
o2["sentAt"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
a1[i1] = v1
}
v0 = a1
}
o0["files"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.computeSharedContent = function computeSharedContent (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `roomBadgeCount`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.roomBadgeCount
const call = native.roomBadgeCountWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "unreadCount": {
wU8(0)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "mentionCount": {
wU8(1)
r0++
const f0 = a0[k0]
if (typeof f0 !== 'number') return false
wF64(f0)
break
}
case "isMarkedUnread": {
wU8(2)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'boolean') return false
wU8(f0 ? 1 : 0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 2) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.roomBadgeCount = function roomBadgeCount (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `computeSpaceBadges`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.computeSpaceBadges
const call = native.computeSpaceBadgesWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "room": {
wU8(0)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "kind": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isSpace": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "hasName": {
wU8(5)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "alias": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isEncrypted": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "gifsOff": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isPeerGone": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "parentSpaceIds": {
wU8(22)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 6) return false
iv.setUint8(at2, n2)
}
break
}
case "unread": {
wU8(1)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "mentionCount": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["spaceId"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["count"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.computeSpaceBadges = function computeSpaceBadges (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `canMoveToSpace`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.canMoveToSpace
const call = native.canMoveToSpaceWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.canMoveToSpace = function canMoveToSpace (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `arrangeRooms`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.arrangeRooms
const call = native.arrangeRoomsWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "room": {
wU8(0)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "kind": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isSpace": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "hasName": {
wU8(5)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "alias": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isEncrypted": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "gifsOff": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isPeerGone": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "parentSpaceIds": {
wU8(22)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 6) return false
iv.setUint8(at2, n2)
}
break
}
case "tag": {
wU8(1)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'boolean') return false
wU8(a1 ? 1 : 0)
}
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.arrangeRooms = function arrangeRooms (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `staysInTree`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.staysInTree
const call = native.staysInTreeWire
const encode = (a0) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "room": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isSpace": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hasName": {
wU8(5)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "alias": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isEncrypted": {
wU8(10)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "gifsOff": {
wU8(16)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isPeerGone": {
wU8(20)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "parentSpaceIds": {
wU8(22)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
break
}
case "tag": {
wU8(1)
const f0 = a0[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.staysInTree = function staysInTree (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `parentSpaceOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parentSpaceOf
const call = native.parentSpaceOfWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "room": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === null || typeof f0 !== 'object' || Array.isArray(f0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in f0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "name": {
wU8(1)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "kind": {
wU8(2)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "isSpace": {
wU8(3)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "hasName": {
wU8(5)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "alias": {
wU8(9)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isEncrypted": {
wU8(10)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
case "gifsOff": {
wU8(16)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'string') return false
wStr(f1)
}
break
}
case "isPeerGone": {
wU8(20)
const f1 = f0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r1++
const f1 = f0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "parentSpaceIds": {
wU8(22)
r1++
const f1 = f0[k1]
if (!Array.isArray(f1)) return false
wU8(1)
wU32(f1.length)
for (let i2 = 0; i2 < f1.length; i2++) {
const v2 = f1[i2]
if (typeof v2 !== 'string') return false
wStr(v2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "room": {
wU8(0)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "id": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "name": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "kind": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "isSpace": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
break
}
case "directUserId": {
wU8(4)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "hasName": {
wU8(5)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isVoice": {
wU8(6)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isPage": {
wU8(7)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "avatarUrl": {
wU8(8)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "alias": {
wU8(9)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isEncrypted": {
wU8(10)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isWorldReadable": {
wU8(11)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isOpenToAll": {
wU8(12)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "isAlone": {
wU8(13)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasAssistant": {
wU8(14)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "createdAt": {
wU8(15)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'number') return false
wF64(f2)
}
break
}
case "gifsOff": {
wU8(16)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "scribeOn": {
wU8(17)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "hasPublicPage": {
wU8(18)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "replacedBy": {
wU8(19)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'string') return false
wStr(f2)
}
break
}
case "isPeerGone": {
wU8(20)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
case "postPowerLevel": {
wU8(21)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "parentSpaceIds": {
wU8(22)
r2++
const f2 = f1[k2]
if (!Array.isArray(f2)) return false
wU8(1)
wU32(f2.length)
for (let i3 = 0; i3 < f2.length; i3++) {
const v3 = f2[i3]
if (typeof v3 !== 'string') return false
wStr(v3)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 6) return false
iv.setUint8(at2, n2)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["id"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["kind"] = v2
break
}
case 3: {
let v2
v2 = rU8() === 1
o2["isSpace"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["directUserId"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["hasName"] = v2
break
}
case 6: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isVoice"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isPage"] = v2
break
}
case 8: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["avatarUrl"] = v2
break
}
case 9: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["alias"] = v2
break
}
case 10: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isEncrypted"] = v2
break
}
case 11: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isWorldReadable"] = v2
break
}
case 12: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isOpenToAll"] = v2
break
}
case 13: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isAlone"] = v2
break
}
case 14: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["hasAssistant"] = v2
break
}
case 15: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["createdAt"] = v2
break
}
case 16: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["gifsOff"] = v2
break
}
case 17: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["scribeOn"] = v2
break
}
case 18: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["hasPublicPage"] = v2
break
}
case 19: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["replacedBy"] = v2
break
}
case 20: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isPeerGone"] = v2
break
}
case 21: {
let v2
v2 = rF64()
o2["postPowerLevel"] = v2
break
}
case 22: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v2 = a3
}
o2["parentSpaceIds"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
o1["room"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.parentSpaceOf = function parentSpaceOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `presenceOfPreset`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.presenceOfPreset
const call = native.presenceOfPresetWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.presenceOfPreset = function presenceOfPreset (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `sortThreadsByActivity`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.sortThreadsByActivity
const call = native.sortThreadsByActivityWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "threadId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "title": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "replyCount": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
case "lastReply": {
wU8(3)
r1++
const f1 = v0[k1]
if (f1 === undefined) {
return false
wU8(0)
} else if (f1 === null) {
wU8(2)
} else {
wU8(1)
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "senderId": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "senderName": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "text": {
wU8(2)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'string') return false
wStr(f2)
break
}
case "sentAt": {
wU8(3)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 4) return false
iv.setUint8(at2, n2)
}
}
break
}
case "unread": {
wU8(4)
r1++
const f1 = v0[k1]
if (f1 === null || typeof f1 !== 'object' || Array.isArray(f1)) return false
{
const at2 = at
wU8(0)
let n2 = 0, r2 = 0
for (const k2 in f1) {
switch (k2) {
case "unreadCount": {
wU8(0)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "mentionCount": {
wU8(1)
r2++
const f2 = f1[k2]
if (typeof f2 !== 'number') return false
wF64(f2)
break
}
case "isMarkedUnread": {
wU8(2)
const f2 = f1[k2]
if (f2 === undefined) {
wU8(0)
} else if (f2 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f2 !== 'boolean') return false
wU8(f2 ? 1 : 0)
}
break
}
default: return false
}
if (++n2 > 255) return false
}
if (r2 !== 2) return false
iv.setUint8(at2, n2)
}
break
}
case "isParticipating": {
wU8(5)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
break
}
case "isMuted": {
wU8(6)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "isResolved": {
wU8(7)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'boolean') return false
wU8(f1 ? 1 : 0)
}
break
}
case "lastOwnReplyAt": {
wU8(8)
const f1 = v0[k1]
if (f1 === undefined) {
wU8(0)
} else if (f1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof f1 !== 'number') return false
wF64(f1)
}
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 6) return false
iv.setUint8(at1, n1)
}
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["threadId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["title"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["replyCount"] = v1
break
}
case 3: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["senderId"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["senderName"] = v3
break
}
case 2: {
let v3
v3 = rStr()
o3["text"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["sentAt"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["lastReply"] = v1
break
}
case 4: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rF64()
o2["unreadCount"] = v2
break
}
case 1: {
let v2
v2 = rF64()
o2["mentionCount"] = v2
break
}
case 2: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["isMarkedUnread"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
o1["unread"] = v1
break
}
case 5: {
let v1
v1 = rU8() === 1
o1["isParticipating"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isMuted"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["isResolved"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["lastOwnReplyAt"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.sortThreadsByActivity = function sortThreadsByActivity (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `readThreadPrefs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.readThreadPrefs
const call = native.readThreadPrefsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["threadId"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["pref"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.readThreadPrefs = function readThreadPrefs (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `withThreadPref`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withThreadPref
const call = native.withThreadPrefWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "roomId": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "threadId": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "pref": {
wU8(2)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 3) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (typeof a3 !== 'string') return false
wStr(a3)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["roomId"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["threadId"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["pref"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.withThreadPref = function withThreadPref (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `resolveThreadFollow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.resolveThreadFollow
const call = native.resolveThreadFollowWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'boolean') return false
wU8(a0 ? 1 : 0)
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rU8() === 1
o0["isFollowing"] = v0
break
}
case 1: {
let v0
v0 = rU8() === 1
o0["isMuted"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.resolveThreadFollow = function resolveThreadFollow (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `timelineWindowOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.timelineWindowOf
const call = native.timelineWindowOfWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
case "sentAt": {
wU8(1)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 2) return false
iv.setUint8(at1, n1)
}
}
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (!Array.isArray(a1)) return false
wU8(1)
wU32(a1.length)
for (let i0 = 0; i0 < a1.length; i0++) {
const v0 = a1[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "at": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'number') return false
wF64(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
}
if (!wDyn(a2)) return false
if (a3 === null || typeof a3 !== 'object' || Array.isArray(a3)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a3) {
switch (k0) {
case "ids": {
wU8(0)
r0++
const f0 = a3[k0]
if (!Array.isArray(f0)) return false
wU8(1)
wU32(f0.length)
for (let i1 = 0; i1 < f0.length; i1++) {
const v1 = f0[i1]
if (v1 === undefined) {
wU8(0)
} else if (v1 === null) {
wU8(2)
} else {
wU8(1)
if (typeof v1 !== 'string') return false
wStr(v1)
}
}
break
}
case "since": {
wU8(1)
const f0 = a3[k0]
if (f0 === undefined) {
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'number') return false
wF64(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rDyn()
a1[i1] = v1
}
v0 = a1
}
o0["messages"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rDyn()
a2[i2] = v2
}
v0 = a2
}
}
}
o0["lines"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["hiddenAbove"] = v0
break
}
case 3: {
let v0
v0 = rDyn()
o0["start"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.timelineWindowOf = function timelineWindowOf (...a) {
  if (a.length <= 4 && encode(a[0], a[1], a[2], a[3]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `olderStartOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.olderStartOf
const call = native.olderStartOfWire
const encode = (a0, a1) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (v0 === null || typeof v0 !== 'object' || Array.isArray(v0)) return false
{
const at1 = at
wU8(0)
let n1 = 0, r1 = 0
for (const k1 in v0) {
switch (k1) {
case "id": {
wU8(0)
r1++
const f1 = v0[k1]
if (typeof f1 !== 'string') return false
wStr(f1)
break
}
default: return false
}
if (++n1 > 255) return false
}
if (r1 !== 1) return false
iv.setUint8(at1, n1)
}
}
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.olderStartOf = function olderStartOf (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `commonSettingsUrlFor`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.commonSettingsUrlFor
const call = native.commonSettingsUrlForWire
const encode = (a0, a1) => {
at = 0
if (a0 === null || typeof a0 !== 'object' || Array.isArray(a0)) return false
{
const at0 = at
wU8(0)
let n0 = 0, r0 = 0
for (const k0 in a0) {
switch (k0) {
case "applicationUrl": {
wU8(0)
r0++
const f0 = a0[k0]
if (f0 === undefined) {
return false
wU8(0)
} else if (f0 === null) {
wU8(2)
} else {
wU8(1)
if (typeof f0 !== 'string') return false
wStr(f0)
}
break
}
default: return false
}
if (++n0 > 255) return false
}
if (r0 !== 1) return false
iv.setUint8(at0, n0)
}
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.commonSettingsUrlFor = function commonSettingsUrlFor (...a) {
  if (a.length <= 2 && encode(a[0], a[1]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}

// `findWebLinks`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.findWebLinks
const call = native.findWebLinksWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rF64()
o1["index"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["url"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.findWebLinks = function findWebLinks (...a) {
  if (a.length <= 1 && encode(a[0]) && wireCall(call)) {
    return decode()
  }
  return json(...a)
}
}
const guards = {
  buildActivity: [false, []],
  buildFeed: [false, []],
  filterFeed: [false, [1]],
  searchFeed: [false, [1]],
  groupByDay: [false, []],
  assistantConsentStatus: [false, [1, 2]],
  requestsWaitingFor: [false, [1, 2]],
  peopleOf: [false, []],
  presentRequest: [false, []],
  findOpenQuestion: [false, [2]],
  computeWaveform: [false, []],
  isPlayableAudio: [false, []],
  isViewablePdf: [false, []],
  isPreviewable: [false, []],
  isGif: [false, []],
  computeAttachmentKind: [false, [0]],
  inferMimeType: [false, [0, 1]],
  isSvg: [false, [0]],
  blobSafeMimeType: [false, [0]],
  svgCanvasSize: [false, []],
  thumbnailSize: [false, []],
  botOf: [false, []],
  botIdOf: [false, [0]],
  ownsBot: [false, [0, 1, 2]],
  offersCommandsOf: [false, [0, 1, 2, 3]],
  findCommandQuery: [false, [0]],
  matchCommands: [false, [1]],
  findLocalCommand: [false, [0]],
  prepareCommandText: [false, [0]],
  prepareOutgoing: [false, [0]],
  shouldMirror: [false, []],
  recordingModeOf: [false, [1]],
  currentDeviceId: [false, [1]],
  countCallPeople: [false, []],
  ringAsks: [false, [2]],
  isRinging: [false, [1]],
  findCallThread: [false, []],
  toCallLine: [false, []],
  fitWindow: [false, []],
  defaultCallWindow: [false, []],
  moveWindow: [false, []],
  placeInCorner: [false, [1]],
  resizeWindow: [false, [1]],
  parseCallWindow: [false, [0]],
  placeCall: [false, []],
  isCallFloating: [false, [1]],
  parseClientConfig: [false, []],
  readComposerStyle: [false, []],
  parseInvitationTarget: [false, [0]],
  groupByInitial: [false, []],
  parseContactsVisibility: [false, []],
  emptyCardOf: [false, []],
  footerOf: [false, []],
  timelineHeadOf: [false, []],
  headOverWindow: [false, []],
  isLandingRoom: [false, []],
  mayPostIn: [false, []],
  isDirectRoom: [false, [1]],
  peerStateOf: [false, []],
  splitsCallChat: [false, []],
  isEncryptedConversation: [false, []],
  isAwaitingRoom: [false, []],
  isEmptyConversation: [false, []],
  isCardInlineIn: [false, []],
  roomNameOf: [false, []],
  assistantOfferIn: [false, []],
  hasAssistantIn: [false, []],
  offersAddContact: [false, []],
  matchPlaceOf: [false, [0, 1]],
  readDeletedChats: [false, []],
  isChatHidden: [false, [1]],
  withoutDeleted: [false, [1]],
  isAfterDeletion: [false, [1]],
  readDismissedQuestions: [false, []],
  withDismissedQuestion: [false, [1]],
  findEmojiQuery: [false, [0]],
  isEmojiDrawable: [false, [0]],
  hasInk: [false, []],
  hasColour: [false, []],
  isDrawn: [false, []],
  undecryptableHelpKey: [false, [0]],
  normalizeRecoveryKey: [false, [0]],
  isNotificationService: [false, []],
  readFolders: [false, []],
  writeFolders: [false, []],
  withHiddenBuiltins: [false, []],
  listFolders: [false, []],
  folderChatTypeOf: [false, []],
  isInFolder: [false, []],
  isRecommendationTaken: [false, [0]],
  formatDuration: [false, []],
  fitPreview: [false, []],
  isRecord: [false, []],
  readText: [false, [1]],
  readTexts: [false, [1]],
  readBoolean: [false, [1]],
  isJumboEmoji: [false, [0]],
  isLikelyIn: [false, [0, 1]],
  autoTranslationOf: [false, [1]],
  parseGeoUri: [false, []],
  openStreetMapUrl: [false, []],
  avatarThumbnailSize: [false, []],
  thumbnailMediaUrl: [false, [0, 1, 3]],
  pickAttendees: [false, [2]],
  newEventUrl: [false, [0, 1]],
  readCreatedMeeting: [false, []],
  meetingMessageText: [false, [1, 2]],
  describeMeetingTime: [false, [3]],
  describePeople: [false, [0]],
  matchMembers: [false, [1, 2]],
  mentionText: [false, []],
  keepPresentMentions: [false, [0]],
  namePattern: [false, [0]],
  matchRooms: [false, [1]],
  roomMentionRef: [false, []],
  roomMentionText: [false, []],
  isRoomMention: [false, []],
  isEveryoneMention: [false, []],
  offersEveryone: [false, [0]],
  describeEmote: [false, [0, 1]],
  bareEmoji: [false, [0]],
  countsAsUnread: [false, []],
  findFirstUnreadId: [false, []],
  isSameDay: [false, []],
  isSelectable: [false, []],
  startsGroup: [false, []],
  groupAlbums: [false, [1]],
  startsDay: [false, []],
  isTranslatable: [false, []],
  isEditable: [false, []],
  shouldNotify: [false, [1]],
  messageLink: [false, [0, 1, 2]],
  roomLink: [false, [0, 1]],
  userPermalink: [false, [0]],
  roomPermalink: [false, [0]],
  parsePermalink: [false, [0, 1]],
  parseRoomAddress: [false, [0, 1]],
  readSeenSteps: [false, []],
  parseSeenSteps: [false, [0]],
  unionSteps: [false, []],
  tallyPoll: [false, [3]],
  pressAnswer: [false, [1]],
  readNewPoll: [false, [0, 1]],
  computeRoomUnread: [false, []],
  computeSilentUnread: [false, []],
  isUnread: [false, []],
  computeTotalUnread: [false, []],
  computeReceiptTargets: [false, []],
  isRoomUnread: [false, []],
  shownUnreadCount: [false, []],
  chooseReceiptEvent: [false, [0, 2]],
  emptyReadState: [false, [0]],
  makeReadRoomState: [false, []],
  hasGifsOff: [false, []],
  offersScribe: [false, []],
  spaceOfRoom: [false, []],
  canBeListenedTo: [false, []],
  computeRoomKind: [false, []],
  canPost: [false, []],
  findSpaceGeneral: [false, [1]],
  readArchive: [false, []],
  withArchived: [false, [1]],
  toRoomLines: [false, [2, 4]],
  folderView: [false, []],
  isInView: [false, []],
  isOnOtherServer: [false, [0, 1]],
  levelOfRole: [false, [0]],
  computeRole: [false, []],
  isRoomOwner: [false, []],
  canManageMember: [false, []],
  canChangeRole: [false, []],
  assignableRoles: [false, []],
  canBanMember: [false, []],
  canSetReadOnly: [false, []],
  ownershipAction: [false, []],
  canTakeOwnership: [false, []],
  canTransferTo: [false, []],
  hasStaffRole: [false, []],
  permissionRoleOf: [false, []],
  givenRoles: [false, []],
  isValidSlug: [false, [0]],
  canHavePublicPage: [false, []],
  publicPagePath: [false, []],
  suggestSlug: [false, [0]],
  isValidColor: [false, [0]],
  parseMatrixUserId: [false, [0]],
  toPreviewText: [false, [0]],
  roomIdOfRoute: [false, [0]],
  readSavedRefs: [false, []],
  withSaved: [false, [1, 2]],
  readAutoTranslate: [false, []],
  normalizeForSearch: [false, [0]],
  isClosedQuery: [false, [0]],
  holdsWord: [false, [0, 1]],
  toIndexedMessage: [false, [0]],
  isLocalEchoId: [false, [0]],
  isEmptyFilter: [false, []],
  matchesFilter: [false, []],
  isServiceUrl: [false, [0]],
  extractLinks: [false, [0]],
  computeSharedContent: [false, []],
  roomBadgeCount: [false, []],
  computeSpaceBadges: [false, []],
  canMoveToSpace: [false, []],
  arrangeRooms: [false, []],
  staysInTree: [false, []],
  parentSpaceOf: [false, []],
  nonBlank: [false, [0]],
  stripBidi: [false, [0]],
  normalizeStatus: [false, [0]],
  isNameTooLong: [false, [0]],
  presenceOfPreset: [false, []],
  computeThreadTitle: [false, [0]],
  sortThreadsByActivity: [false, []],
  readThreadPrefs: [false, []],
  withThreadPref: [false, [1, 2, 3]],
  resolveThreadFollow: [false, [1]],
  timelineWindowOf: [false, []],
  olderStartOf: [false, []],
  commonSettingsUrlFor: [false, [1]],
  findWebLinks: [false, [0]],
  toHref: [false, [0]],
}
const JS_ERRORS = /^(TypeError|RangeError|SyntaxError|ReferenceError|URIError|EvalError): ([\s\S]*)$/
const jsError = e => {
  const message = typeof e?.message === 'string' ? e.message : ''
  const typed = JS_ERRORS.exec(message)
  if (typed) return new globalThis[typed[1]](typed[2])
  if (/Expected$/.test(e?.code)) return Object.assign(new TypeError(message), { code: e.code })
  if (message.startsWith('Rust panic:')) e.code = 'RUST_PANIC'
  return e
}
const illFormed = v => typeof v === 'string' ? !v.isWellFormed() : Array.isArray(v) && v.some(illFormed)
for (const [name, [isAsync, strings]] of Object.entries(guards)) {
  const f = module.exports[name]
  if (typeof f !== 'function') continue
  const check = a => {
    for (const i of strings) {
      if (illFormed(a[i])) throw new TypeError(`argument ${i + 1} of ${name} has a lone surrogate, which a Rust String cannot hold`)
    }
  }
  module.exports[name] = isAsync
    ? { [name]: async function (...a) { check(a); try { return await f(...a) } catch (e) { throw jsError(e) } } }[name]
    : { [name]: function (...a) { check(a); try { return f(...a) } catch (e) { throw jsError(e) } } }[name]
}
