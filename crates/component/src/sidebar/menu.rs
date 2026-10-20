use crate::{
    ActiveTheme as _, Collapsible, Icon, IconName, Placement, Sizable as _, StyledExt, ThemeStyled as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    menu::{ContextMenuExt, PopupMenu},
    sidebar::SidebarItem,
    tooltip::{ManagedTooltipExt as _, Tooltip},
    v_flex,
};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement,
    ParentElement as _, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window, div, percentage, prelude::FluentBuilder,
};
use gpui_base::TestSupportExt as _;
use std::rc::Rc;

/// Menu for the [`super::Sidebar`]
#[derive(Clone)]
pub struct SidebarMenu {
    style: StyleRefinement,
    collapsed: bool,
    items: Vec<SidebarMenuItem>,
}

impl SidebarMenu {
    /// Create a new SidebarMenu
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            items: Vec::new(),
            collapsed: false,
        }
    }

    /// Add a [`SidebarMenuItem`] child menu item to the sidebar menu.
    ///
    /// See also [`SidebarMenu::children`].
    pub fn child(mut self, child: impl Into<SidebarMenuItem>) -> Self {
        self.items.push(child.into());
        self
    }

    /// Add multiple [`SidebarMenuItem`] child menu items to the sidebar menu.
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SidebarMenuItem>>,
    ) -> Self {
        self.items = children.into_iter().map(Into::into).collect();
        self
    }
}

impl Collapsible for SidebarMenu {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl SidebarItem for SidebarMenu {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let id = id.into();

        v_flex()
            .gap_2()
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let id = SharedString::from(format!("{}-{}", id, ix));
                item.collapsed(self.collapsed)
                    .render(id, window, cx)
                    .into_any_element()
            }))
    }
}

impl Styled for SidebarMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// Menu item for the [`SidebarMenu`]
#[derive(Clone)]
pub struct SidebarMenuItem {
    icon: Option<Icon>,
    label: SharedString,
    label_style: StyleRefinement,
    style: StyleRefinement,
    handler: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    active: bool,
    default_open: bool,
    click_to_open: bool,
    collapsed: bool,
    click_to_toggle: bool,
    children: Vec<Self>,
    suffix: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    disabled: bool,
    context_menu: Option<Rc<dyn Fn(PopupMenu, &mut Window, &mut App) -> PopupMenu + 'static>>,
}

impl SidebarMenuItem {
    /// Create a new [`SidebarMenuItem`] with a label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            icon: None,
            label: label.into(),
            label_style: StyleRefinement::default(),
            style: StyleRefinement::default(),
            handler: Rc::new(|_, _, _| {}),
            active: false,
            collapsed: false,
            default_open: false,
            click_to_open: false,
            click_to_toggle: false,
            children: Vec::new(),
            suffix: None,
            disabled: false,
            context_menu: None,
        }
    }

    /// Set the icon for the menu item
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the style for the label
    pub fn label_style(mut self, style: StyleRefinement) -> Self {
        self.label_style = style;
        self
    }

    /// Set the active state of the menu item
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Add a click handler to the menu item
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.handler = Rc::new(handler);
        self
    }

    /// Set the collapsed state of the menu item
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Set the default open state of the Submenu, default is `false`.
    ///
    /// This only used on initial render, the internal state will be used afterwards.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set whether clicking the menu item open the submenu.
    ///
    /// Default is `false`.
    ///
    /// If `false` we only handle open/close via the caret button.
    pub fn click_to_open(mut self, click_to_open: bool) -> Self {
        self.click_to_open = click_to_open;
        self
    }

    /// Set whether clicking the menu item toggles the submenu.
    ///
    /// If click_to_open is `true`, this has no effect.
    ///
    /// Default is `false`.
    pub fn click_to_toggle(mut self, click_to_toggle: bool) -> Self {
        self.click_to_toggle = click_to_toggle;
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Self>>) -> Self {
        self.children = children.into_iter().map(Into::into).collect();
        self
    }

    /// Set the suffix for the menu item.
    pub fn suffix<F, E>(mut self, builder: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> E + 'static,
        E: IntoElement,
    {
        self.suffix = Some(Rc::new(move |window, cx| {
            builder(window, cx).into_any_element()
        }));
        self
    }

    /// Set disabled flat for menu item.
    pub fn disable(mut self, disable: bool) -> Self {
        self.disabled = disable;
        self
    }

    fn is_submenu(&self) -> bool {
        self.children.len() > 0
    }

    fn collapsed_tooltip(&self) -> Option<SharedString> {
        (self.collapsed && self.icon.is_some()).then(|| self.label.clone())
    }

    /// Set the context menu for the menu item.
    pub fn context_menu(
        mut self,
        f: impl Fn(PopupMenu, &mut Window, &mut App) -> PopupMenu + 'static,
    ) -> Self {
        self.context_menu = Some(Rc::new(f));
        self
    }
}

impl FluentBuilder for SidebarMenuItem {}

impl Collapsible for SidebarMenuItem {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl SidebarItem for SidebarMenuItem {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let click_to_open = self.click_to_open;
        let click_to_toggle = self.click_to_toggle;
        let default_open = self.default_open;
        let collapsed_tooltip = self.collapsed_tooltip();
        let id = id.into();
        let is_submenu = self.is_submenu();
        let open_state = if is_submenu {
            Some(window.use_keyed_state(id.clone(), cx, |_, _| default_open))
        } else {
            None
        };
        let focus_handle = window
            .use_keyed_state((id.clone(), "focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let handler = self.handler.clone();
        let is_collapsed = self.collapsed;
        let is_active = self.active;
        let is_hoverable = !is_active && !self.disabled;
        let is_disabled = self.disabled;
        let is_focused = !is_disabled && focus_handle.is_focused(window);
        let is_open = open_state
            .as_ref()
            .map_or(false, |s| !is_collapsed && *s.read(cx));

        div()
            .id(id.clone())
            .test_support()
            .w_full()
            .child(
                h_flex()
                    .size_full()
                    .id("item")
                    .test_support()
                    .role(Role::Button)
                    .aria_label(self.label.clone())
                    .aria_selected(is_active)
                    .when(is_submenu, |this| this.aria_expanded(is_open))
                    .when(!is_disabled, |this| {
                        this.track_focus(&focus_handle.tab_stop(true).tab_index(0))
                    })
                    .overflow_x_hidden()
                    .flex_shrink_0()
                    .p_2()
                    .gap_x_2()
                    .rounded(cx.theme().radius)
                    .when(is_focused, |this| this.focus_ring_style(window, cx))
                    .text_sm()
                    .refine_style(&self.style)
                    .when(is_hoverable, |this| {
                        this.hover(|this| {
                            this.bg(cx.theme().sidebar_accent.opacity(0.8))
                                .text_color(cx.theme().sidebar_accent_foreground)
                        })
                    })
                    .when(is_active, |this| {
                        this.font_medium()
                            .bg(cx.theme().tokens.sidebar_accent)
                            .text_color(cx.theme().sidebar_accent_foreground)
                    })
                    .when_some(self.icon.clone(), |this, icon| this.child(icon))
                    .when(is_collapsed, |this| {
                        this.justify_center().when(is_active, |this| {
                            this.bg(cx.theme().tokens.sidebar_accent)
                                .text_color(cx.theme().sidebar_accent_foreground)
                        })
                    })
                    .when(!is_collapsed, |this| {
                        this.h_7()
                            .child(
                                h_flex()
                                    .flex_1()
                                    .gap_x_2()
                                    .justify_between()
                                    .overflow_x_hidden()
                                    .child(
                                        h_flex()
                                            .flex_1()
                                            .overflow_x_hidden()
                                            .refine_style(&self.label_style)
                                            .child(self.label.clone()),
                                    )
                                    .when_some(self.suffix.clone(), |this, suffix| {
                                        this.child(suffix(window, cx).into_any_element())
                                    }),
                            )
                            .when_some(open_state.clone(), |this, open_state| {
                                this.child(
                                    Button::new("caret")
                                        .with_base(
                                            gpui_base::Button::new("caret").aria_expanded(is_open),
                                        )
                                        .accessibility_label(format!(
                                            "{} {} groups",
                                            if is_open { "Collapse" } else { "Expand" },
                                            self.label,
                                        ))
                                        .xsmall()
                                        .ghost()
                                        .icon(
                                            Icon::new(IconName::ChevronRight)
                                                .size_4()
                                                .when(is_open, |this| {
                                                    this.rotate(percentage(90. / 360.))
                                                }),
                                        )
                                        .on_click({
                                            move |_, _, cx| {
                                                // Avoid trigger item click, just expand/collapse submenu
                                                cx.stop_propagation();
                                                open_state.update(cx, |is_open, cx| {
                                                    *is_open = !*is_open;
                                                    cx.notify();
                                                })
                                            }
                                        }),
                                )
                            })
                    })
                    .when(is_disabled, |this| {
                        this.text_color(cx.theme().muted_foreground)
                    })
                    .when(!is_disabled, |this| {
                        this.on_click({
                            let open_state = open_state.clone();
                            move |ev, window, cx| {
                                if click_to_open {
                                    if let Some(ref s) = open_state {
                                        s.update(cx, |is_open: &mut bool, cx| {
                                            *is_open = true;
                                            cx.notify();
                                        });
                                    }
                                } else if click_to_toggle {
                                    if let Some(ref s) = open_state {
                                        s.update(cx, |is_open: &mut bool, cx| {
                                            *is_open = !*is_open;
                                            cx.notify();
                                        });
                                    }
                                }
                                handler(ev, window, cx)
                            }
                        })
                    })
                    .map(|this| {
                        if let Some(tooltip) = collapsed_tooltip {
                            this.managed_tooltip_at(Placement::Right, move |window, cx| {
                                Tooltip::new(tooltip.clone()).build(window, cx)
                            })
                        } else {
                            this
                        }
                    })
                    .map(|this| {
                        if let Some(context_menu) = self.context_menu {
                            this.context_menu(move |menu, window, cx| {
                                context_menu(menu, window, cx)
                            })
                            .into_any_element()
                        } else {
                            this.into_any_element()
                        }
                    }),
            )
            .when(is_open, |this| {
                this.child(
                    v_flex()
                        .id("submenu")
                        .border_l_1()
                        .border_color(cx.theme().sidebar_border)
                        .gap_1()
                        .ml_3p5()
                        .pl_2p5()
                        .py_0p5()
                        .children(self.children.into_iter().enumerate().map(|(ix, item)| {
                            let id = format!("{}-{}", id, ix);
                            item.render(id, window, cx).into_any_element()
                        })),
                )
            })
    }
}

impl Styled for SidebarMenuItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapsed_icon_item_uses_label_as_tooltip() {
        let item = SidebarMenuItem::new("Projects")
            .icon(Icon::default())
            .collapsed(true);

        assert_eq!(item.collapsed_tooltip().as_deref(), Some("Projects"));
    }

    #[test]
    fn expanded_or_iconless_item_has_no_collapsed_tooltip() {
        let expanded = SidebarMenuItem::new("Projects").icon(Icon::default());
        let iconless = SidebarMenuItem::new("Projects").collapsed(true);

        assert!(expanded.collapsed_tooltip().is_none());
        assert!(iconless.collapsed_tooltip().is_none());
    }
}


#[cfg(test)]
mod accessibility_tests {
    use super::*;
    use crate::setting::{SettingGroup, SettingItem, SettingPage, Settings};
    use gpui::{Context, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Render, TestAppContext, VisualTestContext};
    use gpui_base::test_support::{snapshots, ElementSnapshot};
    use std::cell::Cell;

    fn named(window: &Window, label: &str) -> ElementSnapshot {
        let matches = snapshots(window).into_iter().filter(|s| s.label() == Some(label)).collect::<Vec<_>>();
        assert_eq!(matches.len(), 1, "expected one accessible {label}");
        matches.into_iter().next().unwrap()
    }

    fn press(cx: &mut VisualTestContext, key: &str) {
        let keystroke = Keystroke::parse(key).unwrap();
        cx.simulate_event(KeyDownEvent { keystroke: keystroke.clone(), is_held: false, prefer_character_input: false });
        cx.simulate_event(KeyUpEvent { keystroke });
    }

    struct SettingsProbe;
    impl Render for SettingsProbe {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Settings::new("settings-a11y-probe").pages([
                SettingPage::new("General")
                    .group(SettingGroup::new().title("Sessions").item(SettingItem::render(|_, _, _| div().child("Session values"))))
                    .group(SettingGroup::new().title("Appearance").item(SettingItem::render(|_, _, _| div().child("Appearance values")))),
                SettingPage::new("Memory").group(SettingGroup::new().item(SettingItem::render(|_, _, _| div().child("Memory values")))),
            ])
        }
    }

    #[gpui::test]
    fn settings_categories_groups_and_carets_report_current_accessibility_state(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, _| SettingsProbe);
        let memory = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            let general = named(window, "General");
            assert_eq!(general.role(), Some(Role::Button));
            assert_eq!(general.selected(), Some(true));
            assert_eq!(general.expanded(), Some(false));
            assert_eq!(named(window, "Memory").selected(), Some(false));
            named(window, "Memory").bounds().center()
        });
        cx.simulate_click(memory, Modifiers::default());
        let general = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "Memory").selected(), Some(true));
            assert_eq!(named(window, "General").selected(), Some(false));
            named(window, "General").bounds().center()
        });
        cx.simulate_click(general, Modifiers::default());
        let sessions = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "General").expanded(), Some(true));
            assert_eq!(named(window, "Collapse General groups").expanded(), Some(true));
            assert_eq!(named(window, "Sessions").role(), Some(Role::Button));
            named(window, "Sessions").bounds().center()
        });
        cx.simulate_click(sessions, Modifiers::default());
        let caret = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "Sessions").selected(), Some(true));
            assert_eq!(named(window, "General").selected(), Some(false));
            named(window, "Collapse General groups").bounds().center()
        });
        cx.simulate_click(caret, Modifiers::default());
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "General").expanded(), Some(false));
            assert_eq!(named(window, "Expand General groups").expanded(), Some(false));
            assert!(!snapshots(window).iter().any(|s| s.label() == Some("Sessions")));
            // The caret must be keyboard reachable without selecting its parent.
            for _ in 0..8 {
                if named(window, "Expand General groups").focused() == Some(true) { break; }
                window.focus_next(cx);
                window.draw(cx).clear(cx);
            }
            assert_eq!(named(window, "Expand General groups").focused(), Some(true));
        });
        press(cx, "enter");
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "General").expanded(), Some(true));
            assert_eq!(named(window, "General").selected(), Some(false));
            assert_eq!(named(window, "Sessions").selected(), Some(true));
        });
        press(cx, "space");
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "General").expanded(), Some(false));
            assert_eq!(named(window, "General").selected(), Some(false));
        });
    }

    struct KeyboardProbe {
        activations: Rc<Cell<usize>>,
    }
    impl Render for KeyboardProbe {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let activations = self.activations.clone();
            div().id("keyboard-probe").tab_group().child(
                SidebarMenu::new().children([
                    SidebarMenuItem::new("Enabled").on_click(move |_, _, _| activations.set(activations.get() + 1)),
                    SidebarMenuItem::new("Disabled").disable(true).on_click(|_, _, _| panic!("disabled item activated")),
                ]).render("keyboard-menu", window, cx)
            )
        }
    }

    #[gpui::test]
    fn sidebar_keyboard_activation_is_single_and_disabled_item_is_not_focusable(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let activations = Rc::new(Cell::new(0));
        let captured = activations.clone();
        let (_, cx) = cx.add_window_view(move |_, _| KeyboardProbe { activations: captured });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            window.draw(cx).clear(cx);
            assert_eq!(named(window, "Enabled").focused(), Some(true));
            assert_ne!(named(window, "Disabled").focused(), Some(true));
        });
        for key in ["enter", "space"] {
            press(cx, key);
        }
        assert_eq!(activations.get(), 2);
        let disabled = cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            window.draw(cx).clear(cx);
            assert_ne!(named(window, "Disabled").focused(), Some(true));
            named(window, "Disabled").bounds().center()
        });
        cx.simulate_click(disabled, Modifiers::default());
        assert_eq!(activations.get(), 2);
    }
}
