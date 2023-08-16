use serde::{Deserialize, Serialize};
use serde_json;
use ndarray::prelude::*;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use walkdir::{DirEntry, WalkDir};


struct PathManage {
    _root: PathBuf
}

impl PathManage {

    fn new(root: &str) -> PathManage {
        PathManage{_root: PathManage::find_root(root).unwrap()}
    }

    fn split(path: &PathBuf) -> Vec<&str> {
        path.components().map(|c| c.as_os_str().to_str().unwrap()).collect::<Vec<&str>>()
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with("."))
             .unwrap_or(false)
    }

    fn path(&self, path: &str) -> Option<PathBuf> {
        let path_pathbuf = PathBuf::from(path);
        let path_parts = Self::split(&path_pathbuf);
        //let path_parts: Vec<&str> = Path::new(path).components().map(|c| c.as_os_str().to_str().unwrap()).collect();
        let path_parts: Vec<&str> = path_parts.into_iter().filter(|&part| part != "." && part != ".." && !part.is_empty()).collect();
        let length = path_parts.len();

        for entry in WalkDir::new(self._root
                                                    .as_os_str()
                                                    .to_str().unwrap())
                                                    .into_iter()
                                                    .filter_map(|e| e.ok()) 
                                                    //.filter(|e| Self::is_hidden(e))
        {
            let entry_path: PathBuf = entry.path().to_path_buf();
            let entry_parts = Self::split(&entry_path);
            if (entry_parts.len() as i32) - (length as i32) < 1 { // Avoid neg index and cast to i32 to avoid overflows
                continue;
            }
            let semipath = entry_parts[entry_parts.len() - length ..].to_vec();
            if path_parts.iter().all(|item| semipath.contains(item)) {
                return Some(path_pathbuf);
            }

        }

        None
    }

    fn find_root(root_name: &str) -> Result<PathBuf, std::io::Error> {
        let current_dir = env::current_dir()?;
        let mut path = current_dir.components().collect::<Vec<_>>();
        path.reverse();

        let mut dir = PathBuf::new();
        dir.push(".");

        for component in path {
            if let Some(name) = component.as_os_str().to_str() {
                if name != root_name {
                    dir.push("..");
                } else {
                    return Ok(dir);
                }
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Root directory not found",
        ))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Data<T> {
    FftFreqVals {
        n: T,
        d: T
    },

    ComplexVals {
        mag: Vec<T>,
        phase: Vec<T>
    },

    Array(Vec<T>),

    SineFreqVals {
        fsine: T,
        fsample: T,
        duration: T
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Json {
    pub input_data: Data<f64>,
    pub output_data: Data<f64>, 
    pub function: String,
    pub path: String
}

pub fn read_json(lib_path: &str) -> Json {
    let path = PathManage::new("RusticFourier");
    let json_path = path.path(lib_path).unwrap();//.as_os_str().to_str().unwrap();
    let file = fs::File::open(json_path).unwrap();
    //let reader = std::io::BufReader::new(file);

    let data: Json = serde_json::from_reader(file).unwrap();
    return data;
}

pub fn write_json() {
    std::unimplemented!();
}

#[cfg(test)]
mod tests {
    use crate::io;
    use ndarray::prelude::*;
    use std::path::{Path, PathBuf};
    use std::env;
    use serde_json;

    #[test]
    fn read_sine() {
        let input = vec![0.0, 0.12582098237155617, 0.25164196474311235, 0.37746294711466855, 0.5032839294862247, 0.6291049118577808, 0.7549258942293371, 0.8807468766008931, 1.0065678589724494, 1.1323888413440055, 1.2582098237155617, 1.384030806087118, 1.5098517884586742, 1.6356727708302303, 1.7614937532017862, 1.8873147355733426, 2.0131357179448988, 2.138956700316455, 2.264777682688011, 2.390598665059567, 2.5164196474311233, 2.64224062980268, 2.768061612174236, 2.8938825945457918, 3.0197035769173484, 3.1455245592889045, 3.2713455416604607, 3.3971665240320164, 3.5229875064035725, 3.6488084887751295, 3.7746294711466852, 3.9004504535182414, 4.0262714358897975, 4.152092418261353, 4.27791340063291, 4.4037343830044655, 4.529555365376022, 4.655376347747579, 4.781197330119134, 4.907018312490691, 5.032839294862247, 5.158660277233802, 5.28448125960536, 5.410302241976916, 5.536123224348472, 5.661944206720028, 5.7877651890915836, 5.91358617146314, 6.039407153834697, 6.165228136206252, 6.291049118577809, 6.416870100949364, 6.542691083320921, 6.668512065692478, 6.794333048064033, 6.92015403043559, 7.045975012807145, 7.171795995178702, 7.297616977550259, 7.423437959921814, 7.5492589422933705, 7.675079924664926, 7.800900907036483, 7.9267218894080385, 8.052542871779595, 8.178363854151153, 8.304184836522706, 8.430005818894264, 8.55582680126582, 8.681647783637375, 8.807468766008931, 8.933289748380489, 9.059110730752044, 9.1849317131236, 9.310752695495157, 9.436573677866713, 9.562394660238269, 9.688215642609826, 9.814036624981382, 9.939857607352938, 10.065678589724493, 10.19149957209605, 10.317320554467605, 10.443141536839162, 10.56896251921072, 10.694783501582274, 10.820604483953831, 10.946425466325387, 11.072246448696944, 11.198067431068498, 11.323888413440056, 11.449709395811613, 11.575530378183167, 11.701351360554725, 11.82717234292628, 11.952993325297836, 12.078814307669393, 12.20463529004095, 12.330456272412505, 12.45627725478406, 12.582098237155618, 12.707919219527174, 12.833740201898728, 12.959561184270287, 13.085382166641843, 13.211203149013397, 13.337024131384956, 13.462845113756511, 13.588666096128065, 13.714487078499625, 13.84030806087118, 13.966129043242734, 14.09195002561429, 14.21777100798585, 14.343591990357403, 14.469412972728959, 14.595233955100518, 14.721054937472072, 14.846875919843628, 14.972696902215183, 15.098517884586741, 15.224338866958297, 15.350159849329852, 15.47598083170141, 15.601801814072966, 15.727622796444521, 15.853443778816077, 15.979264761187634, 16.10508574355919, 16.230906725930744, 16.356727708302305, 16.48254869067386, 16.608369673045413, 16.73419065541697, 16.860011637788528, 16.98583262016008, 17.11165360253164, 17.237474584903197, 17.36329556727475, 17.489116549646308, 17.614937532017862, 17.74075851438942, 17.866579496760977, 17.99240047913253, 18.11822146150409, 18.244042443875646, 18.3698634262472, 18.495684408618757, 18.621505390990315, 18.74732637336187, 18.873147355733426, 18.998968338104984, 19.124789320476538, 19.250610302848095, 19.376431285219653, 19.502252267591206, 19.628073249962764, 19.753894232334318, 19.879715214705875, 20.005536197077433, 20.131357179448987, 20.257178161820544, 20.3829991441921, 20.508820126563656, 20.63464110893521, 20.76046209130677, 20.886283073678324, 21.01210405604988, 21.13792503842144, 21.263746020792993, 21.389567003164547, 21.515387985536105, 21.641208967907662, 21.76702995027922, 21.892850932650774, 22.01867191502233, 22.14449289739389, 22.270313879765443, 22.396134862136996, 22.521955844508557, 22.64777682688011, 22.773597809251665, 22.899418791623226, 23.02523977399478, 23.151060756366334, 23.27688173873789, 23.40270272110945, 23.528523703481003, 23.65434468585256, 23.780165668224118, 23.905986650595672, 24.03180763296723, 24.157628615338787, 24.28344959771034, 24.4092705800819, 24.535091562453452, 24.66091254482501, 24.786733527196567, 24.91255450956812, 25.03837549193968, 25.164196474311236, 25.29001745668279, 25.415838439054347, 25.5416594214259, 25.667480403797455, 25.793301386169016, 25.919122368540574, 26.044943350912128, 26.170764333283685, 26.29658531565524, 26.422406298026793, 26.54822728039835, 26.67404826276991, 26.799869245141466, 26.925690227513023, 27.051511209884577, 27.17733219225613, 27.30315317462769, 27.42897415699925, 27.554795139370803, 27.68061612174236, 27.806437104113915, 27.93225808648547, 28.058079068857026, 28.18390005122858, 28.30972103360014, 28.4355420159717, 28.561362998343252, 28.687183980714806, 28.813004963086364, 28.938825945457918, 29.064646927829475, 29.190467910201036, 29.31628889257259, 29.442109874944144, 29.5679308573157, 29.693751839687256, 29.819572822058813, 29.945393804430367, 30.071214786801928, 30.197035769173482, 30.32285675154504, 30.448677733916593, 30.57449871628815, 30.700319698659705, 30.82614068103126, 30.95196166340282, 31.077782645774377, 31.20360362814593, 31.32942461051749, 31.455245592889042, 31.581066575260596, 31.706887557632154, 31.832708540003715, 31.95852952237527, 32.08435050474682, 32.21017148711838, 32.33599246948994, 32.46181345186149, 32.587634434233046, 32.71345541660461, 32.83927639897616, 32.96509738134772, 33.090918363719275, 33.216739346090826, 33.34256032846238, 33.46838131083394, 33.5942022932055, 33.720023275577056, 33.84584425794861, 33.97166524032016, 34.09748622269172, 34.22330720506328, 34.349128187434836, 34.47494916980639, 34.60077015217795, 34.7265911345495, 34.85241211692106, 34.978233099292616, 35.104054081664174, 35.229875064035724, 35.35569604640729, 35.48151702877884, 35.6073380111504, 35.733158993521954, 35.85897997589351, 35.98480095826506, 36.11062194063662, 36.23644292300818, 36.362263905379734, 36.48808488775129, 36.61390587012285, 36.7397268524944, 36.86554783486596, 36.991368817237515, 37.11718979960907, 37.24301078198063, 37.36883176435219, 37.49465274672374, 37.620473729095295, 37.74629471146685, 37.87211569383841, 37.99793667620997, 38.123757658581525, 38.249578640953075, 38.37539962332463, 38.50122060569619, 38.62704158806774, 38.752862570439305, 38.87868355281086, 39.00450453518241, 39.13032551755397, 39.25614649992553, 39.38196748229708, 39.507788464668636, 39.6336094470402, 39.75943042941175, 39.88525141178331, 40.011072394154866, 40.136893376526416, 40.26271435889797, 40.38853534126953, 40.51435632364109, 40.640177306012646, 40.7659982883842, 40.891819270755754, 41.01764025312731, 41.14346123549887, 41.26928221787042, 41.395103200241984, 41.52092418261354, 41.64674516498509, 41.77256614735665, 41.898387129728206, 42.02420811209976, 42.150029094471314, 42.27585007684288, 42.40167105921443, 42.52749204158599, 42.653313023957544, 42.779134006329095, 42.90495498870065, 43.03077597107221, 43.15659695344377, 43.282417935815324, 43.40823891818688, 43.53405990055844, 43.65988088292999, 43.78570186530155, 43.911522847673105, 44.03734383004466, 44.16316481241622, 44.28898579478778, 44.41480677715933, 44.540627759530885, 44.66644874190244, 44.79226972427399, 44.91809070664556, 45.043911689017115, 45.169732671388665, 45.29555365376022, 45.42137463613178, 45.54719561850333, 45.67301660087489, 45.79883758324645, 45.924658565618, 46.05047954798956, 46.17630053036112, 46.30212151273267, 46.427942495104226, 46.55376347747578, 46.67958445984734, 46.8054054422189, 46.931226424590456, 47.057047406962006, 47.182868389333564, 47.30868937170512, 47.43451035407667, 47.560331336448236, 47.68615231881979, 47.811973301191344, 47.9377942835629, 48.06361526593446, 48.18943624830601, 48.315257230677574, 48.44107821304913, 48.56689919542068, 48.69272017779224, 48.8185411601638, 48.94436214253535, 49.070183124906904, 49.19600410727847, 49.32182508965002, 49.44764607202158, 49.573467054393134, 49.699288036764685, 49.82510901913624, 49.9509300015078, 50.07675098387936, 50.202571966250915, 50.32839294862247, 50.45421393099402, 50.58003491336558, 50.705855895737145, 50.831676878108695, 50.95749786048025, 51.0833188428518, 51.20913982522336, 51.33496080759491, 51.460781789966475, 51.58660277233803, 51.71242375470958, 51.83824473708115, 51.9640657194527, 52.089886701824256, 52.215707684195806, 52.34152866656737, 52.46734964893893, 52.59317063131048, 52.718991613682036, 52.844812596053586, 52.97063357842515, 53.0964545607967, 53.22227554316826, 53.34809652553982, 53.47391750791137, 53.59973849028293, 53.72555947265448, 53.851380455026046, 53.9772014373976, 54.103022419769154, 54.22884340214071, 54.35466438451226, 54.480485366883826, 54.60630634925538, 54.732127331626934, 54.8579483139985, 54.98376929637005, 55.10959027874161, 55.23541126111316, 55.36123224348472, 55.487053225856265, 55.61287420822783, 55.73869519059939, 55.86451617297094, 55.9903371553425, 56.11615813771405, 56.24197912008561, 56.36780010245716, 56.493621084828725, 56.61944206720028, 56.74526304957183, 56.8710840319434, 56.99690501431494, 57.122725996686505, 57.248546979058055, 57.37436796142961, 57.50018894380118, 57.62600992617273, 57.751830908544285, 57.877651890915836, 58.0034728732874, 58.12929385565895, 58.25511483803051, 58.38093582040207, 58.50675680277362, 58.63257778514518, 58.75839876751673, 58.88421974988829, 59.01004073225984, 59.1358617146314, 59.26168269700296, 59.38750367937451, 59.513324661746076, 59.639145644117626, 59.76496662648918, 59.890787608860734, 60.0166085912323, 60.142429573603856, 60.268250555975406, 60.394071538346964, 60.519892520718514, 60.64571350309008, 60.77153448546163, 60.89735546783319, 61.02317645020475, 61.1489974325763, 61.27481841494786, 61.40063939731941, 61.526460379690974, 61.65228136206252, 61.77810234443408, 61.90392332680564, 62.02974430917719, 62.155565291548754, 62.281386273920305, 62.40720725629186, 62.53302823866341, 62.65884922103498, 62.784670203406534, 62.910491185778085, 63.03631216814965, 63.16213315052119, 63.28795413289276, 63.41377511526431, 63.539596097635865, 63.66541708000743, 63.79123806237898, 63.91705904475054, 64.0428800271221, 64.16870100949365, 64.2945219918652, 64.42034297423676, 64.54616395660832, 64.67198493897988, 64.79780592135143, 64.92362690372298, 65.04944788609454, 65.17526886846609, 65.30108985083766, 65.42691083320922, 65.55273181558077, 65.67855279795232, 65.80437378032387, 65.93019476269544, 66.05601574506699, 66.18183672743855, 66.3076577098101, 66.43347869218165, 66.55929967455322, 66.68512065692477, 66.81094163929633, 66.93676262166788, 67.06258360403945, 67.188404586411, 67.31422556878255, 67.44004655115411, 67.56586753352566, 67.69168851589723, 67.81750949826878, 67.94333048064033, 68.06915146301189, 68.19497244538344, 68.320793427755, 68.44661441012656, 68.57243539249812, 68.69825637486967, 68.82407735724122, 68.94989833961279, 69.07571932198434, 69.2015403043559, 69.32736128672745, 69.453182269099, 69.57900325147055, 69.70482423384212, 69.83064521621368, 69.95646619858523, 70.0822871809568, 70.20810816332835, 70.3339291456999, 70.45975012807145, 70.58557111044301, 70.71139209281458, 70.83721307518613, 70.96303405755768, 71.08885503992923, 71.2146760223008, 71.34049700467234, 71.46631798704391, 71.59213896941547, 71.71795995178702, 71.84378093415857, 71.96960191653012, 72.09542289890169, 72.22124388127324, 72.3470648636448, 72.47288584601635, 72.5987068283879, 72.72452781075947, 72.85034879313102, 72.97616977550258, 73.10199075787413, 73.2278117402457, 73.35363272261725, 73.4794537049888, 73.60527468736036, 73.73109566973191, 73.85691665210348, 73.98273763447503, 74.10855861684658, 74.23437959921814, 74.3602005815897, 74.48602156396126, 74.61184254633281, 74.73766352870437, 74.86348451107592, 74.98930549344747, 75.11512647581904, 75.24094745819059, 75.36676844056215, 75.4925894229337, 75.61841040530526, 75.74423138767682, 75.87005237004837, 75.99587335241993, 76.12169433479148, 76.24751531716305, 76.3733362995346, 76.49915728190615, 76.62497826427771, 76.75079924664927, 76.87662022902083, 77.00244121139238, 77.12826219376393, 77.25408317613548, 77.37990415850705, 77.50572514087861, 77.63154612325016, 77.75736710562173, 77.88318808799328, 78.00900907036483, 78.13483005273638, 78.26065103510794, 78.3864720174795, 78.51229299985106, 78.6381139822226, 78.76393496459416, 78.88975594696572, 79.01557692933727, 79.14139791170884, 79.2672188940804, 79.39303987645195, 79.5188608588235, 79.64468184119505, 79.77050282356662, 79.89632380593817, 80.02214478830973, 80.14796577068128, 80.27378675305283, 80.3996077354244, 80.52542871779595, 80.65124970016751, 80.77707068253906, 80.90289166491063, 81.02871264728218, 81.15453362965373, 81.28035461202529, 81.40617559439684, 81.5319965767684, 81.65781755913996, 81.78363854151151, 81.90945952388307, 82.03528050625462, 82.16110148862619, 82.28692247099774, 82.4127434533693, 82.53856443574084, 82.6643854181124, 82.79020640048397, 82.91602738285552, 83.04184836522708, 83.16766934759863, 83.29349032997018, 83.41931131234173, 83.5451322947133, 83.67095327708486, 83.79677425945641, 83.92259524182798, 84.04841622419951, 84.17423720657108, 84.30005818894263, 84.4258791713142, 84.55170015368576, 84.67752113605731, 84.80334211842886, 84.92916310080041, 85.05498408317197, 85.18080506554352, 85.30662604791509, 85.43244703028665, 85.55826801265819, 85.68408899502975, 85.8099099774013, 85.93573095977287, 86.06155194214442, 86.18737292451598, 86.31319390688753, 86.43901488925908, 86.56483587163065, 86.6906568540022, 86.81647783637376, 86.94229881874531, 87.06811980111688, 87.19394078348843, 87.31976176585998, 87.44558274823154, 87.5714037306031, 87.69722471297466, 87.82304569534621, 87.94886667771776, 88.07468766008932, 88.20050864246087, 88.32632962483244, 88.45215060720399, 88.57797158957555, 88.70379257194709, 88.82961355431866, 88.95543453669022, 89.08125551906177, 89.20707650143333, 89.33289748380489, 89.45871846617644, 89.58453944854799, 89.71036043091955, 89.83618141329111, 89.96200239566267, 90.08782337803423, 90.21364436040577, 90.33946534277733, 90.46528632514888, 90.59110730752045, 90.71692828989201, 90.84274927226356, 90.96857025463511, 91.09439123700666, 91.22021221937823, 91.34603320174978, 91.47185418412134, 91.5976751664929, 91.72349614886444, 91.849317131236, 91.97513811360756, 92.10095909597912, 92.22678007835067, 92.35260106072224, 92.47842204309379, 92.60424302546534, 92.7300640078369, 92.85588499020845, 92.98170597258002, 93.10752695495157, 93.23334793732312, 93.35916891969468, 93.48498990206623, 93.6108108844378, 93.73663186680935, 93.86245284918091, 93.98827383155246, 94.11409481392401, 94.23991579629558, 94.36573677866713, 94.49155776103869, 94.61737874341024, 94.74319972578179, 94.86902070815334, 94.99484169052491, 95.12066267289647, 95.24648365526802, 95.37230463763959, 95.49812562001114, 95.62394660238269, 95.74976758475424, 95.8755885671258, 96.00140954949737, 96.12723053186892, 96.25305151424047, 96.37887249661202, 96.50469347898358, 96.63051446135515, 96.7563354437267, 96.88215642609826, 97.00797740846981, 97.13379839084136, 97.25961937321291, 97.38544035558448, 97.51126133795604, 97.6370823203276, 97.76290330269914, 97.8887242850707, 98.01454526744226, 98.14036624981381, 98.26618723218537, 98.39200821455694, 98.51782919692849, 98.64365017930004, 98.76947116167159, 98.89529214404315, 99.0211131264147, 99.14693410878627, 99.27275509115782, 99.39857607352937, 99.52439705590093, 99.65021803827248, 99.77603902064405, 99.9018600030156, 100.02768098538716, 100.15350196775871, 100.27932295013026, 100.40514393250183, 100.53096491487338];
        let p = io::read_json("datasets/wavegen/sine/sine.json");
        let epsilon = 0.000000000000001; 
        match p.input_data {
            io::Data::<_>::Array(vals) => {
                for i in 0..input.len() {
                    if input[i] == 0.0 {
                        assert!(vals[i] == input[i]);
                    }
                    else {
                        let delta = (input[i] - vals[i]).abs();
                        let percent_error = delta / input[i];
                        assert!(percent_error < epsilon);
                    }
                }
            }
            _ => {panic!()}
        }
    }

    #[test]
    fn read_json() {
        let function = "read_json";
        let mag = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let phase = vec![5.0, 6.0, 7.0, 8.0];
        let output_data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let path = "datasets/io";

        let p = io::read_json("datasets/io/read_json.json");

        match p.input_data {
            io::Data::<_>::ComplexVals{mag: r_mag, phase: r_phase} => {
                assert_eq!(r_mag, mag);
                assert_eq!(r_phase, phase);
            }
            _ => {panic!()}
        }

        match p.output_data {
            io::Data::<_>::Array(vals) => {
                assert_eq!(vals, output_data);
            }
            _ => {panic!()}
        }

        match p.function {
            val => {assert_eq!(val, function)}
        }

        match p.path {
            val => {assert_eq!(val, path)}
        }
        
    }   

    #[test]
    fn Json_data_type() {
        let data = r#"
        {
            "function": "read_json",
            "input_data": {
                "ComplexVals": {
                    "mag": [
                        0.0, 1.0, 2.0, 3.0, 4.0
                    ],
                    "phase": [
                        5.0, 6.0, 7.0, 8.0
                    ]
                }
            },
            "output_data": {
                "Array": [
                    0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0
                ]
            },
            "path": "datasets/io"
        }"#;

        let function = "read_json";
        let mag = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let phase = vec![5.0, 6.0, 7.0, 8.0];
        let output_data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let path = "datasets/io";

        let p: io::Json = serde_json::from_str(data).unwrap();

        match p.input_data {
            io::Data::<_>::ComplexVals{mag: r_mag, phase: r_phase} => {
                assert_eq!(r_mag, mag);
                assert_eq!(r_phase, phase);
            }
            _ => {panic!()}
        }

        match p.output_data {
            io::Data::<_>::Array(vals) => {
                assert_eq!(vals, output_data);
            }
            _ => {panic!()}
        }

        match p.function {
            val => {assert_eq!(val, function)}
        }

        match p.path {
            val => {assert_eq!(val, path)}
        }
        
    }

    #[test]
    fn find_root() {
        let root = "RusticFourier";
        let change = Path::new("datasets/wavegen/sinc");
        env::set_current_dir(change).unwrap();

        let answer = PathBuf::from("./../../..");
        let result = io::PathManage::find_root(root).unwrap();

        assert_eq!(result, answer);
        
    }

    #[test]
    fn path() {
        let root = io::PathManage::new("RusticFourier");
        root.path("datasets/io");
    }
}