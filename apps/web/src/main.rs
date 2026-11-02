use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use macro_journal_shared::{Entry, EntryDraft};
use pulldown_cmark::{html, Options, Parser};

const API_BASE: &str = "http://127.0.0.1:3000";

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (active_section, set_active_section) = signal("journal".to_string());
    let (entries, set_entries) = signal::<Vec<Entry>>(Vec::new());
    let (active_id, set_active_id) = signal::<Option<i64>>(None);
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (save_status, set_save_status) = signal("BASE LOCALE · CONNEXION".to_string());
    let (api_status, set_api_status) = signal(false);
    let (sidebar_visible, set_sidebar_visible) = signal(true);

    let load_entries = move || {
        spawn_local(async move {
            match fetch_entries().await {
                Ok(items) => {
                    set_entries.set(items);
                    set_save_status.set("BASE LOCALE · CONNECTÉE".to_string());
                    set_api_status.set(true);
                }
                Err(error) => {
                    set_save_status.set(format!("API INDISPONIBLE · {error}"));
                    set_api_status.set(false);
                }
            }
        });
    };

    load_entries();

    let new_note = move |_| {
        set_active_id.set(None);
        set_title.set(String::new());
        set_content.set(String::new());
        set_save_status.set("NOUVEAU BROUILLON · NON ENREGISTRÉ".to_string());
    };

    let save_note = move |_| {
        let draft = EntryDraft {
            title: title.get().trim().to_string(),
            content: content.get(),
        };
        let existing_id = active_id.get();

        if draft.title.is_empty() {
            set_save_status.set("UN TITRE EST REQUIS".to_string());
            return;
        }

        set_save_status.set("ENREGISTREMENT…".to_string());

        spawn_local(async move {
            let result = match existing_id {
                Some(id) => update_entry(id, draft).await,
                None => create_entry(draft).await,
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
                    set_save_status.set(format!("ENREGISTRÉ LOCALEMENT · {}", saved.updated_at));
                    set_api_status.set(true);
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
    let section_title = move || match active_section.get().as_str() {
        "overview" => "Vue générale",
        "journal" => "Journal de recherche",
        "currencies" => "Profils des devises",
        "weekly" => "Préparation hebdomadaire",
        "events" => "Actualités & événements",
        "pairs" => "Analyse des paires",
        "reviews" => "Bilans",
        _ => "Journal de recherche",
    };

    view! {
        <div class="app-shell" class:sidebar_hidden=move || !sidebar_visible.get()>
            <aside class="sidebar">
                <div class="brand-lockup">
                    <div class="brand-mark">"M"</div>
                    <div class="brand-copy">
                        <strong>"MACRO JOURNAL"</strong>
                        <span>"CARNET DE RECHERCHE"</span>
                    </div>
                </div>

                <div class="sidebar-section-label">"ESPACE DE TRAVAIL"</div>
                <nav class="main-nav" aria-label="Main navigation">
                    <button class="nav-item" class:active=move || active_section.get() == "overview" on:click=move |_| set_active_section.set("overview".to_string())>
                        <span class="nav-icon">"⌂"</span><span>"Vue générale"</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "journal" on:click=move |_| set_active_section.set("journal".to_string())>
                        <span class="nav-icon">"▤"</span><span>"Journal de recherche"</span>
                        <span class="nav-count">{move || entries.get().len()}</span>
                    </button>
                    <button class="nav-item" class:active=move || active_section.get() == "currencies" on:click=move |_| set_active_section.set("currencies".to_string())>
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
                <div class="sidebar-bottom">
                    <div class="connection-row">
                        <span class="connection-dot" class:online=move || api_status.get()></span>
                        <span>{move || if api_status.get() { "API LOCALE CONNECTÉE" } else { "API LOCALE HORS LIGNE" }}</span>
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
                        <a href="https://rateprobability.com/" target="_blank" rel="noopener noreferrer">"RATE PROBABILITY ↗"</a>
                        <a href="https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm" target="_blank" rel="noopener noreferrer">"CFTC ↗"</a>
                        <div class="avatar">"MJ"</div>
                    </div>
                </header>

                {move || {
                    if active_section.get() == "journal" {
                        view! {
                            <section class="journal-page">
                                <div class="page-heading">
                                    <div>
                                        <div class="eyebrow">"RECHERCHE PERSONNELLE · LOCAL FIRST"</div>
                                        <h1>"Journal de recherche"</h1>
                                        <p>"Record the facts. Challenge the narrative. Keep the reasoning."</p>
                                    </div>
                                    <button class="button button-primary" on:click=new_note>
                                        <span>"＋"</span> "Nouvelle note"
                                    </button>
                                </div>

                                <div class="journal-layout">
                                    <aside class="notes-rail">
                                        <div class="notes-rail-heading">
                                            <div class="eyebrow">"MES NOTES"</div>
                                            <span class="note-total">{move || entries.get().len()}</span>
                                        </div>
                                        <div class="notes-list">
                                            <Show when=move || entries.get().is_empty()>
                                                <div class="empty-notes">
                                                    <span class="empty-notes-mark">"＋"</span>
                                                    <strong>"Aucune note pour le moment"</strong>
                                                    <span>"Crée ta première note pour commencer à construire ton historique de recherche."</span>
                                                </div>
                                            </Show>
                                            <For
                                                each=move || entries.get()
                                                key=|entry| entry.id
                                                children=move |entry| {
                                                    let id = entry.id;
                                                    let note_title = entry.title.clone();
                                                    let note_content = entry.content.clone();
                                                    let updated = entry.updated_at.clone();
                                                    view! {
                                                        <button
                                                            class="note-list-item"
                                                            class:active=move || active_id.get() == Some(id)
                                                            on:click=move |_| {
                                                                set_active_id.set(Some(id));
                                                                set_title.set(note_title.clone());
                                                                set_content.set(note_content.clone());
                                                                set_save_status.set(format!("OUVERT · {updated}"));
                                                            }
                                                        >
                                                            <strong>{entry.title.clone()}</strong>
                                                            <span>{entry.updated_at.chars().take(10).collect::<String>()}</span>
                                                        </button>
                                                    }
                                                }
                                            />
                                        </div>
                                        <div class="notes-rail-footer">"NOTES STOCKÉES SUR CET APPAREIL"</div>
                                    </aside>

                                    <section class="editor-area">
                                        <div class="editor-toolbar">
                                            <div class="document-kind"><span class="document-icon">"▤"</span> "ANALYSE FONDAMENTALE"</div>
                                            <div class="editor-actions">
                                                <span class="save-state">{save_status}</span>
                                                <button class="button button-quiet" on:click=move |_| load_entries()>"Actualiser"</button>
                                                <button class="button button-primary" on:click=save_note>"Enregistrer"</button>
                                            </div>
                                        </div>

                                        <div class="title-field-wrap">
                                            <input
                                                class="title-field"
                                                type="text"
                                                placeholder="Nouvelle note de recherche"
                                                prop:value=move || title.get()
                                                on:input=move |event| {
                                                    set_title.set(event_target_value(&event));
                                                    set_save_status.set("MODIFICATIONS NON ENREGISTRÉES".to_string());
                                                }
                                            />
                                            <div class="document-meta">
                                                <span>{move || if active_id.get().is_some() { "NOTE ENREGISTRÉE" } else { "NOUVELLE NOTE" }}</span>
                                                <span class="meta-separator">"·"</span>
                                                <span>"MARKDOWN"</span>
                                            </div>
                                        </div>

                                        <div class="split-editor">
                                            <section class="editor-pane">
                                                <div class="pane-heading">
                                                    <span>"ÉDITEUR"</span><span>"SOURCE MARKDOWN"</span>
                                                </div>
                                                <textarea
                                                    class="markdown-input"
                                                    spellcheck="false"
                                                    placeholder="# Research question\n\n## Observations\nRecord what happened. Separate facts from interpretation.\n\n## Fundamental view\nWhat does this change in the economic narrative?\n\n## Market pricing\nWhat is already reflected in prices?\n\n## Counterarguments\nWhat could invalidate this view?\n\n## Sources\n- https://..."
                                                    prop:value=move || content.get()
                                                    on:input=move |event| {
                                                        set_content.set(event_target_value(&event));
                                                        set_save_status.set("MODIFICATIONS NON ENREGISTRÉES".to_string());
                                                    }
                                                ></textarea>
                                                <div class="pane-footer">
                                                    <span>"UTF-8 · MARKDOWN"</span>
                                                    <span>{character_count} " characters"</span>
                                                </div>
                                            </section>

                                            <section class="preview-pane">
                                                <div class="pane-heading">
                                                    <span>"APERÇU"</span><span>"DOCUMENT RENDU"</span>
                                                </div>
                                                <div class="markdown-preview">
                                                    <Show when=move || !content.get().trim().is_empty() fallback=|| view! {
                                                        <div class="preview-empty">
                                                            <div class="preview-empty-mark">"M"</div>
                                                            <strong>"Ton analyse commence ici."</strong>
                                                            <p>"Écris en Markdown à gauche. Le document formaté apparaîtra dans ce panneau."</p>
                                                        </div>
                                                    }>
                                                        <div class="markdown-body" inner_html=preview_html />
                                                    </Show>
                                                </div>
                                                <div class="pane-footer"><span>"LIVE APERÇU"</span><span>"HTML ASSAINI"</span></div>
                                            </section>
                                        </div>
                                    </section>
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
                    <span>{save_status}</span>
                    <span>"MACRO JOURNAL · LOCAL RESEARCH ENVIRONMENT"</span>
                </footer>
            </main>
        </div>
    }
}

async fn fetch_entries() -> Result<Vec<Entry>, String> {
    let response = Request::get(&format!("{API_BASE}/api/entries"))
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

async fn create_entry(draft: EntryDraft) -> Result<Entry, String> {
    let response = Request::post(&format!("{API_BASE}/api/entries"))
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

async fn update_entry(id: i64, draft: EntryDraft) -> Result<Entry, String> {
    let response = Request::put(&format!("{API_BASE}/api/entries/{id}"))
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

    // Markdown can contain raw HTML. Sanitize the generated HTML before using
    // Leptos' inner_html attribute to prevent script injection in the preview.
    ammonia::clean(&unsafe_html)
}
