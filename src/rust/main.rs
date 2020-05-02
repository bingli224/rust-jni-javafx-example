
/**
 * Test Java+Rust with JNI
 *
 * 20:44 THA 02/05/2020
 *
 * References:
 *	https://docs.rs/jni/0.16.0/jni/struct.JavaVM.html
 *	https://l.facebook.com/l.php?u=http%3A%2F%2Fweb.mit.edu%2Fjavadev%2Fdoc%2Ftutorial%2Fnative1.1%2Fimplementing%2Fmethod.html%3Ffbclid%3DIwAR2iCUxBsg6fffxuXAgbuJ-6jpMlteCZQ2VMKxr_hnzbSxcCEvSbGuYpiBM&h=AT2xAylCtafmCK4wHHV-cEci8vmJKQN4YR39Of2nKf7Je1K5Jt3FAB7It3_rFhsIzwQT7i8VmVwbwv2tnSJ-PCcJPE4vl99uhRb-23mRM9JnAsJknUqASU46-IUhto3xYym4&__tn__=R]-R&c[0]=AT03hywS44qs5noAiPLvFP9TMDsqMS2wQ-Gtm1cMXcU1ZJKdW99GTn4Gmk78ns_1YBEhOfzNkonzg55DR0X0kaO-ivUiv4CNJv4g_wiobqwUEMxqOYp0wLcPOTP3YhJcOy2_mDiR9RfOnw
 *	https://stackoverflow.com/questions/56821728/how-can-i-invoke-a-java-method-from-rust-via-jni
 *	https://l.facebook.com/l.php?u=https%3A%2F%2Fstackoverflow.com%2Fquestions%2F17357712%2Fcalling-main-method-from-jni-fails%2F17360803%3Ffbclid%3DIwAR3u-BDMBikNF5uTUvRlFizCoHIINRP4rLrEkXrriSv8j4BwaCKtVtoqIKY&h=AT2cKIOb7NEQ4lpb38XmgBesIrqH17IZwfr47Xr2wYzwVjSIo-ZUuSBp1h6iU-BxhHu9-Di7gBpc6Ks3CNJk_jUZc1E5LeerH6E-bfytpGRZ0Eu0-LMAtuYAEP-0lssMJI7i&__tn__=R]-R&c[0]=AT03hywS44qs5noAiPLvFP9TMDsqMS2wQ-Gtm1cMXcU1ZJKdW99GTn4Gmk78ns_1YBEhOfzNkonzg55DR0X0kaO-ivUiv4CNJv4g_wiobqwUEMxqOYp0wLcPOTP3YhJcOy2_mDiR9RfOnw
 */
use jni::{
	self,
	JavaVM,
	JNIVersion,
	InitArgsBuilder,
	objects::{
		JValue,
				JObject,
	},
};

fn main() {
	let jvm_args = InitArgsBuilder::new ( )
		.version ( JNIVersion::V8 )
		.option ( "-Xcheck:jni" )
		.option ( "-Djava.class.path=.;lib/App.jar;lib/jfxrt.jar" )
		.build ( )
		.unwrap ( );

	let jvm = JavaVM::new ( jvm_args )
		.unwrap ( );
	let jthrd = jvm.attach_current_thread ( )
		.unwrap ( );

	// load my App class
	let app = jthrd.find_class ( "App" ).unwrap ( );
		
	/*
	// test to call the main(String..)
	jthrd.call_static_method (
		app,
		"main",
		"([Ljava/lang/String;)V",
		& [ JValue::from ( JObject::null ( ) ) ]
	).unwrap ( );
	*/

	// test to call the Application.launch(Class,String..)
	jthrd.call_static_method (
		"javafx/application/Application",
		"launch",
		"(Ljava/lang/Class;[Ljava/lang/String;)V",
		& [
			app.into ( ),
			JValue::from ( JObject::null ( ) )
		]
	).unwrap ( );
}
