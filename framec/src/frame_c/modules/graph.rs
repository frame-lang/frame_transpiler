// Dependency graph management for Frame v0.57
// Builds and validates module dependency relationships using topological sorting

use super::errors::{ModuleError, ModuleErrorKind, ModuleResult};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

/// Dependency graph for managing module compilation order
pub struct DependencyGraph {
    /// Map from module path to node information
    nodes: HashMap<String, ModuleNode>,

    /// Adjacency list: module -> list of dependencies
    dependencies: HashMap<String, HashSet<String>>,

    /// Reverse adjacency list: module -> list of dependents
    dependents: HashMap<String, HashSet<String>>,
}

/// Information about a module in the dependency graph
#[derive(Debug, Clone)]
pub struct ModuleNode {
    /// Module identifier (usually file path)
    pub path: String,

    /// Resolved filesystem path
    pub fs_path: PathBuf,

    /// Direct dependencies (modules this module imports)
    pub imports: Vec<String>,

    /// Compilation status
    pub status: CompilationStatus,

    /// Hash of source file for change detection
    pub source_hash: Option<String>,
}

/// Current compilation status of a module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationStatus {
    /// Module discovered but not yet processed
    Pending,

    /// Currently being parsed
    Parsing,

    /// Parsed successfully, AST available
    Parsed,

    /// Compiled successfully
    Compiled,

    /// Loaded from cache
    Cached,

    /// Compilation failed
    Failed,
}

impl DependencyGraph {
    /// Create a new empty dependency graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Add a module to the dependency graph
    pub fn add_module(
        &mut self,
        path: String,
        fs_path: PathBuf,
        imports: Vec<String>,
    ) -> ModuleResult<()> {
        // Create the module node
        let node = ModuleNode {
            path: path.clone(),
            fs_path,
            imports: imports.clone(),
            status: CompilationStatus::Pending,
            source_hash: None,
        };

        // Add to nodes map
        self.nodes.insert(path.clone(), node);

        // Update dependency relationships
        let mut deps = HashSet::new();
        for import in imports {
            deps.insert(import.clone());

            // Add reverse dependency
            self.dependents
                .entry(import)
                .or_insert_with(HashSet::new)
                .insert(path.clone());
        }

        self.dependencies.insert(path, deps);

        Ok(())
    }

    /// Add a dependency relationship between two modules
    pub fn add_dependency(&mut self, from: PathBuf, to: PathBuf) {
        let from_str = from.to_string_lossy().to_string();
        let to_str = to.to_string_lossy().to_string();

        self.dependencies
            .entry(from_str.clone())
            .or_insert_with(HashSet::new)
            .insert(to_str.clone());

        self.dependents
            .entry(to_str)
            .or_insert_with(HashSet::new)
            .insert(from_str);
    }

    /// Check for cycles in the dependency graph
    pub fn check_cycles(&self) -> ModuleResult<()> {
        // Use the compilation_order method which will detect cycles
        self.compilation_order()?;
        Ok(())
    }

    /// Get the build order (alias for compilation_order)
    pub fn get_build_order(&self) -> ModuleResult<Vec<PathBuf>> {
        let order = self.compilation_order()?;
        Ok(order.into_iter().map(PathBuf::from).collect())
    }

    /// Get the compilation order using topological sort
    pub fn compilation_order(&self) -> ModuleResult<Vec<String>> {
        // Kahn's algorithm for topological sorting
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Initialize in-degree counts
        for module in self.nodes.keys() {
            let deps = self.dependencies.get(module).map(|d| d.len()).unwrap_or(0);
            in_degree.insert(module.clone(), deps);

            if deps == 0 {
                queue.push_back(module.clone());
            }
        }

        // Process modules with no dependencies first
        while let Some(module) = queue.pop_front() {
            result.push(module.clone());

            // Update dependents
            if let Some(dependents) = self.dependents.get(&module) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != self.nodes.len() {
            let cycle = self.find_cycle()?;
            return Err(ModuleError::circular_dependency(cycle));
        }

        Ok(result)
    }

    /// Find a cycle in the dependency graph using DFS
    fn find_cycle(&self) -> ModuleResult<Vec<String>> {
        for module in self.nodes.keys() {
            let mut visited = HashSet::new();
            let mut rec_stack = HashSet::new();
            let mut path = Vec::new();

            if let Some(mut cycle) = self.dfs_cycle(module, &mut visited, &mut rec_stack, &mut path)
            {
                // Clean up the cycle paths and remove duplicates
                cycle = cycle
                    .into_iter()
                    .map(|p| self.clean_module_path(&p))
                    .collect();

                // Remove duplicates while preserving cycle structure
                // If the last element equals the first (closing the cycle), keep it
                // If the last element is a duplicate of the second-to-last, remove it
                if cycle.len() > 2 {
                    let last = &cycle[cycle.len() - 1];
                    let second_last = &cycle[cycle.len() - 2];
                    let first = &cycle[0];

                    if last == second_last {
                        // Consecutive duplicate - remove it
                        cycle.pop();
                    } else if last == first && cycle.len() > 3 && second_last == first {
                        // Both last and second-last equal first - remove the duplicate
                        cycle.pop();
                    }
                }

                return Ok(cycle);
            }
        }

        // If topological sort detected a cycle but DFS couldn't find it,
        // try to construct a meaningful error from the remaining unprocessed nodes
        let unprocessed: Vec<String> = self
            .nodes
            .keys()
            .filter(|&module| {
                // Find modules that still have dependencies (couldn't be processed)
                self.dependencies
                    .get(module)
                    .map(|deps| !deps.is_empty())
                    .unwrap_or(false)
            })
            .map(|m| self.clean_module_path(m))
            .collect();

        if unprocessed.len() >= 2 {
            // Build cycle path from dependencies
            let mut cycle = Vec::new();
            let start = unprocessed[0].clone();
            let mut current = start.clone();
            let mut visited_in_path = HashSet::new();

            cycle.push(current.clone());
            visited_in_path.insert(current.clone());

            // Follow dependencies to build cycle
            while cycle.len() <= self.nodes.len() {
                let current_with_prefix = if current.starts_with("./") {
                    current.clone()
                } else {
                    format!("./{}", current)
                };

                if let Some(deps) = self
                    .dependencies
                    .get(&current)
                    .or_else(|| self.dependencies.get(&current_with_prefix))
                {
                    if let Some(next) = deps.iter().next() {
                        let clean_next = self.clean_module_path(next);
                        if clean_next == start {
                            // Found cycle back to start - don't add duplicate
                            break;
                        } else if !visited_in_path.contains(&clean_next) {
                            cycle.push(clean_next.clone());
                            visited_in_path.insert(clean_next.clone());
                            current = clean_next;
                        } else {
                            // Hit a node we've seen - don't add it again
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // Remove consecutive duplicates from the cycle
            let mut deduped_cycle = Vec::new();
            for item in cycle {
                if deduped_cycle.is_empty() || deduped_cycle.last() != Some(&item) {
                    deduped_cycle.push(item);
                }
            }

            Ok(deduped_cycle)
        } else if !unprocessed.is_empty() {
            // Single module with self-dependency
            Ok(vec![unprocessed[0].clone(), unprocessed[0].clone()])
        } else {
            // Fallback to unknown (this really shouldn't happen)
            Ok(vec!["unknown".to_string(), "unknown".to_string()])
        }
    }

    /// Clean up module path for display
    fn clean_module_path(&self, path: &str) -> String {
        // Remove redundant ./ prefixes and normalize paths
        let path = path.replace("././", "./");
        let path = path.replace("//", "/");

        // If path starts with ./, just use the filename
        if path.starts_with("./") {
            path[2..].to_string()
        } else {
            path
        }
    }

    /// DFS helper for cycle detection
    fn dfs_cycle(
        &self,
        module: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(module.to_string());
        rec_stack.insert(module.to_string());
        path.push(module.to_string());

        if let Some(deps) = self.dependencies.get(module) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle - extract the cycle from path
                    let cycle_start = path.iter().position(|m| m == dep)?;
                    let mut cycle = path[cycle_start..].to_vec();
                    // Only add the closing module if it's not already the last element
                    if cycle.last() != Some(dep) {
                        cycle.push(dep.clone()); // Close the cycle
                    }
                    return Some(cycle);
                }
            }
        }

        rec_stack.remove(module);
        path.pop();
        None
    }

    /// Update module status
    pub fn update_status(
        &mut self,
        module_path: &str,
        status: CompilationStatus,
    ) -> ModuleResult<()> {
        if let Some(node) = self.nodes.get_mut(module_path) {
            node.status = status;
            Ok(())
        } else {
            Err(ModuleError::new(
                ModuleErrorKind::NotFound {
                    path: module_path.to_string(),
                    searched_paths: vec![],
                },
                module_path.to_string(),
            ))
        }
    }

    /// Get module information
    pub fn get_module(&self, module_path: &str) -> Option<&ModuleNode> {
        self.nodes.get(module_path)
    }

    /// Get all modules
    pub fn get_all_modules(&self) -> impl Iterator<Item = (&String, &ModuleNode)> {
        self.nodes.iter()
    }

    /// Check if module has any failed dependencies
    pub fn has_failed_dependencies(&self, module_path: &str) -> bool {
        if let Some(deps) = self.dependencies.get(module_path) {
            for dep in deps {
                if let Some(node) = self.nodes.get(dep) {
                    if node.status == CompilationStatus::Failed {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get modules that depend on the given module
    pub fn get_dependents(&self, module_path: &str) -> Vec<String> {
        self.dependents
            .get(module_path)
            .map(|deps| deps.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get statistics about the dependency graph
    pub fn get_stats(&self) -> GraphStats {
        let mut total_edges = 0;
        for deps in self.dependencies.values() {
            total_edges += deps.len();
        }

        let status_counts = self.nodes.values().fold(HashMap::new(), |mut acc, node| {
            *acc.entry(node.status.clone()).or_insert(0) += 1;
            acc
        });

        GraphStats {
            total_modules: self.nodes.len(),
            total_dependencies: total_edges,
            status_counts,
        }
    }
}

/// Statistics about the dependency graph
#[derive(Debug)]
pub struct GraphStats {
    pub total_modules: usize,
    pub total_dependencies: usize,
    pub status_counts: HashMap<CompilationStatus, usize>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
