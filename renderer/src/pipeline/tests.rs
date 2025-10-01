
#[cfg(test)]
mod tests {
    use crate::pipeline::instance::create_device_and_queue;


    #[test]
    fn vertex_test() {
        use crate::TestVertex;
        let verts = vec![TestVertex{ pos:[rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)]}; 3];


        for vert in &verts { println!("Vertex: {:?}", vert); }

        assert_eq!(verts.len(), 3);
        assert_ne!(verts.first().expect("Vertex somehow not existing").pos, verts.last().expect("Vertex somehow not existing").pos);
    }

    #[test]
    fn init_test()
    {
        let (device, queue) = create_device_and_queue();

        //assert!(device);
        //assert!(queue);
    }


}

