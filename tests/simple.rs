use ruspec::ruspec;

ruspec! {
    describe "test module name" {
        before { let context = 5; }
        subject { context + 5 }

        it "test name" {
            assert_eq!(subject, 10);
        }
    }

    describe "test module 2" {
        before { let context = 5; }
        it "test name" {
            assert_eq!(context, 5);
        }

        context "context is 6" {
            before { let context = 6; }
            it "should equal 6" {
                assert_eq!(context, 6);
            }
        }
    }
}
