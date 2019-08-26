struct Stream {
    id: String,
    state: StreamState,
    local_candidates: Vec<Candidate>,
    remote_candidates: Vec<Candidate>,
}
