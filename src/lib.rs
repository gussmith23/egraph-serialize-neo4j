use egraph_serialize::NodeId;
use neo4rs::{BoltMap, BoltString, BoltType};
use rand::Rng;

fn generate_param_name(len: usize) -> String {
    format!(
        "param_{}",
        rand::rng()
            .sample_iter(rand::distr::Alphanumeric)
            .take(len)
            .map(char::from)
            .collect::<String>()
    )
}

fn make_enode(
    node_commands: &mut Vec<String>,
    relationship_commands: &mut Vec<String>,
    params: &mut Vec<(BoltString, BoltType)>,
    egraph: &egraph_serialize::EGraph,
    node_id: &NodeId,
) {
    let internal_id = node_id.to_string();

    let id_param_name = generate_param_name(8);
    let op_param_name = generate_param_name(8);
    node_commands.push(format!(
        "CREATE (`{internal_id}`:ENode {{id: ${id_param_name}, op: ${op_param_name}}})",
    ));

    params.push((
        BoltString {
            value: id_param_name,
        },
        BoltType::String(neo4rs::BoltString {
            value: node_id.to_string(),
        }),
    ));

    params.push((
        BoltString {
            value: op_param_name,
        },
        BoltType::String(neo4rs::BoltString {
            value: egraph[node_id].op.clone(),
        }),
    ));

    // Create relationships to children.
    for child_nodeid in egraph[node_id].children.iter() {
        let child_eclassid = &egraph[child_nodeid].eclass;
        relationship_commands.push(format!(
            "CREATE (`{internal_id}`)-[:HAS_ARG]->(`{child_internal_id}`)",
            internal_id = internal_id,
            child_internal_id = child_eclassid
        ));
    }
}

fn make_eclass(
    node_commands: &mut Vec<String>,
    relationship_commands: &mut Vec<String>,
    params: &mut Vec<(BoltString, BoltType)>,
    egraph: &egraph_serialize::EGraph,
    class_id: &egraph_serialize::ClassId,
) {
    let internal_id = class_id.to_string();
    let id_param_name = generate_param_name(8);
    node_commands.push(format!(
        "CREATE (`{internal_id}`:EClass {{id: ${id_param_name}}})",
    ));
    params.push((
        BoltString {
            value: id_param_name,
        },
        BoltType::String(neo4rs::BoltString {
            value: class_id.to_string(),
        }),
    ));
    for node_id in egraph[class_id].nodes.iter() {
        let node_internal_id = node_id.to_string();
        relationship_commands.push(format!(
            "CREATE (`{internal_id}`)-[:HAS_ENODE]->(`{node_internal_id}`)",
            internal_id = internal_id,
            node_internal_id = node_internal_id
        ));
    }
}

pub fn commands_from_serialized_egraph(egraph: &egraph_serialize::EGraph) -> (String, BoltMap) {
    let mut node_commands = Vec::new();
    let mut relationship_commands = Vec::new();
    let mut params = Vec::new();

    for (node_id, _node) in egraph.nodes.iter() {
        make_enode(
            &mut node_commands,
            &mut relationship_commands,
            &mut params,
            egraph,
            node_id,
        );
    }
    for (class_id, _class) in egraph.classes().iter() {
        make_eclass(
            &mut node_commands,
            &mut relationship_commands,
            &mut params,
            egraph,
            class_id,
        );
    }

    let query_string = node_commands
        .iter()
        .cloned()
        .chain(relationship_commands.iter().cloned())
        .collect::<Vec<_>>()
        .join("\n");

    let params = BoltMap {
        value: params.into_iter().collect(),
    };

    (query_string, params)
}
