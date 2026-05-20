use crate::JsRudofConfig;
use napi::Error;
use napi_derive::napi;
use rudof_lib::{
    Rudof,
    errors::RudofError,
    formats::{
        DataFormat, DataReaderMode, InputSpec, ResultDataFormat, ResultShExValidationFormat,
        ResultShaclValidationFormat, ShExFormat, ShaclFormat, ShaclValidationMode,
        ShaclValidationSortByMode, ShExValidationSortByMode, ShapeMapFormat,
    },
};
use std::{io::BufWriter, str::FromStr};

fn cnv_err(e: RudofError) -> Error { Error::from_reason(e.to_string()) }
fn cnv_input(s: &str) -> napi::Result<InputSpec> { InputSpec::from_str(s).map_err(|e| Error::from_reason(e.to_string())) }
fn serialize_output(w: BufWriter<Vec<u8>>) -> napi::Result<String> {
    let bytes = w.into_inner().map_err(|e| Error::from_reason(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| Error::from_reason(e.to_string()))
}

// ── Enums ─────────────────────────────────────────────────────────────────────

#[napi(string_enum)] pub enum JsRdfFormat { Turtle, NTriples, NQuads, TriG, RdfXml, JsonLd, N3 }
#[napi(string_enum)] pub enum JsShaclFormat { Turtle, NTriples, NQuads, TriG, RdfXml, JsonLd, N3 }
#[napi(string_enum)] pub enum JsShExFormat { ShExC, ShExJ, Turtle }
#[napi(string_enum)] pub enum JsShapeMapFormat { Compact, Json }
#[napi(string_enum)] pub enum JsReaderMode { Lax, Strict }
#[napi(string_enum)] pub enum JsShaclValidationMode { Native, Sparql }

#[napi(string_enum)]
pub enum JsShaclValidationSortMode { Severity, Node, Component, Value, Path, SourceShape, Details }

#[napi(string_enum)]
pub enum JsResultShaclValidationFormat {
    Details, Turtle, NTriples, RdfXml, TriG, N3, NQuads, Minimal, Compact, Json, Csv,
}

#[napi(string_enum)]
pub enum JsResultShexValidationFormat {
    Details, Turtle, NTriples, RdfXml, TriG, N3, NQuads, Compact, Json, Csv,
}

#[napi(string_enum)]
pub enum JsShexValidationSortMode { Node, Shape, Status, Details }

#[napi(string_enum)]
pub enum JsResultDataFormat { Turtle, NTriples, NQuads, JsonLd, RdfXml, TriG, N3, Compact, Json }

// ── Converters ────────────────────────────────────────────────────────────────

fn cnv_rdf_format(f: Option<JsRdfFormat>) -> Option<DataFormat> {
    f.map(|f| match f {
        JsRdfFormat::Turtle => DataFormat::Turtle, JsRdfFormat::NTriples => DataFormat::NTriples,
        JsRdfFormat::NQuads => DataFormat::NQuads, JsRdfFormat::TriG => DataFormat::TriG,
        JsRdfFormat::RdfXml => DataFormat::RdfXml, JsRdfFormat::JsonLd => DataFormat::JsonLd,
        JsRdfFormat::N3 => DataFormat::N3,
    })
}
fn cnv_shacl_format(f: Option<JsShaclFormat>) -> Option<ShaclFormat> {
    f.map(|f| match f {
        JsShaclFormat::Turtle => ShaclFormat::Turtle, JsShaclFormat::NTriples => ShaclFormat::NTriples,
        JsShaclFormat::NQuads => ShaclFormat::NQuads, JsShaclFormat::TriG => ShaclFormat::TriG,
        JsShaclFormat::RdfXml => ShaclFormat::RdfXml, JsShaclFormat::JsonLd => ShaclFormat::JsonLd,
        JsShaclFormat::N3 => ShaclFormat::N3,
    })
}
fn cnv_shex_format(f: Option<JsShExFormat>) -> Option<ShExFormat> {
    f.map(|f| match f {
        JsShExFormat::ShExC => ShExFormat::ShExC, JsShExFormat::ShExJ => ShExFormat::ShExJ,
        JsShExFormat::Turtle => ShExFormat::Turtle,
    })
}
fn cnv_shapemap_format(f: Option<JsShapeMapFormat>) -> Option<ShapeMapFormat> {
    f.map(|f| match f { JsShapeMapFormat::Compact => ShapeMapFormat::Compact, JsShapeMapFormat::Json => ShapeMapFormat::Json })
}
fn cnv_reader_mode(m: Option<JsReaderMode>) -> Option<DataReaderMode> {
    m.map(|m| match m { JsReaderMode::Lax => DataReaderMode::Lax, JsReaderMode::Strict => DataReaderMode::Strict })
}
fn cnv_shacl_validation_mode(m: Option<JsShaclValidationMode>) -> Option<ShaclValidationMode> {
    m.map(|m| match m { JsShaclValidationMode::Native => ShaclValidationMode::Native, JsShaclValidationMode::Sparql => ShaclValidationMode::Sparql })
}
fn cnv_shacl_sort_mode(m: Option<JsShaclValidationSortMode>) -> Option<ShaclValidationSortByMode> {
    m.map(|m| match m {
        JsShaclValidationSortMode::Severity => ShaclValidationSortByMode::Severity,
        JsShaclValidationSortMode::Node => ShaclValidationSortByMode::Node,
        JsShaclValidationSortMode::Component => ShaclValidationSortByMode::Component,
        JsShaclValidationSortMode::Value => ShaclValidationSortByMode::Value,
        JsShaclValidationSortMode::Path => ShaclValidationSortByMode::Path,
        JsShaclValidationSortMode::SourceShape => ShaclValidationSortByMode::SourceShape,
        JsShaclValidationSortMode::Details => ShaclValidationSortByMode::Details,
    })
}
fn cnv_result_shacl_format(f: Option<JsResultShaclValidationFormat>) -> Option<ResultShaclValidationFormat> {
    f.map(|f| match f {
        JsResultShaclValidationFormat::Details => ResultShaclValidationFormat::Details,
        JsResultShaclValidationFormat::Turtle => ResultShaclValidationFormat::Turtle,
        JsResultShaclValidationFormat::NTriples => ResultShaclValidationFormat::NTriples,
        JsResultShaclValidationFormat::RdfXml => ResultShaclValidationFormat::RdfXml,
        JsResultShaclValidationFormat::TriG => ResultShaclValidationFormat::TriG,
        JsResultShaclValidationFormat::N3 => ResultShaclValidationFormat::N3,
        JsResultShaclValidationFormat::NQuads => ResultShaclValidationFormat::NQuads,
        JsResultShaclValidationFormat::Minimal => ResultShaclValidationFormat::Minimal,
        JsResultShaclValidationFormat::Compact => ResultShaclValidationFormat::Compact,
        JsResultShaclValidationFormat::Json => ResultShaclValidationFormat::Json,
        JsResultShaclValidationFormat::Csv => ResultShaclValidationFormat::Csv,
    })
}
fn cnv_result_shex_format(f: Option<JsResultShexValidationFormat>) -> Option<ResultShExValidationFormat> {
    f.map(|f| match f {
        JsResultShexValidationFormat::Details => ResultShExValidationFormat::Details,
        JsResultShexValidationFormat::Turtle => ResultShExValidationFormat::Turtle,
        JsResultShexValidationFormat::NTriples => ResultShExValidationFormat::NTriples,
        JsResultShexValidationFormat::RdfXml => ResultShExValidationFormat::RdfXml,
        JsResultShexValidationFormat::TriG => ResultShExValidationFormat::TriG,
        JsResultShexValidationFormat::N3 => ResultShExValidationFormat::N3,
        JsResultShexValidationFormat::NQuads => ResultShExValidationFormat::NQuads,
        JsResultShexValidationFormat::Compact => ResultShExValidationFormat::Compact,
        JsResultShexValidationFormat::Json => ResultShExValidationFormat::Json,
        JsResultShexValidationFormat::Csv => ResultShExValidationFormat::Csv,
    })
}
fn cnv_shex_sort_mode(m: Option<JsShexValidationSortMode>) -> Option<ShExValidationSortByMode> {
    m.map(|m| match m {
        JsShexValidationSortMode::Node => ShExValidationSortByMode::Node,
        JsShexValidationSortMode::Shape => ShExValidationSortByMode::Shape,
        JsShexValidationSortMode::Status => ShExValidationSortByMode::Status,
        JsShexValidationSortMode::Details => ShExValidationSortByMode::Details,
    })
}
fn cnv_result_data_format(f: Option<JsResultDataFormat>) -> Option<ResultDataFormat> {
    f.map(|f| match f {
        JsResultDataFormat::Turtle => ResultDataFormat::Turtle,
        JsResultDataFormat::NTriples => ResultDataFormat::NTriples,
        JsResultDataFormat::NQuads => ResultDataFormat::NQuads,
        JsResultDataFormat::JsonLd => ResultDataFormat::JsonLd,
        JsResultDataFormat::RdfXml => ResultDataFormat::RdfXml,
        JsResultDataFormat::TriG => ResultDataFormat::TriG,
        JsResultDataFormat::N3 => ResultDataFormat::N3,
        JsResultDataFormat::Compact => ResultDataFormat::Compact,
        JsResultDataFormat::Json => ResultDataFormat::Json,
    })
}

// ── Rudof ─────────────────────────────────────────────────────────────────────

/// The central Rudof context. Holds all loaded state: data, schemas, and validation results.
///
/// All methods are synchronous. Operations that query SPARQL endpoints will block
/// the calling thread — offload with `node:worker_threads` or a thread pool.
#[napi(js_name = "Rudof")]
pub struct JsRudof { inner: Rudof }

#[napi]
impl JsRudof {
    /// Creates a new `Rudof` instance with the given configuration.
    #[napi(constructor)]
    pub fn new(config: &JsRudofConfig) -> Self {
        Self { inner: Rudof::new(config.inner.clone()) }
    }

    // Reset
    #[napi] pub fn reset_all(&mut self) { self.inner.reset_all().execute(); }
    #[napi] pub fn reset_data(&mut self) { self.inner.reset_data().execute(); }
    #[napi] pub fn reset_shacl(&mut self) { self.inner.reset_shacl_shapes().execute(); }
    #[napi] pub fn reset_shex(&mut self) { self.inner.reset_shex().execute(); }
    #[napi] pub fn reset_shapemap(&mut self) { self.inner.reset_shapemap().execute(); }
    #[napi] pub fn reset_query(&mut self) { self.inner.reset_query().execute(); }

    // Data

    /// Loads RDF data from a string, file path, URL, or SPARQL endpoint.
    /// When `endpoint` is provided the `input` parameter is ignored.
    #[napi]
    pub fn read_data(
        &mut self,
        input: Option<String>,
        format: Option<JsRdfFormat>,
        base: Option<String>,
        reader_mode: Option<JsReaderMode>,
        merge: Option<bool>,
        endpoint: Option<String>,
    ) -> napi::Result<()> {
        let mut parsed_input = None;
        if let Some(ref s) = input { parsed_input = Some(vec![cnv_input(s)?]); }
        let rdf_format = cnv_rdf_format(format);
        let rm = cnv_reader_mode(reader_mode);
        let mut b = self.inner.load_data();
        if let Some(ref specs) = parsed_input { b = b.with_data(specs); }
        if let Some(ref f) = rdf_format { b = b.with_data_format(f); }
        if let Some(ref s) = base { b = b.with_base(s); }
        if let Some(ref rm) = rm { b = b.with_reader_mode(rm); }
        if let Some(m) = merge { b = b.with_merge(m); }
        if let Some(ref ep) = endpoint { b = b.with_endpoint(ep); }
        b.execute().map_err(cnv_err)
    }

    /// Serializes the currently loaded RDF data to a string.
    #[napi]
    pub fn serialize_data(&mut self, format: Option<JsResultDataFormat>) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_result_data_format(format);
        let mut b = self.inner.serialize_data(&mut writer);
        if let Some(ref f) = fmt { b = b.with_result_data_format(f); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    // SHACL

    /// Loads SHACL shapes from a string, file path, or URL.
    /// If `input` is omitted, shapes are extracted from the currently loaded data graph.
    #[napi]
    pub fn read_shacl(
        &mut self,
        input: Option<String>,
        format: Option<JsShaclFormat>,
        base: Option<String>,
        reader_mode: Option<JsReaderMode>,
    ) -> napi::Result<()> {
        let mut parsed_input = None;
        if let Some(ref s) = input { parsed_input = Some(cnv_input(s)?); }
        let shacl_format = cnv_shacl_format(format);
        let rm = cnv_reader_mode(reader_mode);
        let mut b = self.inner.load_shacl_shapes();
        if let Some(ref spec) = parsed_input { b = b.with_shacl_schema(spec); }
        if let Some(ref f) = shacl_format { b = b.with_shacl_schema_format(f); }
        if let Some(ref s) = base { b = b.with_base(s); }
        if let Some(ref rm) = rm { b = b.with_reader_mode(rm); }
        b.execute().map_err(cnv_err)
    }

    /// Serializes the currently loaded SHACL shapes to a string.
    #[napi]
    pub fn serialize_shacl(&mut self, format: Option<JsShaclFormat>) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_shacl_format(format);
        let mut b = self.inner.serialize_shacl_shapes(&mut writer);
        if let Some(ref f) = fmt { b = b.with_shacl_result_format(f); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    /// Validates the loaded RDF data against the loaded SHACL shapes.
    /// Call `readData` and `readShacl` first.
    #[napi]
    pub fn validate_shacl(&mut self, mode: Option<JsShaclValidationMode>) -> napi::Result<()> {
        let m = cnv_shacl_validation_mode(mode);
        let mut b = self.inner.validate_shacl();
        if let Some(ref m) = m { b = b.with_shacl_validation_mode(m); }
        b.execute().map_err(cnv_err)
    }

    /// Serializes the results of the last SHACL validation to a string.
    /// Call `validateShacl` first.
    #[napi]
    pub fn serialize_shacl_validation_results(
        &mut self,
        format: Option<JsResultShaclValidationFormat>,
        sort_mode: Option<JsShaclValidationSortMode>,
    ) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_result_shacl_format(format);
        let sm = cnv_shacl_sort_mode(sort_mode);
        let mut b = self.inner.serialize_shacl_validation_results(&mut writer);
        if let Some(ref f) = fmt { b = b.with_result_shacl_validation_format(f); }
        if let Some(ref s) = sm { b = b.with_shacl_validation_sort_order_mode(s); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    // ShEx

    /// Loads a ShEx schema from a string, file path, or URL.
    #[napi]
    pub fn read_shex(
        &mut self,
        input: String,
        format: Option<JsShExFormat>,
        base: Option<String>,
        reader_mode: Option<JsReaderMode>,
    ) -> napi::Result<()> {
        let spec = cnv_input(&input)?;
        let shex_format = cnv_shex_format(format);
        let rm = cnv_reader_mode(reader_mode);
        let mut b = self.inner.load_shex_schema(&spec);
        if let Some(ref f) = shex_format { b = b.with_shex_schema_format(f); }
        if let Some(ref s) = base { b = b.with_base(s); }
        if let Some(ref rm) = rm { b = b.with_reader_mode(rm); }
        b.execute().map_err(cnv_err)
    }

    /// Serializes the currently loaded ShEx schema to a string.
    #[napi]
    pub fn serialize_shex(&mut self, format: Option<JsShExFormat>) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_shex_format(format);
        let mut b = self.inner.serialize_shex_schema(&mut writer);
        if let Some(ref f) = fmt { b = b.with_result_shex_format(f); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    /// Validates the loaded RDF data against the loaded ShEx schema using the loaded ShapeMap.
    /// Call `readData`, `readShex`, and `readShapemap` first.
    #[napi]
    pub fn validate_shex(&mut self) -> napi::Result<()> {
        self.inner.validate_shex().execute().map_err(cnv_err)
    }

    /// Serializes the results of the last ShEx validation to a string.
    #[napi]
    pub fn serialize_shex_validation_results(
        &mut self,
        format: Option<JsResultShexValidationFormat>,
        sort_mode: Option<JsShexValidationSortMode>,
    ) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_result_shex_format(format);
        let sm = cnv_shex_sort_mode(sort_mode);
        let mut b = self.inner.serialize_shex_validation_results(&mut writer);
        if let Some(ref f) = fmt { b = b.with_result_shex_validation_format(f); }
        if let Some(ref s) = sm { b = b.with_shex_validation_sort_order_mode(s); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    // ShapeMap

    /// Loads a ShapeMap from a string, file path, or URL.
    #[napi]
    pub fn read_shapemap(
        &mut self,
        input: String,
        format: Option<JsShapeMapFormat>,
        base_nodes: Option<String>,
        base_shapes: Option<String>,
    ) -> napi::Result<()> {
        let spec = cnv_input(&input)?;
        let sm_format = cnv_shapemap_format(format);
        let mut b = self.inner.load_shapemap(&spec);
        if let Some(ref f) = sm_format { b = b.with_shapemap_format(f); }
        if let Some(ref bn) = base_nodes { b = b.with_base_nodes(bn); }
        if let Some(ref bs) = base_shapes { b = b.with_base_shapes(bs); }
        b.execute().map_err(cnv_err)
    }

    /// Serializes the currently loaded ShapeMap to a string.
    #[napi]
    pub fn serialize_shapemap(&mut self, format: Option<JsShapeMapFormat>) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let fmt = cnv_shapemap_format(format);
        let mut b = self.inner.serialize_shapemap(&mut writer);
        if let Some(ref f) = fmt { b = b.with_result_shapemap_format(f); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    // SPARQL query

    /// Loads a SPARQL query from a string, file path, or URL.
    #[napi]
    pub fn read_query(&mut self, input: String) -> napi::Result<()> {
        let spec = cnv_input(&input)?;
        self.inner.load_query(&spec).execute().map_err(cnv_err)
    }

    /// Executes the loaded SPARQL query against the loaded data.
    #[napi]
    pub fn run_query(&mut self) -> napi::Result<()> {
        self.inner.run_query().execute().map_err(cnv_err)
    }

    /// Serializes the results of the last executed SPARQL query to a string.
    #[napi]
    pub fn serialize_query_results(&mut self) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        self.inner.serialize_query_results(&mut writer).execute().map_err(cnv_err)?;
        serialize_output(writer)
    }

    // Misc

    /// Lists known SPARQL endpoints from the current configuration.
    /// Returns an array of `[name, url]` pairs.
    #[napi]
    pub fn list_endpoints(&mut self) -> napi::Result<Vec<Vec<String>>> {
        let eps = self.inner.list_endpoints().execute().map_err(cnv_err)?;
        Ok(eps.into_iter().map(|(k, v)| vec![k, v]).collect())
    }

    /// Returns a formatted neighbourhood view of a node in the RDF graph.
    #[napi]
    pub fn node_info(
        &mut self,
        node_selector: String,
        show_colors: Option<bool>,
        depth: Option<u32>,
    ) -> napi::Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        let mut b = self.inner.show_node_info(&node_selector, &mut writer);
        if let Some(c) = show_colors { b = b.with_show_colors(c); }
        if let Some(d) = depth { b = b.with_depth(d as usize); }
        b.execute().map_err(cnv_err)?;
        serialize_output(writer)
    }
}
