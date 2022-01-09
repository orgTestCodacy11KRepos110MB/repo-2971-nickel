use std::collections::HashMap;

use codespan::ByteIndex;
use log::debug;
use nickel::{
    position::TermPos,
    term::MetaValue,
    typecheck::linearization::{LinearizationState, ScopeId},
};

use super::{
    interface::{Resolved, TermKind, UsageState},
    LinearizationItem,
};

#[derive(Debug, Default)]
pub struct Completed {
    pub lin: Vec<LinearizationItem<Resolved>>,
    pub id_mapping: HashMap<usize, usize>,
    pub scope_mapping: HashMap<Vec<ScopeId>, Vec<usize>>,
}

impl Completed {
    pub fn get_item(&self, id: usize) -> Option<&LinearizationItem<Resolved>> {
        self.id_mapping
            .get(&id)
            .and_then(|index| self.lin.get(*index))
    }

    pub fn get_in_scope(
        &self,
        LinearizationItem { scope, .. }: &LinearizationItem<Resolved>,
    ) -> Vec<&LinearizationItem<Resolved>> {
        (0..scope.len())
            .into_iter()
            .map(|end| &scope[..=end])
            .flat_map(|scope| {
                eprintln!("in scope {:?}: {:?}", scope, self.scope_mapping.get(scope));

                self.scope_mapping
                    .get(scope)
                    .map_or_else(Vec::new, Clone::clone)
            })
            .map(|id| self.get_item(id))
            .flatten()
            .collect()
    }

    /// Finds the index of a linearization item for a given location
    /// The linearization is a list of items that are sorted by their physical occurence.
    /// - Each element has a corresponding span in the source
    /// - Spans are either equal (same starting point, same length)
    ///   or shorter but never intersecting
    ///
    ///   (start_element_2 >= start_element_1 AND end_element_2 <= end_element_1)
    ///
    /// For any location a binary search is used to efficiently find the index
    /// of the *last* element that starts at this position.
    /// This corresponds to the most concrete Element as the linearization is
    /// 1. produced by a stable sort and
    /// 2. lower elements are more concrete
    ///
    /// If a perfect match cannot be found, the binary search still provides an
    /// anchor point from which we reversely find the first element that *contains*
    /// the location looked up
    ///
    /// If neither is possible `None` is returned as no corresponding linearization
    /// item could be found.
    ///
    pub fn item_at(
        &self,
        locator: (codespan::FileId, ByteIndex),
    ) -> Option<&LinearizationItem<Resolved>> {
        let (file_id, start) = locator;
        let linearization = &self.lin;
        let item = match linearization.binary_search_by_key(&locator, |item| match item.pos {
            TermPos::Original(span) | TermPos::Inherited(span) => (span.src_id, span.start),
            TermPos::None => unreachable!(),
        }) {
            // Found item(s) starting at `locator`
            // search for most precise element
            Ok(index) => linearization[index..]
                .iter()
                .enumerate()
                .take_while(|(_, item)| {
                    let pos = match item.pos {
                        TermPos::Original(span) | TermPos::Inherited(span) => {
                            (span.src_id, span.start)
                        }
                        TermPos::None => unreachable!(),
                    };
                    pos == (file_id, start)
                })
                .inspect(|(offset, item)| debug!("taken: {:?} @ {}", item, index + offset))
                .map(|(_, item)| item)
                .last(),
            // No perfect match found
            // iterate back finding the first wrapping linearization item
            Err(index) => {
                linearization[..index].iter().rfold(None, |acc, item| {
                    let pos = match item.pos {
                        TermPos::Original(pos) | TermPos::Inherited(pos) => {
                            Some((pos.start, pos.end, pos.src_id))
                        }
                        TermPos::None => None,
                    };
                    // Returning the stored item directly ensures we return the first (reversly) found item
                    acc.or_else(|| {
                        if pos == None {
                            return None;
                        }
                        let (istart, iend, ifile) = pos.unwrap();

                        debug!(
                            "{} < {} < {} in {:?} = {:?}",
                            istart, start, iend, file_id, ifile
                        );

                        if file_id == ifile && start > istart && start < iend {
                            return Some(item);
                        }

                        None
                    })
                })
            }
        };
        item
    }

    /// Resolve type and meta information for a given item
    pub fn resolve_item_type_meta(
        &self,
        item: &LinearizationItem<Resolved>,
    ) -> (Resolved, Vec<String>) {
        let mut extra = Vec::new();

        let item = match item.kind {
            TermKind::Usage(UsageState::Resolved(usage)) => {
                usage.and_then(|u| self.get_item(u + 1)).unwrap_or(item)
            }
            TermKind::Declaration(_, _) => self.get_item(item.id + 1).unwrap_or(item),
            _ => item,
        };

        if let Some(MetaValue {
            ref doc,
            ref types,
            ref contracts,
            priority,
            ..
        }) = item.meta.as_ref()
        {
            if let Some(doc) = doc {
                extra.push(doc.to_owned());
            }
            if let Some(types) = types {
                extra.push(types.label.tag.to_string());
            }
            if !contracts.is_empty() {
                extra.push(
                    contracts
                        .iter()
                        .map(|contract| format!("{}", contract.label.types,))
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }

            extra.push(format!("Merge Priority: {:?}", priority));
        }

        (item.ty.to_owned(), extra)
    }
}

impl LinearizationState for Completed {}