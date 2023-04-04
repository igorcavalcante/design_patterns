#[cfg(test)]
mod test {
    use crate::items::*;

    #[test]
    fn calling_add_item_from_an_not_composed_item() {
        let mut treatment = Treatment::new(200.0);
        let sig = Signature::new(100.0);

        let res = treatment.add_item(Box::new(sig));

        assert!(res.is_err())
    }

    #[test]
    fn assemblying_an_complte_glass_product() {
        let treatment = Treatment::new(200.0);
        let mut lens = Lens::new(300.0);
        let _ = lens.add_item(Box::new(treatment));

        let sig = Signature::new(100.0);
        let mut frame = Frame::new(500.0);
        let _ = frame.add_item(Box::new(lens));
        let _ = frame.add_item(Box::new(sig));

        assert_eq!(frame.price(), 1100.0);
    }
}