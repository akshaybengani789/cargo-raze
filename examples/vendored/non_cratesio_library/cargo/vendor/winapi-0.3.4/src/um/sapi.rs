// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! SAPI 5.4 definitions
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, ULONG, WORD};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPWSTR, ULONGLONG};
pub use um::sapi53::{
    SPDATAKEYLOCATION,
    SPDKL_DefaultLocation,
    SPDKL_CurrentUser,
    SPDKL_LocalMachine,
    SPDKL_CurrentConfig,
    SPDUI_EngineProperties,
    SPDUI_AddRemoveWord,
    SPDUI_UserTraining,
    SPDUI_MicTraining,
    SPDUI_RecoProfileProperties,
    SPDUI_AudioProperties,
    SPDUI_AudioVolume,
    SPDUI_UserEnrollment,
    SPDUI_ShareData,
    SPDUI_Tutorial,
    SPSTREAMFORMAT,
    SPSF_Default,
    SPSF_NoAssignedFormat,
    SPSF_Text,
    SPSF_NonStandardFormat,
    SPSF_ExtendedAudioFormat,
    SPSF_8kHz8BitMono,
    SPSF_8kHz8BitStereo,
    SPSF_8kHz16BitMono,
    SPSF_8kHz16BitStereo,
    SPSF_11kHz8BitMono,
    SPSF_11kHz8BitStereo,
    SPSF_11kHz16BitMono,
    SPSF_11kHz16BitStereo,
    SPSF_12kHz8BitMono,
    SPSF_12kHz8BitStereo,
    SPSF_12kHz16BitMono,
    SPSF_12kHz16BitStereo,
    SPSF_16kHz8BitMono,
    SPSF_16kHz8BitStereo,
    SPSF_16kHz16BitMono,
    SPSF_16kHz16BitStereo,
    SPSF_22kHz8BitMono,
    SPSF_22kHz8BitStereo,
    SPSF_22kHz16BitMono,
    SPSF_22kHz16BitStereo,
    SPSF_24kHz8BitMono,
    SPSF_24kHz8BitStereo,
    SPSF_24kHz16BitMono,
    SPSF_24kHz16BitStereo,
    SPSF_32kHz8BitMono,
    SPSF_32kHz8BitStereo,
    SPSF_32kHz16BitMono,
    SPSF_32kHz16BitStereo,
    SPSF_44kHz8BitMono,
    SPSF_44kHz8BitStereo,
    SPSF_44kHz16BitMono,
    SPSF_44kHz16BitStereo,
    SPSF_48kHz8BitMono,
    SPSF_48kHz8BitStereo,
    SPSF_48kHz16BitMono,
    SPSF_48kHz16BitStereo,
    SPSF_TrueSpeech_8kHz1BitMono,
    SPSF_CCITT_ALaw_8kHzMono,
    SPSF_CCITT_ALaw_8kHzStereo,
    SPSF_CCITT_ALaw_11kHzMono,
    SPSF_CCITT_ALaw_11kHzStereo,
    SPSF_CCITT_ALaw_22kHzMono,
    SPSF_CCITT_ALaw_22kHzStereo,
    SPSF_CCITT_ALaw_44kHzMono,
    SPSF_CCITT_ALaw_44kHzStereo,
    SPSF_CCITT_uLaw_8kHzMono,
    SPSF_CCITT_uLaw_8kHzStereo,
    SPSF_CCITT_uLaw_11kHzMono,
    SPSF_CCITT_uLaw_11kHzStereo,
    SPSF_CCITT_uLaw_22kHzMono,
    SPSF_CCITT_uLaw_22kHzStereo,
    SPSF_CCITT_uLaw_44kHzMono,
    SPSF_CCITT_uLaw_44kHzStereo,
    SPSF_ADPCM_8kHzMono,
    SPSF_ADPCM_8kHzStereo,
    SPSF_ADPCM_11kHzMono,
    SPSF_ADPCM_11kHzStereo,
    SPSF_ADPCM_22kHzMono,
    SPSF_ADPCM_22kHzStereo,
    SPSF_ADPCM_44kHzMono,
    SPSF_ADPCM_44kHzStereo,
    SPSF_GSM610_8kHzMono,
    SPSF_GSM610_11kHzMono,
    SPSF_GSM610_22kHzMono,
    SPSF_GSM610_44kHzMono,
    SPSF_NUM_FORMATS,
    SPDFID_Text,
    SPDFID_WaveFormatEx,
    SPREG_USER_ROOT,
    SPREG_LOCAL_MACHINE_ROOT,
    SPCAT_AUDIOOUT,
    SPCAT_AUDIOIN,
    SPCAT_VOICES,
    SPCAT_RECOGNIZERS,
    SPCAT_APPLEXICONS,
    SPCAT_PHONECONVERTERS,
    SPCAT_TEXTNORMALIZERS,
    SPCAT_RECOPROFILES,
    SPMMSYS_AUDIO_IN_TOKEN_ID,
    SPMMSYS_AUDIO_OUT_TOKEN_ID,
    SPCURRENT_USER_LEXICON_TOKEN_ID,
    SPTOKENVALUE_CLSID,
    SPTOKENKEY_FILES,
    SPTOKENKEY_UI,
    SPTOKENKEY_ATTRIBUTES,
    SPTOKENKEY_RETAINEDAUDIO,
    SPTOKENKEY_AUDIO_LATENCY_WARNING,
    SPTOKENKEY_AUDIO_LATENCY_TRUNCATE,
    SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL,
    SPVOICECATEGORY_TTSRATE,
    SPPROP_RESOURCE_USAGE,
    SPPROP_HIGH_CONFIDENCE_THRESHOLD,
    SPPROP_NORMAL_CONFIDENCE_THRESHOLD,
    SPPROP_LOW_CONFIDENCE_THRESHOLD,
    SPPROP_RESPONSE_SPEED,
    SPPROP_COMPLEX_RESPONSE_SPEED,
    SPPROP_ADAPTATION_ON,
    SPPROP_PERSISTED_BACKGROUND_ADAPTATION,
    SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION,
    SPPROP_UX_IS_LISTENING,
    SPTOPIC_SPELLING,
    SPWILDCARD,
    SPDICTATION,
    SPREG_SAFE_USER_TOKENS,
    SPINFDICTATION,
    SP_LOW_CONFIDENCE,
    SP_NORMAL_CONFIDENCE,
    SP_HIGH_CONFIDENCE,
    DEFAULT_WEIGHT,
    SP_MAX_WORD_LENGTH,
    SP_MAX_PRON_LENGTH,
    SP_EMULATE_RESULT,
    ISpNotifyCallback,
    SPNOTIFYCALLBACK,
    ISpNotifySource, ISpNotifySourceVtbl,
    ISpNotifySink, ISpNotifySinkVtbl,
    ISpNotifyTranslator, ISpNotifyTranslatorVtbl,
    ISpDataKey, ISpDataKeyVtbl,
    ISpRegDataKey, ISpRegDataKeyVtbl,
    ISpObjectTokenCategory, ISpObjectTokenCategoryVtbl,
    ISpObjectToken, ISpObjectTokenVtbl,
    ISpObjectTokenInit, ISpObjectTokenInitVtbl,
    IEnumSpObjectTokens, IEnumSpObjectTokensVtbl,
    ISpObjectWithToken, ISpObjectWithTokenVtbl,
    ISpResourceManager, ISpResourceManagerVtbl,
    SPEVENTLPARAMTYPE,
    SPET_LPARAM_IS_UNDEFINED,
    SPET_LPARAM_IS_TOKEN,
    SPET_LPARAM_IS_OBJECT,
    SPET_LPARAM_IS_POINTER,
    SPET_LPARAM_IS_STRING,
    SPEVENTENUM,
    SPEI_UNDEFINED,
    SPEI_START_INPUT_STREAM,
    SPEI_END_INPUT_STREAM,
    SPEI_VOICE_CHANGE,
    SPEI_TTS_BOOKMARK,
    SPEI_WORD_BOUNDARY,
    SPEI_PHONEME,
    SPEI_SENTENCE_BOUNDARY,
    SPEI_VISEME,
    SPEI_TTS_AUDIO_LEVEL,
    SPEI_TTS_PRIVATE,
    SPEI_MIN_TTS,
    SPEI_MAX_TTS,
    SPEI_END_SR_STREAM,
    SPEI_SOUND_START,
    SPEI_SOUND_END,
    SPEI_PHRASE_START,
    SPEI_RECOGNITION,
    SPEI_HYPOTHESIS,
    SPEI_SR_BOOKMARK,
    SPEI_PROPERTY_NUM_CHANGE,
    SPEI_PROPERTY_STRING_CHANGE,
    SPEI_FALSE_RECOGNITION,
    SPEI_INTERFERENCE,
    SPEI_REQUEST_UI,
    SPEI_RECO_STATE_CHANGE,
    SPEI_ADAPTATION,
    SPEI_START_SR_STREAM,
    SPEI_RECO_OTHER_CONTEXT,
    SPEI_SR_AUDIO_LEVEL,
    SPEI_SR_RETAINEDAUDIO,
    SPEI_SR_PRIVATE,
};
pub const ACTIVE_CATEGORY_CHANGED: SPEVENTENUM = 53;
pub use um::sapi53::{
    SPEI_RESERVED5,
    SPEI_RESERVED6,
    SPEI_MIN_SR,
    SPEI_MAX_SR,
    SPEI_RESERVED1,
    SPEI_RESERVED2,
    SPEI_RESERVED3,
    SPFEI_FLAGCHECK,
    SPFEI_ALL_TTS_EVENTS,
    SPFEI_ALL_SR_EVENTS,
    SPFEI_ALL_EVENTS,
    SPFEI,
    SPEVENT,
    SPSERIALIZEDEVENT,
    SPSERIALIZEDEVENT64,
    SPEVENTEX,
    SPINTERFERENCE,
    SPINTERFERENCE_NONE,
    SPINTERFERENCE_NOISE,
    SPINTERFERENCE_NOSIGNAL,
    SPINTERFERENCE_TOOLOUD,
    SPINTERFERENCE_TOOQUIET,
    SPINTERFERENCE_TOOFAST,
    SPINTERFERENCE_TOOSLOW,
    SPINTERFERENCE_LATENCY_WARNING,
    SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN,
    SPINTERFERENCE_LATENCY_TRUNCATE_END,
    SPENDSRSTREAMFLAGS,
    SPESF_NONE,
    SPESF_STREAM_RELEASED,
    SPESF_EMULATED,
    SPVFEATURE,
    SPVFEATURE_STRESSED,
    SPVFEATURE_EMPHASIS,
    SPVISEMES,
    SP_VISEME_0,
    SP_VISEME_1,
    SP_VISEME_2,
    SP_VISEME_3,
    SP_VISEME_4,
    SP_VISEME_5,
    SP_VISEME_6,
    SP_VISEME_7,
    SP_VISEME_8,
    SP_VISEME_9,
    SP_VISEME_10,
    SP_VISEME_11,
    SP_VISEME_12,
    SP_VISEME_13,
    SP_VISEME_14,
    SP_VISEME_15,
    SP_VISEME_16,
    SP_VISEME_17,
    SP_VISEME_18,
    SP_VISEME_19,
    SP_VISEME_20,
    SP_VISEME_21,
    SPEVENTSOURCEINFO,
    ISpEventSource, ISpEventSourceVtbl,
    ISpEventSource2, ISpEventSource2Vtbl,
    ISpEventSink, ISpEventSinkVtbl,
    ISpStreamFormat, ISpStreamFormatVtbl,
    SPFILEMODE,
    SPFM_OPEN_READONLY,
    SPFM_OPEN_READWRITE,
    SPFM_CREATE,
    SPFM_CREATE_ALWAYS,
    SPFM_NUM_MODES,
    ISpStream, ISpStreamVtbl,
    ISpStreamFormatConverter, ISpStreamFormatConverterVtbl,
    SPAUDIOSTATE,
    SPAS_CLOSED,
    SPAS_STOP,
    SPAS_PAUSE,
    SPAS_RUN,
    SPAUDIOSTATUS,
    SPAUDIOBUFFERINFO,
    ISpAudio, ISpAudioVtbl,
    ISpMMSysAudio, ISpMMSysAudioVtbl,
    ISpTranscript, ISpTranscriptVtbl,
    SPDISPLAYATTRIBUTES,
    SPAF_ONE_TRAILING_SPACE,
    SPAF_TWO_TRAILING_SPACES,
    SPAF_CONSUME_LEADING_SPACES,
    SPAF_BUFFER_POSITION,
    SPAF_ALL,
    SPAF_USER_SPECIFIED,
    SPPHONEID,
    PSPPHONEID,
    PCSPPHONEID,
    SPPHRASEELEMENT,
    SPPHRASERULE,
    SPPHRASEPROPERTYUNIONTYPE,
    SPPPUT_UNUSED,
    SPPPUT_ARRAY_INDEX,
    SPPHRASEPROPERTY,
    SPPHRASEREPLACEMENT,
    SPSEMANTICERRORINFO,
    SPSEMANTICFORMAT,
    SPPHRASE_50,
// SPPHRASESIZE_500,
};
pub use um::sapi53::SPPHRASE as SPPHRASE_53;
STRUCT!{struct SPPHRASE {
    cbSize: ULONG,
    LangID: WORD,
    wHomophoneGroupId: WORD,
    ullGrammarID: ULONGLONG,
    ftStartTime: ULONGLONG,
    ullAudioStreamPosition: ULONGLONG,
    ulAudioSizeBytes: ULONG,
    ulRetainedSizeBytes: ULONG,
    ulAudioSizeTime: ULONG,
    Rule: SPPHRASERULE,
    pProperties: *const SPPHRASEPROPERTY,
    pElements: *const SPPHRASEELEMENT,
    cReplacements: ULONG,
    pReplacements: *const SPPHRASEREPLACEMENT,
    SREngineID: GUID,
    ulSREnginePrivateDataSize: ULONG,
    pSREnginePrivateData: *const BYTE,
    pSML: LPWSTR,
    pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
    SemanticTagFormat: SPSEMANTICFORMAT,
}}
pub use um::sapi53::{
    SPSERIALIZEDPHRASE,
    SPRULE,
    SPVALUETYPE,
    SPDF_PROPERTY,
    SPDF_REPLACEMENT,
    SPDF_RULE,
    SPDF_DISPLAYTEXT,
    SPDF_LEXICALFORM ,
    SPDF_PRONUNCIATION,
    SPDF_AUDIO,
    SPDF_ALTERNATES,
    SPDF_ALL,
    SPBINARYGRAMMAR,
    SPPHRASERNG,
    SPPR_ALL_ELEMENTS,
    SP_GETWHOLEPHRASE,
    SPRR_ALL_ELEMENTS,
    SPSTATEHANDLE,
    SPRECOEVENTFLAGS,
    SPREF_AutoPause,
    SPREF_Emulated,
    SPREF_SMLTimeout,
    SPREF_ExtendableParse,
    SPREF_ReSent,
    SPREF_Hypothesis,
    SPREF_FalseRecognition,
    SPPARTOFSPEECH,
    SPPS_NotOverriden,
    SPPS_Unknown,
    SPPS_Noun,
    SPPS_Verb,
    SPPS_Modifier,
    SPPS_Function,
    SPPS_Interjection,
    SPPS_Noncontent,
    SPPS_LMA,
    SPPS_SuppressWord,
    SPLEXICONTYPE,
    eLEXTYPE_USER,
    eLEXTYPE_APP,
    eLEXTYPE_VENDORLEXICON,
    eLEXTYPE_LETTERTOSOUND,
    eLEXTYPE_MORPHOLOGY,
    eLEXTYPE_RESERVED4,
    eLEXTYPE_USER_SHORTCUT,
    eLEXTYPE_RESERVED6,
    eLEXTYPE_RESERVED7,
    eLEXTYPE_RESERVED8,
    eLEXTYPE_RESERVED9,
    eLEXTYPE_RESERVED10,
    eLEXTYPE_PRIVATE1,
    eLEXTYPE_PRIVATE2,
    eLEXTYPE_PRIVATE3,
    eLEXTYPE_PRIVATE4,
    eLEXTYPE_PRIVATE5,
    eLEXTYPE_PRIVATE6,
    eLEXTYPE_PRIVATE7,
    eLEXTYPE_PRIVATE8,
    eLEXTYPE_PRIVATE9,
    eLEXTYPE_PRIVATE10,
    eLEXTYPE_PRIVATE11,
    eLEXTYPE_PRIVATE12,
    eLEXTYPE_PRIVATE13,
    eLEXTYPE_PRIVATE14,
    eLEXTYPE_PRIVATE15,
    eLEXTYPE_PRIVATE16,
    eLEXTYPE_PRIVATE17,
    eLEXTYPE_PRIVATE18,
    eLEXTYPE_PRIVATE19,
    eLEXTYPE_PRIVATE20,
    SPWORDTYPE,
    eWORDTYPE_ADDED,
    eWORDTYPE_DELETED,
    SPPRONUNCIATIONFLAGS,
    ePRONFLAG_USED,
    SPWORDPRONUNCIATION,
    SPWORDPRONUNCIATIONLIST,
    SPWORD,
    SPWORDLIST,
    ISpLexicon, ISpLexiconVtbl,
    ISpContainerLexicon, ISpContainerLexiconVtbl,
    SPSHORTCUTTYPE,
    SPSHT_NotOverriden,
    SPSHT_Unknown,
    SPSHT_EMAIL,
    SPSHT_OTHER,
    SPPS_RESERVED1,
    SPPS_RESERVED2,
    SPPS_RESERVED3,
    SPPS_RESERVED4,
    SPSHORTCUTPAIR,
    SPSHORTCUTPAIRLIST,
    ISpShortcut, ISpShortcutVtbl,
    ISpPhoneConverter, ISpPhoneConverterVtbl,
    ISpPhoneticAlphabetConverter, ISpPhoneticAlphabetConverterVtbl,
    ISpPhoneticAlphabetSelection, ISpPhoneticAlphabetSelectionVtbl,
    SPVPITCH,
    SPVACTIONS,
    SPVA_Speak,
    SPVA_Silence,
    SPVA_Pronounce,
    SPVA_Bookmark,
    SPVA_SpellOut,
    SPVA_Section,
    SPVA_ParseUnknownTag,
    SPVCONTEXT,
    SPVSTATE,
    SPRUNSTATE,
    SPRS_DONE,
    SPRS_IS_SPEAKING,
    SPVLIMITS,
    SPMIN_VOLUME,
    SPMAX_VOLUME,
    SPMIN_RATE,
    SPMAX_RATE,
    SPVPRIORITY,
    SPVPRI_NORMAL,
    SPVPRI_ALERT,
    SPVPRI_OVER,
    SPVOICESTATUS,
    SPEAKFLAGS,
    SPF_DEFAULT,
    SPF_ASYNC,
    SPF_PURGEBEFORESPEAK,
    SPF_IS_FILENAME,
    SPF_IS_XML,
    SPF_IS_NOT_XML,
    SPF_PERSIST_XML,
    SPF_NLP_SPEAK_PUNC,
    SPF_PARSE_SAPI,
    SPF_PARSE_SSML,
    SPF_PARSE_AUTODETECT,
    SPF_NLP_MASK,
    SPF_PARSE_MASK,
    SPF_VOICE_MASK,
    SPF_UNUSED_FLAGS,
    ISpVoice, ISpVoiceVtbl,
    ISpPhrase, ISpPhraseVtbl,
    ISpPhraseAlt, ISpPhraseAltVtbl,
    SPXMLRESULTOPTIONS,
    SPXRO_SML,
    SPXRO_Alternates_SML,
    ISpPhrase2, ISpPhrase2Vtbl,
    SPRECORESULTTIMES,
    SPSERIALIZEDRESULT,
    ISpRecoResult, ISpRecoResultVtbl,
    SPCOMMITFLAGS,
    SPCF_NONE,
    SPCF_ADD_TO_USER_LEXICON,
    SPCF_DEFINITE_CORRECTION,
    ISpRecoResult2, ISpRecoResult2Vtbl,
    ISpXMLRecoResult, ISpXMLRecoResultVtbl,
    SPTEXTSELECTIONINFO,
    SPWORDPRONOUNCEABLE,
    SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE,
    SPWP_UNKNOWN_WORD_PRONOUNCEABLE,
    SPWP_KNOWN_WORD_PRONOUNCEABLE,
    SPGRAMMARSTATE,
    SPGS_DISABLED,
    SPGS_ENABLED,
    SPGS_EXCLUSIVE,
    SPCONTEXTSTATE,
    SPCS_DISABLED,
    SPCS_ENABLED,
    SPRULESTATE,
    SPRS_INACTIVE,
    SPRS_ACTIVE,
    SPRS_ACTIVE_WITH_AUTO_PAUSE,
    SPWT_LEXICAL_NO_SPECIAL_CHARS,
    SPPROPERTYINFO,
    SPCFGRULEATTRIBUTES,
    SPRAF_TopLevel,
    SPRAF_Active,
    SPRAF_Export,
    SPRAF_Import,
    SPRAF_Interpreter,
    SPRAF_Dynamic,
    SPRAF_Root,
    SPRAF_AutoPause,
    SPRAF_UserDelimited,
    ISpGrammarBuilder, ISpGrammarBuilderVtbl,
    SPLOADOPTIONS,
    SPLO_STATIC,
    SPLO_DYNAMIC,
    ISpRecoGrammar, ISpRecoGrammarVtbl,
    SPMATCHINGMODE,
    AllWords,
    Subsequence,
    OrderedSubset,
    SubsequenceContentRequired,
    OrderedSubsetContentRequired,
    PHONETICALPHABET,
    PA_Ipa,
    PA_Ups,
    PA_Sapi,
    ISpGrammarBuilder2, ISpGrammarBuilder2Vtbl,
    SPRP_NORMAL,
    ISpRecoGrammar2, ISpRecoGrammar2Vtbl,
    ISpeechResourceLoader, ISpeechResourceLoaderVtbl,
    SPRECOCONTEXTSTATUS,
    SPBOOKMARKOPTIONS,
    SPBO_NONE,
    SPBO_PAUSE,
    SPBO_AHEAD,
    SPBO_TIME_UNITS,
    SPAUDIOOPTIONS,
    SPAO_NONE,
    SPAO_RETAIN_AUDIO,
    ISpRecoContext, ISpRecoContextVtbl,
    SPGRAMMAROPTIONS,
    SPGO_SAPI,
    SPGO_SRGS,
    SPGO_UPS,
    SPGO_SRGS_MS_SCRIPT,
    SPGO_SRGS_W3C_SCRIPT,
    SPGO_SRGS_STG_SCRIPT,
    SPGO_SRGS_SCRIPT,
    SPGO_FILE,
    SPGO_HTTP,
    SPGO_RES,
    SPGO_OBJECT,
    SPGO_DEFAULT,
    SPGO_ALL,
    SPADAPTATIONSETTINGS,
    SPADS_Default,
    SPADS_CurrentRecognizer,
    SPADS_RecoProfile,
    SPADS_Immediate,
    SPADS_Reset,
    SPADS_HighVolumeDataSource,
    SPADAPTATIONRELEVANCE,
    SPAR_Unknown,
    SPAR_Low,
    SPAR_Medium,
    SPAR_High,
    ISpRecoContext2, ISpRecoContext2Vtbl,
    ISpProperties, ISpPropertiesVtbl,
    SP_MAX_LANGIDS,
    SPRECOGNIZERSTATUS,
    SPWAVEFORMATTYPE,
    SPWF_INPUT,
    SPWF_SRENGINE,
    SPSTREAMFORMATTYPE,
    SPRECOSTATE,
    SPRST_INACTIVE,
    SPRST_ACTIVE,
    SPRST_ACTIVE_ALWAYS,
    SPRST_INACTIVE_WITH_PURGE,
    SPRST_NUM_STATES,
    ISpRecognizer, ISpRecognizerVtbl,
    ISpSerializeState, ISpSerializeStateVtbl,
    ISpRecognizer2, ISpRecognizer2Vtbl,
};
ENUM!{enum SPCATEGORYTYPE {
    SPCT_COMMAND,
    SPCT_DICTATION,
    SPCT_SLEEP,
    SPCT_SUB_COMMAND,
    SPCT_SUB_DICTATION,
}}
RIDL!(#[uuid(0xda0cd0f9, 0x14a2, 0x4f09, 0x8c, 0x2a, 0x85, 0xcc, 0x48, 0x97, 0x93, 0x45)]
interface ISpRecoCategory(ISpRecoCategoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(
        peCategoryType: *mut SPCATEGORYTYPE,
    ) -> HRESULT,
});
RIDL!(#[uuid(0xdf1b943c, 0x5838, 0x4aa2, 0x87, 0x06, 0xd7, 0xcd, 0x5b, 0x33, 0x34, 0x99)]
interface ISpRecognizer3(ISpRecognizer3Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCategory(
        categoryType: SPCATEGORYTYPE,
        ppCategory: *mut *mut ISpRecoCategory,
    ) -> HRESULT,
    fn SetActiveCategory(
        pCategory: *mut ISpRecoCategory,
    ) -> HRESULT,
    fn GetActiveCategory(
        ppCategory: *mut *mut ISpRecoCategory,
    ) -> HRESULT,
});
pub use um::sapi53::{
    SPNORMALIZATIONLIST,
    ISpEnginePronunciation, ISpEnginePronunciationVtbl,
    SPDISPLAYTOKEN,
    SPDISPLAYPHRASE,
    ISpDisplayAlternates, ISpDisplayAlternatesVtbl,
    SpeechLanguageId,
    DISPID_SpeechDataKey,
    DISPID_SDKSetBinaryValue,
    DISPID_SDKGetBinaryValue,
    DISPID_SDKSetStringValue,
    DISPID_SDKGetStringValue,
    DISPID_SDKSetLongValue,
    DISPID_SDKGetlongValue,
    DISPID_SDKOpenKey,
    DISPID_SDKCreateKey,
    DISPID_SDKDeleteKey,
    DISPID_SDKDeleteValue,
    DISPID_SDKEnumKeys,
    DISPID_SDKEnumValues,
    DISPID_SpeechObjectToken,
    DISPID_SOTId,
    DISPID_SOTDataKey,
    DISPID_SOTCategory,
    DISPID_SOTGetDescription,
    DISPID_SOTSetId,
    DISPID_SOTGetAttribute,
    DISPID_SOTCreateInstance,
    DISPID_SOTRemove,
    DISPID_SOTGetStorageFileName,
    DISPID_SOTRemoveStorageFileName,
    DISPID_SOTIsUISupported,
    DISPID_SOTDisplayUI,
    DISPID_SOTMatchesAttributes,
    SpeechDataKeyLocation,
    SDKLDefaultLocation,
    SDKLCurrentUser,
    SDKLLocalMachine,
    SDKLCurrentConfig,
    SpeechTokenContext,
    STCInprocServer,
    STCInprocHandler ,
    STCLocalServer,
    STCRemoteServer,
    STCAll,
    SpeechTokenShellFolder,
    STSF_AppData,
    STSF_LocalAppData,
    STSF_CommonAppData,
    STSF_FlagCreate,
    DISPID_SpeechObjectTokens,
    DISPID_SOTsCount,
    DISPID_SOTsItem,
    DISPID_SOTs_NewEnum,
    DISPID_SpeechObjectTokenCategory,
    DISPID_SOTCId,
    DISPID_SOTCDefault,
    DISPID_SOTCSetId,
    DISPID_SOTCGetDataKey,
    DISPID_SOTCEnumerateTokens,
    SpeechAudioFormatType,
    SAFTDefault,
    SAFTNoAssignedFormat,
    SAFTText,
    SAFTNonStandardFormat,
    SAFTExtendedAudioFormat,
    SAFT8kHz8BitMono,
    SAFT8kHz8BitStereo,
    SAFT8kHz16BitMono,
    SAFT8kHz16BitStereo,
    SAFT11kHz8BitMono,
    SAFT11kHz8BitStereo,
    SAFT11kHz16BitMono,
    SAFT11kHz16BitStereo,
    SAFT12kHz8BitMono,
    SAFT12kHz8BitStereo,
    SAFT12kHz16BitMono,
    SAFT12kHz16BitStereo,
    SAFT16kHz8BitMono,
    SAFT16kHz8BitStereo,
    SAFT16kHz16BitMono,
    SAFT16kHz16BitStereo,
    SAFT22kHz8BitMono,
    SAFT22kHz8BitStereo,
    SAFT22kHz16BitMono,
    SAFT22kHz16BitStereo,
    SAFT24kHz8BitMono,
    SAFT24kHz8BitStereo,
    SAFT24kHz16BitMono,
    SAFT24kHz16BitStereo,
    SAFT32kHz8BitMono,
    SAFT32kHz8BitStereo,
    SAFT32kHz16BitMono,
    SAFT32kHz16BitStereo,
    SAFT44kHz8BitMono,
    SAFT44kHz8BitStereo,
    SAFT44kHz16BitMono,
    SAFT44kHz16BitStereo,
    SAFT48kHz8BitMono,
    SAFT48kHz8BitStereo,
    SAFT48kHz16BitMono,
    SAFT48kHz16BitStereo,
    SAFTTrueSpeech_8kHz1BitMono,
    SAFTCCITT_ALaw_8kHzMono,
    SAFTCCITT_ALaw_8kHzStereo,
    SAFTCCITT_ALaw_11kHzMono,
    SAFTCCITT_ALaw_11kHzStereo,
    SAFTCCITT_ALaw_22kHzMono,
    SAFTCCITT_ALaw_22kHzStereo,
    SAFTCCITT_ALaw_44kHzMono,
    SAFTCCITT_ALaw_44kHzStereo,
    SAFTCCITT_uLaw_8kHzMono,
    SAFTCCITT_uLaw_8kHzStereo,
    SAFTCCITT_uLaw_11kHzMono,
    SAFTCCITT_uLaw_11kHzStereo,
    SAFTCCITT_uLaw_22kHzMono,
    SAFTCCITT_uLaw_22kHzStereo,
    SAFTCCITT_uLaw_44kHzMono,
    SAFTCCITT_uLaw_44kHzStereo,
    SAFTADPCM_8kHzMono,
    SAFTADPCM_8kHzStereo,
    SAFTADPCM_11kHzMono,
    SAFTADPCM_11kHzStereo,
    SAFTADPCM_22kHzMono,
    SAFTADPCM_22kHzStereo,
    SAFTADPCM_44kHzMono,
    SAFTADPCM_44kHzStereo,
    SAFTGSM610_8kHzMono,
    SAFTGSM610_11kHzMono,
    SAFTGSM610_22kHzMono,
    SAFTGSM610_44kHzMono,
    DISPID_SpeechAudioFormat,
    DISPID_SAFType,
    DISPID_SAFGuid,
    DISPID_SAFGetWaveFormatEx,
    DISPID_SAFSetWaveFormatEx,
    DISPID_SpeechBaseStream,
    DISPID_SBSFormat,
    DISPID_SBSRead,
    DISPID_SBSWrite,
    DISPID_SBSSeek,
    SpeechStreamSeekPositionType,
    SSSPTRelativeToStart,
    SSSPTRelativeToCurrentPosition,
    SSSPTRelativeToEnd,
    DISPID_SpeechAudio,
    DISPID_SAStatus,
    DISPID_SABufferInfo,
    DISPID_SADefaultFormat,
    DISPID_SAVolume,
    DISPID_SABufferNotifySize,
    DISPID_SAEventHandle,
    DISPID_SASetState,
    SpeechAudioState,
    SASClosed,
    SASStop,
    SASPause,
    SASRun,
    DISPID_SpeechMMSysAudio,
    DISPID_SMSADeviceId,
    DISPID_SMSALineId,
    DISPID_SMSAMMHandle,
    DISPID_SpeechFileStream,
    DISPID_SFSOpen,
    DISPID_SFSClose,
    SpeechStreamFileMode,
    SSFMOpenForRead,
    SSFMOpenReadWrite,
    SSFMCreate,
    SSFMCreateForWrite,
    DISPID_SpeechCustomStream,
    DISPID_SCSBaseStream,
    DISPID_SpeechMemoryStream,
    DISPID_SMSSetData,
    DISPID_SMSGetData,
    DISPID_SpeechAudioStatus,
    DISPID_SASFreeBufferSpace,
    DISPID_SASNonBlockingIO,
    DISPID_SASState,
    DISPID_SASCurrentSeekPosition,
    DISPID_SASCurrentDevicePosition,
    DISPID_SpeechAudioBufferInfo,
    DISPID_SABIMinNotification,
    DISPID_SABIBufferSize,
    DISPID_SABIEventBias,
    DISPID_SpeechWaveFormatEx,
    DISPID_SWFEFormatTag,
    DISPID_SWFEChannels,
    DISPID_SWFESamplesPerSec,
    DISPID_SWFEAvgBytesPerSec,
    DISPID_SWFEBlockAlign,
    DISPID_SWFEBitsPerSample,
    DISPID_SWFEExtraData,
    DISPID_SpeechVoice,
    DISPID_SVStatus,
    DISPID_SVVoice,
    DISPID_SVAudioOutput,
    DISPID_SVAudioOutputStream,
    DISPID_SVRate,
    DISPID_SVVolume,
    DISPID_SVAllowAudioOuputFormatChangesOnNextSet,
    DISPID_SVEventInterests,
    DISPID_SVPriority,
    DISPID_SVAlertBoundary,
    DISPID_SVSyncronousSpeakTimeout,
    DISPID_SVSpeak,
    DISPID_SVSpeakStream,
    DISPID_SVPause,
    DISPID_SVResume,
    DISPID_SVSkip,
    DISPID_SVGetVoices,
    DISPID_SVGetAudioOutputs,
    DISPID_SVWaitUntilDone,
    DISPID_SVSpeakCompleteEvent,
    DISPID_SVIsUISupported,
    DISPID_SVDisplayUI,
    SpeechVoicePriority,
    SVPNormal,
    SVPAlert,
    SVPOver,
    SpeechVoiceSpeakFlags,
    SVSFDefault,
    SVSFlagsAsync,
    SVSFPurgeBeforeSpeak,
    SVSFIsFilename,
    SVSFIsXML,
    SVSFIsNotXML,
    SVSFPersistXML,
    SVSFNLPSpeakPunc,
    SVSFParseSapi,
    SVSFParseSsml,
    SVSFParseAutodetect,
    SVSFNLPMask,
    SVSFParseMask,
    SVSFVoiceMask,
    SVSFUnusedFlags,
    SpeechVoiceEvents,
    SVEStartInputStream,
    SVEEndInputStream,
    SVEVoiceChange,
    SVEBookmark,
    SVEWordBoundary,
    SVEPhoneme,
    SVESentenceBoundary,
    SVEViseme,
    SVEAudioLevel,
    SVEPrivate,
    SVEAllEvents,
    DISPID_SpeechVoiceStatus,
    DISPID_SVSCurrentStreamNumber,
    DISPID_SVSLastStreamNumberQueued,
    DISPID_SVSLastResult,
    DISPID_SVSRunningState,
    DISPID_SVSInputWordPosition,
    DISPID_SVSInputWordLength,
    DISPID_SVSInputSentencePosition,
    DISPID_SVSInputSentenceLength,
    DISPID_SVSLastBookmark,
    DISPID_SVSLastBookmarkId,
    DISPID_SVSPhonemeId,
    DISPID_SVSVisemeId,
    SpeechRunState,
    SRSEDone,
    SRSEIsSpeaking,
    SpeechVisemeType,
    SVP_0,
    SVP_1,
    SVP_2,
    SVP_3,
    SVP_4,
    SVP_5,
    SVP_6,
    SVP_7,
    SVP_8,
    SVP_9,
    SVP_10,
    SVP_11,
    SVP_12,
    SVP_13,
    SVP_14,
    SVP_15,
    SVP_16,
    SVP_17,
    SVP_18,
    SVP_19,
    SVP_20,
    SVP_21,
    SpeechVisemeFeature,
    SVF_None,
    SVF_Stressed,
    SVF_Emphasis,
    DISPID_SpeechVoiceEvent,
    DISPID_SVEStreamStart,
    DISPID_SVEStreamEnd,
    DISPID_SVEVoiceChange,
    DISPID_SVEBookmark,
    DISPID_SVEWord,
    DISPID_SVEPhoneme,
    DISPID_SVESentenceBoundary,
    DISPID_SVEViseme,
    DISPID_SVEAudioLevel,
    DISPID_SVEEnginePrivate,
    DISPID_SpeechRecognizer,
    DISPID_SRRecognizer,
    DISPID_SRAllowAudioInputFormatChangesOnNextSet,
    DISPID_SRAudioInput,
    DISPID_SRAudioInputStream,
    DISPID_SRIsShared,
    DISPID_SRState,
    DISPID_SRStatus,
    DISPID_SRProfile,
    DISPID_SREmulateRecognition,
    DISPID_SRCreateRecoContext,
    DISPID_SRGetFormat,
    DISPID_SRSetPropertyNumber,
    DISPID_SRGetPropertyNumber,
    DISPID_SRSetPropertyString,
    DISPID_SRGetPropertyString,
    DISPID_SRIsUISupported,
    DISPID_SRDisplayUI,
    DISPID_SRGetRecognizers,
    DISPID_SVGetAudioInputs,
    DISPID_SVGetProfiles,
    SpeechRecognizerState,
    SRSInactive,
    SRSActive,
    SRSActiveAlways,
    SRSInactiveWithPurge,
    SpeechDisplayAttributes,
    SDA_No_Trailing_Space,
    SDA_One_Trailing_Space,
    SDA_Two_Trailing_Spaces,
    SDA_Consume_Leading_Spaces,
    SpeechFormatType,
    SFTInput,
    SFTSREngine,
    SpeechEmulationCompareFlags,
    SECFIgnoreCase,
    SECFIgnoreKanaType,
    SECFIgnoreWidth,
    SECFNoSpecialChars,
    SECFEmulateResult,
    SECFDefault,
    DISPID_SpeechRecognizerStatus,
    DISPID_SRSAudioStatus,
    DISPID_SRSCurrentStreamPosition,
    DISPID_SRSCurrentStreamNumber,
    DISPID_SRSNumberOfActiveRules,
    DISPID_SRSClsidEngine,
    DISPID_SRSSupportedLanguages,
    DISPID_SpeechRecoContext,
    DISPID_SRCRecognizer,
    DISPID_SRCAudioInInterferenceStatus,
    DISPID_SRCRequestedUIType,
    DISPID_SRCVoice,
    DISPID_SRAllowVoiceFormatMatchingOnNextSet,
    DISPID_SRCVoicePurgeEvent,
    DISPID_SRCEventInterests,
    DISPID_SRCCmdMaxAlternates,
    DISPID_SRCState,
    DISPID_SRCRetainedAudio,
    DISPID_SRCRetainedAudioFormat,
    DISPID_SRCPause,
    DISPID_SRCResume,
    DISPID_SRCCreateGrammar,
    DISPID_SRCCreateResultFromMemory,
    DISPID_SRCBookmark,
    DISPID_SRCSetAdaptationData,
    SpeechRetainedAudioOptions,
    SRAONone,
    SRAORetainAudio,
    SpeechBookmarkOptions,
    SBONone,
    SBOPause,
    SpeechInterference,
    SINone,
    SINoise,
    SINoSignal,
    SITooLoud,
    SITooQuiet,
    SITooFast,
    SITooSlow,
    SpeechRecoEvents,
    SREStreamEnd,
    SRESoundStart,
    SRESoundEnd,
    SREPhraseStart,
    SRERecognition,
    SREHypothesis,
    SREBookmark,
    SREPropertyNumChange,
    SREPropertyStringChange,
    SREFalseRecognition,
    SREInterference,
    SRERequestUI,
    SREStateChange,
    SREAdaptation,
    SREStreamStart,
    SRERecoOtherContext,
    SREAudioLevel,
    SREPrivate,
    SREAllEvents,
    SpeechRecoContextState,
    SRCS_Disabled,
    SRCS_Enabled,
    DISPIDSPRG,
    DISPID_SRGId,
    DISPID_SRGRecoContext,
    DISPID_SRGState,
    DISPID_SRGRules,
    DISPID_SRGReset,
    DISPID_SRGCommit,
    DISPID_SRGCmdLoadFromFile,
    DISPID_SRGCmdLoadFromObject,
    DISPID_SRGCmdLoadFromResource,
    DISPID_SRGCmdLoadFromMemory,
    DISPID_SRGCmdLoadFromProprietaryGrammar,
    DISPID_SRGCmdSetRuleState,
    DISPID_SRGCmdSetRuleIdState,
    DISPID_SRGDictationLoad,
    DISPID_SRGDictationUnload,
    DISPID_SRGDictationSetState,
    DISPID_SRGSetWordSequenceData,
    DISPID_SRGSetTextSelection,
    DISPID_SRGIsPronounceable,
    SpeechLoadOption,
    SLOStatic,
    SLODynamic,
    SpeechWordPronounceable,
    SWPUnknownWordUnpronounceable,
    SWPUnknownWordPronounceable,
    SWPKnownWordPronounceable,
    SpeechGrammarState,
    SGSEnabled,
    SGSDisabled,
    SGSExclusive,
    SpeechRuleState,
    SGDSInactive,
    SGDSActive,
    SGDSActiveWithAutoPause,
    SGDSActiveUserDelimited,
    SpeechRuleAttributes,
    SRATopLevel,
    SRADefaultToActive,
    SRAExport,
    SRAImport,
    SRAInterpreter,
    SRADynamic,
    SRARoot,
    SpeechGrammarWordType,
    SGDisplay,
    SGLexical,
    SGPronounciation,
    SGLexicalNoSpecialChars,
    DISPID_SpeechRecoContextEvents,
    DISPID_SRCEStartStream,
    DISPID_SRCEEndStream,
    DISPID_SRCEBookmark,
    DISPID_SRCESoundStart,
    DISPID_SRCESoundEnd,
    DISPID_SRCEPhraseStart,
    DISPID_SRCERecognition,
    DISPID_SRCEHypothesis,
    DISPID_SRCEPropertyNumberChange,
    DISPID_SRCEPropertyStringChange,
    DISPID_SRCEFalseRecognition,
    DISPID_SRCEInterference,
    DISPID_SRCERequestUI,
    DISPID_SRCERecognizerStateChange,
    DISPID_SRCEAdaptation,
    DISPID_SRCERecognitionForOtherContext,
    DISPID_SRCEAudioLevel,
    DISPID_SRCEEnginePrivate,
    SpeechRecognitionType,
    SRTStandard,
    SRTAutopause,
    SRTEmulated,
    SRTSMLTimeout,
    SRTExtendableParse,
    SRTReSent,
    DISPID_SpeechGrammarRule,
    DISPID_SGRAttributes,
    DISPID_SGRInitialState,
    DISPID_SGRName,
    DISPID_SGRId,
    DISPID_SGRClear,
    DISPID_SGRAddResource,
    DISPID_SGRAddState,
    DISPID_SpeechGrammarRules,
    DISPID_SGRsCount,
    DISPID_SGRsDynamic,
    DISPID_SGRsAdd,
    DISPID_SGRsCommit,
    DISPID_SGRsCommitAndSave,
    DISPID_SGRsFindRule,
    DISPID_SGRsItem,
    DISPID_SGRs_NewEnum,
    DISPID_SpeechGrammarRuleState,
    DISPID_SGRSRule,
    DISPID_SGRSTransitions,
    DISPID_SGRSAddWordTransition,
    DISPID_SGRSAddRuleTransition,
    DISPID_SGRSAddSpecialTransition,
    SpeechSpecialTransitionType,
    SSTTWildcard,
    SSTTDictation,
    SSTTTextBuffer,
    DISPID_SpeechGrammarRuleStateTransitions,
    DISPID_SGRSTsCount,
    DISPID_SGRSTsItem,
    DISPID_SGRSTs_NewEnum,
    DISPID_SpeechGrammarRuleStateTransition,
    DISPID_SGRSTType,
    DISPID_SGRSTText,
    DISPID_SGRSTRule,
    DISPID_SGRSTWeight,
    DISPID_SGRSTPropertyName,
    DISPID_SGRSTPropertyId,
    DISPID_SGRSTPropertyValue,
    DISPID_SGRSTNextState,
    SpeechGrammarRuleStateTransitionType,
    SGRSTTEpsilon,
    SGRSTTWord,
    SGRSTTRule,
    SGRSTTDictation,
    SGRSTTWildcard,
    SGRSTTTextBuffer,
    DISPIDSPTSI,
    DISPIDSPTSI_ActiveOffset,
    DISPIDSPTSI_ActiveLength,
    DISPIDSPTSI_SelectionOffset,
    DISPIDSPTSI_SelectionLength,
    DISPID_SpeechRecoResult,
    DISPID_SRRRecoContext,
    DISPID_SRRTimes,
    DISPID_SRRAudioFormat,
    DISPID_SRRPhraseInfo,
    DISPID_SRRAlternates,
    DISPID_SRRAudio,
    DISPID_SRRSpeakAudio,
    DISPID_SRRSaveToMemory,
    DISPID_SRRDiscardResultInfo,
    SpeechDiscardType,
    SDTProperty,
    SDTReplacement,
    SDTRule,
    SDTDisplayText,
    SDTLexicalForm,
    SDTPronunciation,
    SDTAudio,
    SDTAlternates,
    SDTAll,
    DISPID_SpeechXMLRecoResult,
    DISPID_SRRGetXMLResult,
    DISPID_SRRGetXMLErrorInfo,
    DISPID_SpeechRecoResult2,
    DISPID_SRRSetTextFeedback,
    DISPID_SpeechPhraseBuilder,
    DISPID_SPPBRestorePhraseFromMemory,
    DISPID_SpeechRecoResultTimes,
    DISPID_SRRTStreamTime,
    DISPID_SRRTLength,
    DISPID_SRRTTickCount,
    DISPID_SRRTOffsetFromStart,
    DISPID_SpeechPhraseAlternate,
    DISPID_SPARecoResult,
    DISPID_SPAStartElementInResult,
    DISPID_SPANumberOfElementsInResult,
    DISPID_SPAPhraseInfo,
    DISPID_SPACommit,
    DISPID_SpeechPhraseAlternates,
    DISPID_SPAsCount,
    DISPID_SPAsItem,
    DISPID_SPAs_NewEnum,
    DISPID_SpeechPhraseInfo,
    DISPID_SPILanguageId,
    DISPID_SPIGrammarId,
    DISPID_SPIStartTime,
    DISPID_SPIAudioStreamPosition,
    DISPID_SPIAudioSizeBytes,
    DISPID_SPIRetainedSizeBytes,
    DISPID_SPIAudioSizeTime,
    DISPID_SPIRule,
    DISPID_SPIProperties,
    DISPID_SPIElements,
    DISPID_SPIReplacements,
    DISPID_SPIEngineId,
    DISPID_SPIEnginePrivateData,
    DISPID_SPISaveToMemory,
    DISPID_SPIGetText,
    DISPID_SPIGetDisplayAttributes,
    DISPID_SpeechPhraseElement,
    DISPID_SPEAudioTimeOffset,
    DISPID_SPEAudioSizeTime,
    DISPID_SPEAudioStreamOffset,
    DISPID_SPEAudioSizeBytes,
    DISPID_SPERetainedStreamOffset,
    DISPID_SPERetainedSizeBytes,
    DISPID_SPEDisplayText,
    DISPID_SPELexicalForm,
    DISPID_SPEPronunciation,
    DISPID_SPEDisplayAttributes,
    DISPID_SPERequiredConfidence,
    DISPID_SPEActualConfidence,
    DISPID_SPEEngineConfidence,
    SpeechEngineConfidence,
    SECLowConfidence,
    SECNormalConfidence,
    SECHighConfidence,
    DISPID_SpeechPhraseElements,
    DISPID_SPEsCount,
    DISPID_SPEsItem,
    DISPID_SPEs_NewEnum,
    DISPID_SpeechPhraseReplacement,
    DISPID_SPRDisplayAttributes,
    DISPID_SPRText,
    DISPID_SPRFirstElement,
    DISPID_SPRNumberOfElements,
    DISPID_SpeechPhraseReplacements,
    DISPID_SPRsCount,
    DISPID_SPRsItem,
    DISPID_SPRs_NewEnum,
    DISPID_SpeechPhraseProperty,
    DISPID_SPPName,
    DISPID_SPPId,
    DISPID_SPPValue,
    DISPID_SPPFirstElement,
    DISPID_SPPNumberOfElements,
    DISPID_SPPEngineConfidence,
    DISPID_SPPConfidence,
    DISPID_SPPParent,
    DISPID_SPPChildren,
    DISPID_SpeechPhraseProperties,
    DISPID_SPPsCount,
    DISPID_SPPsItem,
    DISPID_SPPs_NewEnum,
    DISPID_SpeechPhraseRule,
    DISPID_SPRuleName,
    DISPID_SPRuleId,
    DISPID_SPRuleFirstElement,
    DISPID_SPRuleNumberOfElements,
    DISPID_SPRuleParent,
    DISPID_SPRuleChildren,
    DISPID_SPRuleConfidence,
    DISPID_SPRuleEngineConfidence,
    DISPID_SpeechPhraseRules,
    DISPID_SPRulesCount,
    DISPID_SPRulesItem,
    DISPID_SPRules_NewEnum,
    DISPID_SpeechLexicon,
    DISPID_SLGenerationId,
    DISPID_SLGetWords,
    DISPID_SLAddPronunciation,
    DISPID_SLAddPronunciationByPhoneIds,
    DISPID_SLRemovePronunciation,
    DISPID_SLRemovePronunciationByPhoneIds,
    DISPID_SLGetPronunciations,
    DISPID_SLGetGenerationChange,
    SpeechLexiconType,
    SLTUser,
    SLTApp,
    SpeechPartOfSpeech,
    SPSNotOverriden,
    SPSUnknown,
    SPSNoun,
    SPSVerb,
    SPSModifier,
    SPSFunction,
    SPSInterjection,
    SPSLMA,
    SPSSuppressWord,
    DISPID_SpeechLexiconWords,
    DISPID_SLWsCount,
    DISPID_SLWsItem,
    DISPID_SLWs_NewEnum,
    SpeechWordType,
    SWTAdded,
    SWTDeleted,
    DISPID_SpeechLexiconWord,
    DISPID_SLWLangId,
    DISPID_SLWType,
    DISPID_SLWWord,
    DISPID_SLWPronunciations,
    DISPID_SpeechLexiconProns,
    DISPID_SLPsCount,
    DISPID_SLPsItem,
    DISPID_SLPs_NewEnum,
    DISPID_SpeechLexiconPronunciation,
    DISPID_SLPType,
    DISPID_SLPLangId,
    DISPID_SLPPartOfSpeech,
    DISPID_SLPPhoneIds,
    DISPID_SLPSymbolic,
    DISPID_SpeechPhoneConverter,
    DISPID_SPCLangId,
    DISPID_SPCPhoneToId,
    DISPID_SPCIdToPhone,
    LIBID_SpeechLib,
    ISpeechDataKey, ISpeechDataKeyVtbl,
    ISpeechObjectToken, ISpeechObjectTokenVtbl,
    ISpeechObjectTokens, ISpeechObjectTokensVtbl,
    ISpeechObjectTokenCategory, ISpeechObjectTokenCategoryVtbl,
    ISpeechAudioBufferInfo, ISpeechAudioBufferInfoVtbl,
    ISpeechAudioStatus, ISpeechAudioStatusVtbl,
    ISpeechAudioFormat, ISpeechAudioFormatVtbl,
    ISpeechWaveFormatEx, ISpeechWaveFormatExVtbl,
    ISpeechBaseStream, ISpeechBaseStreamVtbl,
    ISpeechFileStream, ISpeechFileStreamVtbl,
    ISpeechMemoryStream, ISpeechMemoryStreamVtbl,
    ISpeechCustomStream, ISpeechCustomStreamVtbl,
    ISpeechAudio, ISpeechAudioVtbl,
    ISpeechMMSysAudio, ISpeechMMSysAudioVtbl,
    ISpeechVoice, ISpeechVoiceVtbl,
    ISpeechVoiceStatus, ISpeechVoiceStatusVtbl,
    _ISpeechVoiceEvents, _ISpeechVoiceEventsVtbl,
    ISpeechRecognizer, ISpeechRecognizerVtbl,
    ISpeechRecognizerStatus, ISpeechRecognizerStatusVtbl,
    ISpeechRecoContext, ISpeechRecoContextVtbl,
    ISpeechRecoGrammar, ISpeechRecoGrammarVtbl,
    _ISpeechRecoContextEvents, _ISpeechRecoContextEventsVtbl,
    ISpeechGrammarRule, ISpeechGrammarRuleVtbl,
    ISpeechGrammarRules, ISpeechGrammarRulesVtbl,
    ISpeechGrammarRuleState, ISpeechGrammarRuleStateVtbl,
    ISpeechGrammarRuleStateTransition, ISpeechGrammarRuleStateTransitionVtbl,
    ISpeechGrammarRuleStateTransitions, ISpeechGrammarRuleStateTransitionsVtbl,
    ISpeechTextSelectionInformation, ISpeechTextSelectionInformationVtbl,
    ISpeechRecoResult, ISpeechRecoResultVtbl,
    ISpeechRecoResult2, ISpeechRecoResult2Vtbl,
    ISpeechRecoResultTimes, ISpeechRecoResultTimesVtbl,
    ISpeechPhraseAlternate, ISpeechPhraseAlternateVtbl,
    ISpeechPhraseAlternates, ISpeechPhraseAlternatesVtbl,
    ISpeechPhraseInfo, ISpeechPhraseInfoVtbl,
    ISpeechPhraseElement, ISpeechPhraseElementVtbl,
    ISpeechPhraseElements, ISpeechPhraseElementsVtbl,
    ISpeechPhraseReplacement, ISpeechPhraseReplacementVtbl,
    ISpeechPhraseReplacements, ISpeechPhraseReplacementsVtbl,
    ISpeechPhraseProperty, ISpeechPhrasePropertyVtbl,
    ISpeechPhraseProperties, ISpeechPhrasePropertiesVtbl,
    ISpeechPhraseRule, ISpeechPhraseRuleVtbl,
    ISpeechPhraseRules, ISpeechPhraseRulesVtbl,
    ISpeechLexicon, ISpeechLexiconVtbl,
    ISpeechLexiconWords, ISpeechLexiconWordsVtbl,
    ISpeechLexiconWord, ISpeechLexiconWordVtbl,
    ISpeechLexiconPronunciations, ISpeechLexiconPronunciationsVtbl,
    ISpeechLexiconPronunciation, ISpeechLexiconPronunciationVtbl,
    Speech_Default_Weight,
    Speech_Max_Word_Length,
    Speech_Max_Pron_Length,
    Speech_StreamPos_Asap,
    Speech_StreamPos_RealTime,
    SpeechAllElements,
    ISpeechXMLRecoResult, ISpeechXMLRecoResultVtbl,
    ISpeechRecoResultDispatch, ISpeechRecoResultDispatchVtbl,
    ISpeechPhraseInfoBuilder, ISpeechPhraseInfoBuilderVtbl,
    ISpeechPhoneConverter, ISpeechPhoneConverterVtbl,
    CLSID_SpNotifyTranslator,
    CLSID_SpObjectTokenCategory,
    CLSID_SpObjectToken,
    CLSID_SpResourceManager,
    CLSID_SpStreamFormatConverter,
    CLSID_SpMMAudioEnum,
    CLSID_SpMMAudioIn,
    CLSID_SpMMAudioOut,
    CLSID_SpStream,
    CLSID_SpVoice,
    CLSID_SpSharedRecoContext,
    CLSID_SpInprocRecognizer,
    CLSID_SpSharedRecognizer,
    CLSID_SpLexicon,
    CLSID_SpUnCompressedLexicon,
    CLSID_SpCompressedLexicon,
    CLSID_SpShortcut,
    CLSID_SpPhoneConverter,
    CLSID_SpPhoneticAlphabetConverter,
    CLSID_SpNullPhoneConverter,
    CLSID_SpTextSelectionInformation,
    CLSID_SpPhraseInfoBuilder,
    CLSID_SpAudioFormat,
    CLSID_SpWaveFormatEx,
    CLSID_SpInProcRecoContext,
    CLSID_SpCustomStream,
    CLSID_SpFileStream,
    CLSID_SpMemoryStream,
};
