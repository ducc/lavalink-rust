use ::*;

pub struct Node {

}

impl Node {

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

    pub fn add_node(&mut self, client: Client, server_uri: String, password: String) {
        // headers
        // Authorization: password
        // Num-Shards: client.num_shards.to_string()
        // User-Id: client.user_id.to_string()

        // todo open websocket

        unimplemented!()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        unimplemented!()
    }
}