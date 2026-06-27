#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// The text has stabilized and is safe to format.
    Stable,
    /// The text is still changing (e.g., partial STT delta).
    Unstable,
    /// The text was already formatted or protected and should NOT be touched.
    Protected,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub start: usize,
    pub end: usize,
    pub region_type: RegionType,
}

pub struct RegionClassifier;

impl RegionClassifier {
    pub fn new() -> Self {
        Self
    }

    /// Classifies regions of a string based on their stability.
    pub fn classify(&self, text: &str, protected_spans: &[(usize, usize)]) -> Vec<Region> {
        let mut regions = Vec::new();
        
        let mut current_offset = 0;
        
        for &(p_start, p_end) in protected_spans {
            if p_start > current_offset {
                regions.push(Region {
                    start: current_offset,
                    end: p_start,
                    region_type: RegionType::Stable, // Simplified: non-protected is stable for now
                });
            }
            
            regions.push(Region {
                start: p_start,
                end: p_end,
                region_type: RegionType::Protected,
            });
            
            current_offset = p_end;
        }
        
        if current_offset < text.len() {
            regions.push(Region {
                start: current_offset,
                end: text.len(),
                region_type: RegionType::Stable,
            });
        }
        
        regions
    }
}
