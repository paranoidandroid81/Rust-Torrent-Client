pub trait TrackerService {
    fn send_tracker_request(&self, request: TrackerRequest) -> TrackerResponse {
        
    }
}