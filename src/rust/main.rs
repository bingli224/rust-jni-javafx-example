
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
