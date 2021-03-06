use atbash_cipher as cipher;

#[test]
fn test_encode_a() {
    assert_eq!(cipher::encode("a"), "z");
}

#[test]
fn test_encode_yes() {
    assert_eq!(cipher::encode("yes"), "bvh");
}

#[test]
fn test_encode_no() {
    assert_eq!(cipher::encode("no"), "ml");
}

#[test]
fn test_encode_omg() {
    assert_eq!(cipher::encode("OMG"), "lnt");
}

#[test]
fn test_encode_spaces() {
    assert_eq!(cipher::encode("O M G"), "lnt");
}

#[test]
fn test_encode_mindblowingly() {
    assert_eq!(cipher::encode("mindblowingly"), "nrmwy oldrm tob");
}

#[test]
fn test_encode_numbers() {
    assert_eq!(
        cipher::encode("Testing,1 2 3, testing."),
        "gvhgr mt123 gvhgr mt"
    );
}

#[test]
fn test_encode_deep_thought() {
    assert_eq!(cipher::encode("Truth is fiction."), "gifgs rhurx grlm");
}

#[test]
fn test_encode_all_the_letters() {
    assert_eq!(
        cipher::encode("The quick brown fox jumps over the lazy dog."),
        "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"
    );
}

#[test]
fn test_decode_exercism() {
    assert_eq!(cipher::decode("vcvix rhn"), "exercism");
}

#[test]
fn test_decode_a_sentence() {
    assert_eq!(
        cipher::decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v"),
        "anobstacleisoftenasteppingstone"
    );
}

#[test]
fn test_decode_numbers() {
    assert_eq!(cipher::decode("gvhgr mt123 gvhgr mt"), "testing123testing");
}

#[test]
fn test_decode_all_the_letters() {
    assert_eq!(
        cipher::decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"),
        "thequickbrownfoxjumpsoverthelazydog"
    );
}

#[test]
fn test_decode_with_too_many_spaces() {
    assert_eq!(cipher::decode("vc vix    r hn"), "exercism");
}

#[test]
fn test_decode_with_no_spaces() {
    assert_eq!(
        cipher::decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv"),
        "anobstacleisoftenasteppingstone",
    );
}
