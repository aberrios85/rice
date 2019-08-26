// So we receive an SDP Offer or Answer with some remote ice candidates
use crate::candidate::Candidate;
use crate::agent::Role;
use std::cmp;
use std::net::Ipv4Addr;
use std::collections::HashMap;

//https://tools.ietf.org/html/rfc8445#section-6.1.2.6
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PairState{
    Waiting,
    InProgress,
    Succeeded,
    Failed,
    Frozen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CandidatePair {
    local: Candidate,
    remote: Candidate,
    //default: bool,
    //valid: bool,
    //nominated: bool,
    state: PairState,
}

#[derive(Debug, PartialEq)]
pub enum ChecklistState {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, PartialEq)]
// Implement https://tools.ietf.org/html/rfc8445#section-6.1.2.5 (Limit on size of CandidatePair Collection of 100)
pub struct Checklist {
    pairs: Vec<CandidatePair>,
    state: ChecklistState,
}

// RFC 8445 6.1.2.2. Forming Candidate Pairs
// Returns a Checklist, which is a Vector of CandidatePairs along with a state
pub fn pair_candidates(local: Vec<Candidate>, remote: Vec<Candidate>) -> Checklist {
    let mut pairs = vec![];
    // Could we use .zip here?
    for local in local.clone() {
        for remote in remote.clone() {
            if local.component == remote.component {
                let pair = CandidatePair{
                    local: local.clone(),
                    remote,
                    state: PairState::Frozen,
                };
                pairs.push(pair);
            }
            //and if local_candidate.addr == remote_candidate.addr TODO: Add Ipv6 support
        }
    };

    Checklist{ pairs: pairs, state: ChecklistState::Running }
}

// RFC 8445 6.1.2.3. Computing Pair Priority and Ordering Pairs
impl CandidatePair {
    pub fn priority(&self, role: Role) -> u64 {
        let (g, d) = match role {
            Role::Controlling => (self.local.priority() as u64,self.remote.priority() as u64),
            Role::Controlled => (self.remote.priority() as u64,self.local.priority() as u64),
        };

        let priority = (2u64.pow(32)*cmp::min(g, d)) +
        (2*cmp::max(g, d));
        if g>d { priority + 1 } else { priority + 0 };

        println!("Calculated this pair's pair priority as {:?}", priority);
        priority // this is a huge number, is this correct?
    }
}

// RFC 8445 6.1.2.4.  Pruning the Pairs
impl Checklist {
    // Prunes and Orders the Pairs
    pub fn prune(mut self, role: Role) -> Checklist {
        let mut map: HashMap<(Ipv4Addr, Candidate), CandidatePair> = HashMap::new();

        for x in self.pairs {
            // Key is on same local base address AND same remote candidate
            map.entry((x.local.base_addr, x.remote))
                .and_modify(|y| {
                    if x.priority(role) > y.priority(role) {
                        *y = x;
                    }
                })
                .or_insert(x);
        }

        let mut result: Vec<CandidatePair> = map.into_iter().map(|(_, x)| x).collect();
        result.sort_by_key(|x| x.priority(role));
        self.pairs = result;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_candidates() {
        let local_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        let remote_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        // Given 3 remotes and 3 locals there should be 9 total pairs
        assert_eq!(pair_candidates(local_candidates, remote_candidates).pairs.len(), 9);
    }

    #[test]
    fn test_pair_priority_calculation() {
        let local_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        let remote_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        let checklist = pair_candidates(local_candidates, remote_candidates);

        assert_eq!(checklist.pairs[0].priority(Role::Controlled), 9151314442783293438);
    }

    #[test]
    fn test_pair_prune_and_ordering() {
        let local_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        let remote_candidates = vec![Candidate::new(),Candidate::new(),Candidate::new()];
        let checklist = pair_candidates(local_candidates, remote_candidates);
        assert_eq!(checklist.prune(Role::Controlling).pairs.len(), 1);
    }
}
