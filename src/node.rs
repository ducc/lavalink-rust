#[derive(Clone)]
pub struct Node {
    server_uri: String,
    password: String,
    // todo stats
}

impl Node {
    fn new(server_uri: String, password: String) -> Self {
        Self {
            server_uri,
            password,
        }
    }
}

pub struct NodeManager {
    nodes: Vec<Node>,
}

impl NodeManager {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, server_uri: String, password: String) {
        let node = Node::new(server_uri, password);
        self.nodes.push(node);

        // headers
        // Authorization: password
        // Num-Shards: client.num_shards.to_string()
        // User-Id: client.user_id.to_string()

        // todo open websocket

        //unimplemented!()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }
}