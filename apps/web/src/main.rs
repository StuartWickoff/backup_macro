use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use macro_journal_shared::{Entry, EntryDraft};
use pulldown_cmark::{html, Event, Options, Parser};

const API_BASE: &str = "http://127.0.0.1:3000";

#[derive(Clone, Copy)]
struct CurrencyInfo {
    code: &'static str,
    name: &'static str,
    country: &'static str,
    bank: &'static str,
    flag: &'static str,
    external: bool,
}

const CURRENCIES: [CurrencyInfo; 9] = [
    CurrencyInfo {
        code: "USD",
        name: "Dollar américain",
        country: "États-Unis",
        bank: "Federal Reserve",
        flag: "us",
        external: false,
    },
    CurrencyInfo {
        code: "EUR",
        name: "Euro",
        country: "Zone euro",
        bank: "Banque centrale européenne",
        flag: "eu",
        external: false,
    },
    CurrencyInfo {
        code: "GBP",
        name: "Livre sterling",
        country: "Royaume-Uni",
        bank: "Bank of England",
        flag: "gb",
        external: false,
    },
    CurrencyInfo {
        code: "JPY",
        name: "Yen japonais",
        country: "Japon",
        bank: "Bank of Japan",
        flag: "jp",
        external: false,
    },
    CurrencyInfo {
        code: "CHF",
        name: "Franc suisse",
        country: "Suisse",
        bank: "Swiss National Bank",
        flag: "ch",
        external: false,
    },
    CurrencyInfo {
        code: "CAD",
        name: "Dollar canadien",
        country: "Canada",
        bank: "Bank of Canada",
        flag: "ca",
        external: false,
    },
    CurrencyInfo {
        code: "AUD",
        name: "Dollar australien",
        country: "Australie",
        bank: "Reserve Bank of Australia",
        flag: "au",
        external: false,
    },
    CurrencyInfo {
        code: "NZD",
        name: "Dollar néo-zélandais",
        country: "Nouvelle-Zélande",
        bank: "Reserve Bank of New Zealand",
        flag: "nz",
        external: false,
    },
    CurrencyInfo {
        code: "CNY",
        name: "Yuan renminbi",
        country: "Chine",
        bank: "People’s Bank of China",
        flag: "cn",
        external: true,
    },
];

const UNASSIGNED_CURRENCY: CurrencyInfo = CurrencyInfo {
    code: "UNASSIGNED",
    name: "Notes à classer",
    country: "Anciennes notes",
    bank: "Aucune devise associée",
    flag: "unassigned",
    external: false,
};

fn currency_info(code: &str) -> Option<CurrencyInfo> {
    if code == UNASSIGNED_CURRENCY.code {
        Some(UNASSIGNED_CURRENCY)
    } else {
        CURRENCIES
            .iter()
            .find(|currency| currency.code == code)
            .copied()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (active_section, set_active_section) = signal("currencies".to_string());
    let (entries, set_entries) = signal::<Vec<Entry>>(Vec::new());
    let (active_id, set_active_id) = signal::<Option<i64>>(None);
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (search, set_search) = signal(String::new());
    let (selected_currency, set_selected_currency) = signal::<Option<String>>(None);
    let (editor_open, set_editor_open) = signal(false);
    let (editing, set_editing) = signal(false);
    let (preview_mode, set_preview_mode) = signal(false);
    let (save_status, set_save_status) = signal("BASE LOCALE · CONNEXION".to_string());
    let (api_status, set_api_status) = signal(false);
    let (sidebar_visible, set_sidebar_visible) = signal(true);

    let load_entries = move |currency_code: String| {
        spawn_local(async move {
            match fetch_entries(&currency_code).await {
                Ok(items) => {
                    let still_selected = selected_currency.get().as_deref()
                        == Some(currency_code.as_str())
                        && active_section.get() == "currencies";
                    let still_unassigned =
                        currency_code == "UNASSIGNED" && active_section.get() == "journal";
                    if still_selected || still_unassigned {
                        set_entries.set(items);
                        set_save_status.set("BASE LOCALE · CONNECTÉE".to_string());
                        set_api_status.set(true);
                    }
                }
                Err(error) => {
                    let still_selected = selected_currency.get().as_deref()
                        == Some(currency_code.as_str())
                        && active_section.get() == "currencies";
                    let still_unassigned =
                        currency_code == "UNASSIGNED" && active_section.get() == "journal";
                    if still_selected || still_unassigned {
                        set_save_status.set(format!("API INDISPONIBLE · {error}"));
                        set_api_status.set(false);
                    }
                }
            }
        });
    };

    spawn_local(async move {
        match check_api().await {
            Ok(()) => {
                set_api_status.set(true);
                set_save_status.set("BASE LOCALE · CONNECTÉE".to_string());
            }
            Err(error) => {
                set_api_status.set(false);
                set_save_status.set(format!("API INDISPONIBLE · {error}"));
            }
        }
    });

    let new_note = move |_| {
        set_active_id.set(None);
        set_title.set(String::new());
        set_content.set(String::new());
        set_editor_open.set(true);
        set_editing.set(true);
        set_preview_mode.set(false);
        set_save_status.set("NOUVEAU BROUILLON · NON ENREGISTRÉ".to_string());
    };

    let open_currency = move |code: &'static str| {
        set_active_section.set("currencies".to_string());
        set_selected_currency.set(Some(code.to_string()));
        set_editor_open.set(false);
        set_search.set(String::new());
        set_entries.set(Vec::new());
        load_entries(code.to_string());
    };

    let open_unassigned = move |_| {
        set_active_section.set("journal".to_string());
        set_selected_currency.set(None);
        set_editor_open.set(false);
        set_search.set(String::new());
        set_entries.set(Vec::new());
        load_entries("UNASSIGNED".to_string());
    };

    let save_note = move |_| {
        let draft = EntryDraft {
            title: title.get().trim().to_string(),
            content: content.get(),
        };
        let existing_id = active_id.get();
        let currency_code = selected_currency
            .get()
            .unwrap_or_else(|| "UNASSIGNED".to_string());

        if draft.title.is_empty() {
            set_save_status.set("UN TITRE EST REQUIS".to_string());
            return;
        }

        set_save_status.set("ENREGISTREMENT…".to_string());

        spawn_local(async move {
            let result = match existing_id {
                Some(id) => update_entry(id, &currency_code, draft).await,
                None => create_entry(&currency_code, draft).await,
            };

            match result {
                Ok(saved) => {
                    set_active_id.set(Some(saved.id));
                    set_entries.update(|all| {
                        if let Some(existing) = all.iter_mut().find(|item| item.id == saved.id) {
                            *existing = saved.clone();
                        } else {
                            all.insert(0, saved.clone());
                        }
                        all.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
                    });
                    set_save_status.set(format!(
                        "ENREGISTRÉ LOCALEMENT · {}",
                        format_date(&saved.updated_at)
                    ));
                    set_api_status.set(true);
                    set_editor_open.set(false);
                    set_editing.set(false);
                }
                Err(error) => {
                    set_save_status.set(format!("ÉCHEC DE L’ENREGISTREMENT · {error}"));
                    set_api_status.set(false);
                }
            }
        });
    };

    let preview_html = move || render_markdown(&content.get());
    let character_count = move || content.get().chars().count();
    let filtered_entries = move || {
        let query = search.get().trim().to_lowercase();
        entries
            .get()
            .into_iter()
            .filter(|entry| {
                query.is_empty()
                    || entry.title.to_lowercase().contains(&query)
                    || entry.content.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    };
    let section_title = move || match active_section.get().as_str() {
        "overview" => "Vue générale",
        "journal" => "Notes à classer",
        "currencies" => selected_currency
            .get()
            .and_then(|code| currency_info(&code))
            .map(|currency| currency.code)
            .unwrap_or("Dossiers de devises"),
        "weekly" => "Préparation hebdomadaire",
        "events" => "Actualités & événements",
        "pairs" => "Analyse des paires",
        "reviews" => "Bilans",
        _ => "Journal de recherche",
    };

    view! {
        <div class="app-shell" class:sidebar_hidden=move || !sidebar_visible.get()>
            <aside class="sidebar">
                <a class="brand-lockup" href="#" on:click=move |_| {
                    set_active_section.set("journal".to_string());
                    set_editor_open.set(false);
                    set_selected_currency.set(None);
                    set_search.set(String::new());
                    set_entries.set(Vec::new());
                    load_entries("UNASSIGNED".to_string());
                }>
                    <div class="brand-mark">"M"</div>
                    <div class="brand-copy">
                        <strong>"MACRO JOURNAL"</strong>
                        <span>"CARNET DE RECHERCHE"</span>
                    </div>
                </a>

                <div class="sidebar-section-label">"ESPACE DE TRAVAIL"</div>
                <nav class="main-nav" aria-label="Navigation principale">
                    <button class="nav-item" class:active=move || active_section.get() == "overview" on:click=move |_| {
                        set_active_section.set("overview".to_string());
                        set_selected_currency.set(None);
                        set_editor_open.set(false);
                    }>
                        <span class="nav-icon">"⌂"</span><span>"Vue générale"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "journal" on:click=open_unassigned>
                        <span class="nav-icon">"▤"</span><span>"Notes à classer"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "currencies" on:click=move |_| {
                        set_active_section.set("currencies".to_string());
                        set_selected_currency.set(None);
                        set_editor_open.set(false);
                    }>
                        <span class="nav-icon">"◎"</span><span>"Devises"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "weekly" on:click=move |_| set_active_section.set("weekly".to_string())>
                        <span class="nav-icon">"▦"</span><span>"Préparation hebdomadaire"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "events" on:click=move |_| set_active_section.set("events".to_string())>
                        <span class="nav-icon">"◷"</span><span>"Actualités & événements"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "pairs" on:click=move |_| set_active_section.set("pairs".to_string())>
                        <span class="nav-icon">"⇄"</span><span>"Analyse des paires"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "reviews" on:click=move |_| set_active_section.set("reviews".to_string())>
                        <span class="nav-icon">"↺"</span><span>"Bilans"</span>
                    </button>
                </nav>

                <div class="sidebar-spacer"></div>
                <div class="sidebar-note">
                    <span class="sidebar-note-mark">"“"</span>
                    <p><span>"Les faits d’abord."</span><span>"La conviction ensuite."</span></p>
                    <span>"VOTRE MÉTHODE, VOTRE HISTORIQUE"</span>
                </div>
                <div class="sidebar-bottom">
                    <div class="connection-row">
                        <span class="connection-dot" class:online=move || api_status.get()></span>
                        <span>{move || if api_status.get() { "BASE LOCALE CONNECTÉE" } else { "BASE LOCALE HORS LIGNE" }}</span>
                    </div>
                    <div class="sidebar-version">"MACRO JOURNAL · 0.1.0"</div>
                </div>
            </aside>

            <main class="main-area">
                <header class="topbar">
                    <div class="topbar-start">
                        <button
                            type="button"
                            class="sidebar-toggle"
                            aria-label=move || if sidebar_visible.get() { "Masquer la barre latérale" } else { "Afficher la barre latérale" }
                            aria-expanded=move || sidebar_visible.get().to_string()
                            title=move || if sidebar_visible.get() { "Masquer la navigation" } else { "Afficher la navigation" }
                            on:click=move |_| set_sidebar_visible.update(|visible| *visible = !*visible)
                        >
                            <span aria-hidden="true">{move || if sidebar_visible.get() { "‹" } else { "☰" }}</span>
                        </button>
                        <div class="breadcrumb">
                            <span>"ESPACE DE TRAVAIL"</span><span class="breadcrumb-separator">"/"</span>
                            <strong>{section_title}</strong>
                        </div>
                    </div>
                    <div class="topbar-actions">
                        <span class="topbar-private"><span>"●"</span> "LOCAL & PRIVÉ"</span>
                        <a href="https://rateprobability.com/" target="_blank" rel="noopener noreferrer">"RATE PROBABILITY ↗"</a>
                        <a href="https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm" target="_blank" rel="noopener noreferrer">"CFTC ↗"</a>
                        <div class="avatar">"MJ"</div>
                    </div>
                </header>

                {move || {
                    if editor_open.get()
                        && (active_section.get() == "journal"
                            || selected_currency.get().is_some())
                    {
                        view! {
                            <section class="editor-page">
                                <div class="editor-page-top">
                                    <button class="back-link" on:click=move |_| set_editor_open.set(false)>
                                        <span aria-hidden="true">"←"</span>
                                        {move || selected_currency.get()
                                            .and_then(|code| currency_info(&code))
                                            .map(|currency| format!("Retour au dossier {}", currency.code))
                                            .unwrap_or_else(|| "Retour aux notes à classer".to_string())}
                                    </button>
                                    <div class="editor-actions">
                                        <span class="save-state">{save_status}</span>
                                        <Show when=move || !editing.get()>
                                            <button class="button button-quiet" on:click=move |_| set_editing.set(true)>
                                                <span>"✎"</span> "Modifier"
                                            </button>
                                        </Show>
                                        <Show when=move || editing.get()>
                                            <button
                                                class="button button-quiet"
                                                class:button-selected=move || preview_mode.get()
                                                on:click=move |_| set_preview_mode.update(|preview| *preview = !*preview)
                                            >
                                                {move || if preview_mode.get() { "Reprendre l’écriture" } else { "Aperçu" }}
                                            </button>
                                            <button class="button button-primary" on:click=save_note>
                                                <span>"✓"</span> "Enregistrer"
                                            </button>
                                        </Show>
                                    </div>
                                </div>

                                <article class="document-card">
                                    <div class="document-kicker">
                                        {move || selected_currency.get()
                                            .and_then(|code| currency_info(&code))
                                            .map(|currency| view! {
                                                <img class="document-flag" src=format!("/flags/{}.png", currency.flag) alt=format!("Drapeau : {}", currency.country)/>
                                            }.into_any())
                                            .unwrap_or_else(|| view! { <span class="document-kicker-mark">"M"</span> }.into_any())}
                                        <span>{move || selected_currency.get()
                                            .and_then(|code| currency_info(&code))
                                            .map(|currency| format!("DOSSIER {}", currency.code))
                                            .unwrap_or_else(|| "NOTES À CLASSER".to_string())}</span>
                                        <span class="kicker-divider">"·"</span>
                                        <span>{move || if active_id.get().is_some() { "NOTE" } else { "NOUVELLE NOTE" }}</span>
                                    </div>

                                    <Show
                                        when=move || editing.get()
                                        fallback=move || view! {
                                            <h1 class="document-title">{move || title.get()}</h1>
                                        }
                                    >
                                        <input
                                            class="document-title-input"
                                            type="text"
                                            aria-label="Titre de l’analyse"
                                            placeholder="Donnez un titre à votre observation"
                                            prop:value=move || title.get()
                                            on:input=move |event| {
                                                set_title.set(event_target_value(&event));
                                                set_save_status.set("MODIFICATIONS NON ENREGISTRÉES".to_string());
                                            }
                                        />
                                    </Show>

                                    <div class="document-byline">
                                        <span class="document-status-dot"></span>
                                        <span>{move || if active_id.get().is_some() { "RECHERCHE PERSONNELLE" } else { "BROUILLON" }}</span>
                                        <span class="kicker-divider">"·"</span>
                                        <span>{move || {
                                            entries.get().iter()
                                                .find(|entry| Some(entry.id) == active_id.get())
                                                .map(|entry| format_date(&entry.updated_at))
                                                .unwrap_or_else(|| "À l’instant".to_string())
                                        }}</span>
                                    </div>

                                    <div class="document-rule"></div>
                                    <Show
                                        when=move || editing.get() && !preview_mode.get()
                                        fallback=move || view! {
                                            <Show
                                                when=move || !content.get().trim().is_empty()
                                                fallback=|| view! {
                                                    <div class="document-empty">
                                                        <div class="empty-illustration">"✳"</div>
                                                        <h2>"Tout commence par une observation."</h2>
                                                        <p>"Ajoutez des faits, une hypothèse ou une question à explorer. Vous pourrez enrichir votre analyse au fil des publications."</p>
                                                    </div>
                                                }
                                            >
                                                <div class="document-body markdown-body" inner_html=preview_html />
                                            </Show>
                                        }
                                    >
                                        <div class="writing-area">
                                            <label class="writing-label" for="analysis-content">"VOTRE ANALYSE"</label>
                                            <textarea
                                                id="analysis-content"
                                                class="writing-input"
                                                aria-label="Contenu de l’analyse"
                                                placeholder="Quelle est l’observation ? Qu’est-ce que cela change dans votre lecture de la devise ? Notez aussi les éléments qui pourraient contredire votre hypothèse."
                                                prop:value=move || content.get()
                                                on:input=move |event| {
                                                    set_content.set(event_target_value(&event));
                                                    set_save_status.set("MODIFICATIONS NON ENREGISTRÉES".to_string());
                                                }
                                            ></textarea>
                                            <div class="writing-footer">
                                                <span>"Écrivez librement — les retours à la ligne sont conservés."</span>
                                                <span>{character_count} " caractères"</span>
                                            </div>
                                        </div>
                                    </Show>
                                </article>
                            </section>
                        }.into_any()
                    } else if active_section.get() == "currencies"
                        && selected_currency.get().is_some()
                    {
                        let currency = selected_currency
                            .get()
                            .and_then(|code| currency_info(&code))
                            .unwrap_or(UNASSIGNED_CURRENCY);
                        view! {
                            <section class="currency-page">
                                <div class="currency-back-row">
                                    <button class="back-link" on:click=move |_| {
                                        set_active_section.set("currencies".to_string());
                                        set_selected_currency.set(None);
                                        set_editor_open.set(false);
                                        set_search.set(String::new());
                                    }>
                                        <span aria-hidden="true">"←"</span> "Tous les dossiers"
                                    </button>
                                    <span class="scope-indicator"><span>"●"</span> "ESPACE INDÉPENDANT"</span>
                                </div>

                                <div class="currency-hero">
                                    <img class="currency-hero-flag" src=format!("/flags/{}.png", currency.flag) alt=format!("Drapeau : {}", currency.country)/>
                                    <div class="currency-identity">
                                        <div class="eyebrow">{if currency.external { "FACTEUR EXTERNE" } else { "DOSSIER DEVISE" }}</div>
                                        <h1>{currency.name} <span>{currency.code}</span></h1>
                                        <p>{currency.country} <span>"·"</span> {currency.bank}</p>
                                    </div>
                                    <button class="button button-primary currency-new-note" on:click=new_note>
                                        <span>"＋"</span> "Nouvelle note"
                                    </button>
                                </div>

                                <div class="currency-notice">
                                    <span class="notice-icon">"✳"</span>
                                    <span>"Ce dossier contient uniquement les notes liées à " {currency.code} {". Les analyses restent séparées entre devises."}</span>
                                </div>

                                <section class="currency-notes">
                                    <div class="currency-notes-heading">
                                        <div>
                                            <div class="eyebrow">"HISTORIQUE DU DOSSIER"</div>
                                            <h2>"Notes de recherche"</h2>
                                        </div>
                                        <label class="search-box">
                                            <span class="search-icon" aria-hidden="true">"⌕"</span>
                                            <input
                                                type="search"
                                                aria-label=format!("Rechercher dans les notes {}", currency.code)
                                                placeholder="Rechercher dans ce dossier…"
                                                prop:value=move || search.get()
                                                on:input=move |event| set_search.set(event_target_value(&event))
                                            />
                                        </label>
                                    </div>
                                    <Show
                                        when=move || !filtered_entries().is_empty()
                                        fallback=move || view! {
                                            <div class="empty-journal currency-empty">
                                                <div class="empty-journal-icon">{move || if entries.get().is_empty() { "✳" } else { "⌕" }}</div>
                                                <h3>{move || if entries.get().is_empty() { format!("Le dossier {} est prêt.", currency.code) } else { "Aucune note ne correspond à cette recherche.".to_string() }}</h3>
                                                <p>{move || if entries.get().is_empty() { "Consignez ici les données, décisions de banque centrale et hypothèses propres à cette devise.".to_string() } else { "Essayez un autre mot-clé ou effacez votre recherche.".to_string() }}</p>
                                                <Show when=move || entries.get().is_empty()>
                                                    <button class="text-action" on:click=new_note>"Créer la première note" <span>"→"</span></button>
                                                </Show>
                                            </div>
                                        }
                                    >
                                        <div class="notes-grid">
                                            <For
                                                each=move || filtered_entries()
                                                key=|entry| entry.id
                                                children=move |entry| {
                                                    let note_title = entry.title.clone();
                                                    let note_content = entry.content.clone();
                                                    let updated = entry.updated_at.clone();
                                                    let snippet = preview_text(&entry.content);
                                                    view! {
                                                        <button
                                                            class="research-card"
                                                            on:click=move |_| {
                                                                set_active_id.set(Some(entry.id));
                                                                set_title.set(note_title.clone());
                                                                set_content.set(note_content.clone());
                                                                set_editor_open.set(true);
                                                                set_editing.set(false);
                                                                set_preview_mode.set(false);
                                                                set_save_status.set(format!("OUVERT · {}", format_date(&updated)));
                                                            }
                                                        >
                                                            <div class="research-card-top">
                                                                <span class="research-card-type"><span>"✳"</span> {currency.code} " · ANALYSE"</span>
                                                                <span class="card-arrow" aria-hidden="true">"↗"</span>
                                                            </div>
                                                            <h3>{entry.title.clone()}</h3>
                                                            <p class="research-card-excerpt">{if snippet.is_empty() { "Aucun contenu pour le moment.".to_string() } else { snippet }}</p>
                                                            <div class="research-card-footer">
                                                                <span class="card-date"><span>"◷"</span> {format_date(&entry.updated_at)}</span>
                                                                <span class="card-read">"Ouvrir la note"</span>
                                                            </div>
                                                        </button>
                                                    }
                                                }
                                            />
                                        </div>
                                    </Show>
                                </section>
                            </section>
                        }.into_any()
                    } else if active_section.get() == "currencies"
                        || active_section.get() == "overview"
                    {
                        view! {
                            <section class="currency-index-page">
                                <div class="currency-index-intro">
                                    <div class="eyebrow"><span class="eyebrow-dot"></span> "RECHERCHE FONDAMENTALE · VOS DOSSIERS"</div>
                                    <h1>"Une devise, un espace."</h1>
                                    <p>"Chaque économie a son propre dossier. Vos observations restent cloisonnées ; les comparaisons entre devises viendront plus tard."</p>
                                </div>

                                <div class="currency-group-heading">
                                    <div>
                                        <div class="eyebrow">"LES 8 DEVISES MAJEURES"</div>
                                        <h2>"Choisissez votre dossier"</h2>
                                    </div>
                                    <span class="folder-count">"08 DOSSIERS"</span>
                                </div>

                                <div class="currency-grid">
                                    <For
                                        each=move || {
                                            CURRENCIES.iter().filter(|currency| !currency.external).copied().collect::<Vec<_>>()
                                        }
                                        key=|currency| currency.code
                                        children=move |currency| {
                                            view! {
                                                <button class="currency-card" on:click=move |_| open_currency(currency.code)>
                                                    <div class="currency-card-top">
                                                        <img class="currency-card-flag" src=format!("/flags/{}.png", currency.flag) alt=format!("Drapeau : {}", currency.country)/>
                                                        <span class="currency-card-arrow" aria-hidden="true">"↗"</span>
                                                    </div>
                                                    <div class="currency-card-code">{currency.code}</div>
                                                    <h3>{currency.name}</h3>
                                                    <p>{currency.country}</p>
                                                    <div class="currency-card-bottom">
                                                        <span>{currency.bank}</span>
                                                        <span>"OUVRIR"</span>
                                                    </div>
                                                </button>
                                            }
                                        }
                                    />
                                </div>

                                <div class="external-heading">
                                    <div>
                                        <div class="eyebrow">"FACTEUR EXTERNE"</div>
                                        <h2>"À surveiller"</h2>
                                    </div>
                                    <p>"La Chine, notamment pour l’AUD et le NZD."</p>
                                </div>
                                <For
                                    each=move || {
                                        CURRENCIES.iter().filter(|currency| currency.external).copied().collect::<Vec<_>>()
                                    }
                                    key=|currency| currency.code
                                    children=move |currency| {
                                        view! {
                                            <button class="currency-card external-currency-card" on:click=move |_| open_currency(currency.code)>
                                                <div class="currency-card-top">
                                                    <img class="currency-card-flag" src=format!("/flags/{}.png", currency.flag) alt=format!("Drapeau : {}", currency.country)/>
                                                    <span class="external-badge">"CONTEXTE EXTERNE"</span>
                                                    <span class="currency-card-arrow" aria-hidden="true">"↗"</span>
                                                </div>
                                                <div class="currency-card-code">{currency.code}</div>
                                                <h3>{currency.name}</h3>
                                                <p>{currency.country}</p>
                                                <div class="currency-card-bottom">
                                                    <span>{currency.bank}</span>
                                                    <span>"OUVRIR"</span>
                                                </div>
                                            </button>
                                        }
                                    }
                                />

                                <button class="unassigned-link" on:click=open_unassigned>
                                    <span class="unassigned-mark">"↗"</span>
                                    <span><strong>"Notes à classer"</strong><small>"Retrouver les anciennes notes qui n’étaient pas associées à une devise."</small></span>
                                    <span class="currency-card-arrow">"→"</span>
                                </button>
                            </section>
                        }.into_any()
                    } else if active_section.get() == "journal" {
                        view! {
                            <section class="dashboard-page">
                                <div class="welcome-banner">
                                    <div class="welcome-copy">
                                        <div class="eyebrow"><span class="eyebrow-dot"></span> "ARCHIVE HISTORIQUE · SANS DEVISE ASSOCIÉE"</div>
                                        <h1>"Notes à classer."<br/><span>"Votre historique est conservé."</span></h1>
                                        <p>"Ces notes précèdent le classement par devise. Elles restent ici, intactes ; toute nouvelle analyse commence dans son dossier dédié."</p>
                                    </div>
                                    <div class="welcome-art" aria-hidden="true">
                                        <div class="art-orbit orbit-one"></div>
                                        <div class="art-orbit orbit-two"></div>
                                        <div class="art-core"><span>"MJ"</span></div>
                                        <div class="art-spark spark-one"></div>
                                        <div class="art-spark spark-two"></div>
                                        <div class="art-caption">"FONDAMENTAUX · POSITIONNEMENT · PRIX"</div>
                                    </div>
                                </div>

                                <div class="stats-grid">
                                    <article class="stat-card">
                                        <div class="stat-label"><span class="stat-icon">"▤"</span> "DANS CETTE ARCHIVE"</div>
                                        <div class="stat-value">{move || entries.get().len()}</div>
                                        <div class="stat-detail">"notes sans devise associée"</div>
                                    </article>
                                    <article class="stat-card">
                                        <div class="stat-label"><span class="stat-icon stat-icon-sand">"◷"</span> "DERNIÈRE ACTIVITÉ"</div>
                                        <div class="stat-value stat-date">{move || {
                                            entries.get().first()
                                                .map(|entry| format_date(&entry.updated_at))
                                                .unwrap_or_else(|| "—".to_string())
                                        }}</div>
                                        <div class="stat-detail">{move || if entries.get().is_empty() { "Votre première note vous attend" } else { "Mise à jour la plus récente" }}</div>
                                    </article>
                                    <article class="stat-card stat-card-local">
                                        <div class="stat-label"><span class="stat-icon stat-icon-green">"⌂"</span> "VOTRE ESPACE"</div>
                                        <div class="stat-value stat-local">{move || if api_status.get() { "Local" } else { "Hors ligne" }}</div>
                                        <div class="stat-detail">{move || if api_status.get() { "Données conservées sur cet appareil" } else { "Connexion à la base locale indisponible" }}</div>
                                    </article>
                                </div>

                                <div class="dashboard-columns">
                                    <section class="research-section">
                                        <div class="section-heading">
                                            <div>
                                                <div class="eyebrow">"VOTRE HISTORIQUE"</div>
                                                <h2>"Anciennes notes"</h2>
                                                <p>"Aucune note n’est réaffectée automatiquement à une devise."</p>
                                            </div>
                                            <label class="search-box">
                                                <span class="search-icon" aria-hidden="true">"⌕"</span>
                                                <input
                                                    type="search"
                                                    aria-label="Rechercher dans les notes"
                                                    placeholder="Rechercher une note…"
                                                    prop:value=move || search.get()
                                                    on:input=move |event| set_search.set(event_target_value(&event))
                                                />
                                            </label>
                                        </div>

                                        <Show
                                            when=move || !filtered_entries().is_empty()
                                            fallback=move || view! {
                                                <div class="empty-journal">
                                                    <div class="empty-journal-icon">{move || if entries.get().is_empty() { "✳" } else { "⌕" }}</div>
                                                    <h3>{move || if entries.get().is_empty() { "Aucune ancienne note à classer." } else { "Aucune note ne correspond à cette recherche." }}</h3>
                                                    <p>{move || if entries.get().is_empty() { "Les notes créées à partir de maintenant seront enregistrées dans le dossier de la devise concernée.".to_string() } else { "Essayez un autre mot-clé ou effacez votre recherche.".to_string() }}</p>
                                                </div>
                                            }
                                        >
                                            <div class="notes-grid">
                                                <For
                                                    each=move || filtered_entries()
                                                    key=|entry| entry.id
                                                    children=move |entry| {
                                                        let note_title = entry.title.clone();
                                                        let note_content = entry.content.clone();
                                                        let updated = entry.updated_at.clone();
                                                        let snippet = preview_text(&entry.content);
                                                        view! {
                                                            <button
                                                                class="research-card"
                                                                on:click=move |_| {
                                                                    set_active_id.set(Some(entry.id));
                                                                    set_title.set(note_title.clone());
                                                                    set_content.set(note_content.clone());
                                                                    set_editor_open.set(true);
                                                                    set_editing.set(false);
                                                                    set_preview_mode.set(false);
                                                                    set_save_status.set(format!("OUVERT · {}", format_date(&updated)));
                                                                }
                                                            >
                                                                <div class="research-card-top">
                                                                    <span class="research-card-type"><span>"✳"</span> "ANALYSE"</span>
                                                                    <span class="card-arrow" aria-hidden="true">"↗"</span>
                                                                </div>
                                                                <h3>{entry.title.clone()}</h3>
                                                                <p class="research-card-excerpt">{if snippet.is_empty() { "Aucun contenu pour le moment.".to_string() } else { snippet }}</p>
                                                                <div class="research-card-footer">
                                                                    <span class="card-date"><span>"◷"</span> {format_date(&entry.updated_at)}</span>
                                                                    <span class="card-read">"Ouvrir la note"</span>
                                                                </div>
                                                            </button>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </Show>
                                    </section>

                                    <aside class="guidance-column">
                                        <section class="method-card">
                                            <div class="method-heading">
                                                <span class="method-icon">"✳"</span>
                                                <span class="eyebrow">"VOTRE CADRE D’ANALYSE"</span>
                                            </div>
                                            <h2>"Une conviction se construit."</h2>
                                            <p>"Pas de signal magique. Une lecture solide relie plusieurs angles et garde la place au doute."</p>
                                            <div class="method-steps">
                                                <div class="method-step">
                                                    <span class="step-number">"01"</span>
                                                    <div><strong>"Les fondamentaux"</strong><span>"Taux, inflation, emploi, croissance"</span></div>
                                                </div>
                                                <div class="method-step">
                                                    <span class="step-number">"02"</span>
                                                    <div><strong>"Le contexte de marché"</strong><span>"Positionnement, corrélations, sentiment"</span></div>
                                                </div>
                                                <div class="method-step">
                                                    <span class="step-number">"03"</span>
                                                    <div><strong>"Votre raisonnement"</strong><span>"Faits, hypothèse, contre-arguments"</span></div>
                                                </div>
                                            </div>
                                        </section>

                                        <section class="resources-card">
                                            <div class="eyebrow">"À PORTÉE DE MAIN"</div>
                                            <a href="https://rateprobability.com/" target="_blank" rel="noopener noreferrer" class="resource-link">
                                                <span class="resource-mark">"↗"</span>
                                                <span><strong>"Anticipations de taux"</strong><small>"Rate Probability"</small></span>
                                                <span class="resource-arrow">"↗"</span>
                                            </a>
                                            <a href="https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm" target="_blank" rel="noopener noreferrer" class="resource-link">
                                                <span class="resource-mark resource-mark-sand">"↗"</span>
                                                <span><strong>"Positionnement des acteurs"</strong><small>"Rapports CFTC · COT"</small></span>
                                                <span class="resource-arrow">"↗"</span>
                                            </a>
                                        </section>
                                    </aside>
                                </div>
                            </section>
                        }.into_any()
                    } else {
                        view! {
                            <section class="placeholder-page">
                                <div class="eyebrow">"MACRO JOURNAL · ESPACE DE TRAVAIL"</div>
                                <div class="placeholder-mark">"MJ"</div>
                                <h1>{section_title}</h1>
                                <p>"Cet espace fait partie de la structure prévue. Il sera développé lorsque le socle du journal sera stabilisé."</p>
                                <button class="button button-primary" on:click=move |_| set_active_section.set("journal".to_string())>
                                    "Retour au journal de recherche"
                                </button>
                            </section>
                        }.into_any()
                    }
                }}

                <footer class="app-footer">
                    <span><span class="footer-dot" class:online=move || api_status.get()></span> {move || if api_status.get() { "Base locale connectée" } else { "En attente de la base locale" }}</span>
                    <span>"VOS NOTES RESTENT SUR CET APPAREIL"</span>
                </footer>
            </main>
        </div>
    }
}

fn format_date(timestamp: &str) -> String {
    let date = timestamp.split('T').next().unwrap_or(timestamp);
    let parts = date.split('-').collect::<Vec<_>>();
    if parts.len() == 3 {
        format!("{}/{}/{}", parts[2], parts[1], parts[0])
    } else {
        date.to_string()
    }
}

fn preview_text(source: &str) -> String {
    let text = Parser::new(source)
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.to_string()),
            Event::SoftBreak | Event::HardBreak => Some(" ".to_string()),
            _ => None,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let preview = text.chars().take(150).collect::<String>();
    if text.chars().count() > 150 {
        format!("{preview}…")
    } else {
        preview
    }
}

async fn check_api() -> Result<(), String> {
    let response = Request::get(&format!("{API_BASE}/api/health"))
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(format!("API returned HTTP {}", response.status()))
    }
}

async fn fetch_entries(currency_code: &str) -> Result<Vec<Entry>, String> {
    let response = Request::get(&format!(
        "{API_BASE}/api/currencies/{currency_code}/entries"
    ))
    .send()
    .await
    .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("API returned HTTP {}", response.status()));
    }

    response
        .json::<Vec<Entry>>()
        .await
        .map_err(|error| error.to_string())
}

async fn create_entry(currency_code: &str, draft: EntryDraft) -> Result<Entry, String> {
    let response = Request::post(&format!(
        "{API_BASE}/api/currencies/{currency_code}/entries"
    ))
    .json(&draft)
    .map_err(|error| error.to_string())?
    .send()
    .await
    .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("API returned HTTP {}", response.status()));
    }

    response
        .json::<Entry>()
        .await
        .map_err(|error| error.to_string())
}

async fn update_entry(id: i64, currency_code: &str, draft: EntryDraft) -> Result<Entry, String> {
    let response = Request::put(&format!(
        "{API_BASE}/api/currencies/{currency_code}/entries/{id}"
    ))
    .json(&draft)
    .map_err(|error| error.to_string())?
    .send()
    .await
    .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("API returned HTTP {}", response.status()));
    }

    response
        .json::<Entry>()
        .await
        .map_err(|error| error.to_string())
}

fn render_markdown(source: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(source, options);
    let mut unsafe_html = String::new();
    html::push_html(&mut unsafe_html, parser);

    ammonia::clean(&unsafe_html)
}
