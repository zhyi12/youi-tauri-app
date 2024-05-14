use std::f64::consts::PI;
use geo::{coord, GeodesicDestination, LineString, point, Polygon};

///
/// 六边形
///
#[derive(Clone, Copy)]
pub struct Hexagon{

    ///
    /// 中心x坐标
    ///
    pub x: f64,

    ///
    /// 中心y坐标
    ///
    pub y: f64,
    ///
    /// 六边形边长
    ///
    s: f64,

}

impl Hexagon {
    pub fn new(x:f64,y:f64,s:f64)->Self{
        Self{
            x,y,s
        }
    }
}

impl Hexagon{

    pub fn get_size(&self)->&f64{
        &self.s
    }

    ///
    /// 六边形的宽width=sqrt(3)/2*size，
    ///
    pub fn get_width(&self)->f64{
        3.0_f64.sqrt()/2.0*&self.s
    }
    ///
    /// 六边形的高是height=size*2
    ///
    pub fn get_height(&self)->f64{
        &self.s*2.
    }

    ///
    /// 和相邻六边形的垂直距离是vert=1.5*size
    ///
    pub fn get_vert(&self)->f64{
        &self.s*1.5
    }
}

impl Hexagon{

    ///
    /// 地理多边形区域
    ///
    pub fn to_geo_poly(&self)->Polygon{
        // 中心点
        let c = point!{x:self.x,y:self.y};
        let tp = c.geodesic_destination(30.,self.s);

        let line = LineString::new(vec![
            tp.0,
            c.geodesic_destination(90.,self.s).0,
            c.geodesic_destination(150.,self.s).0,
            c.geodesic_destination(210.,self.s).0,
            c.geodesic_destination(270.,self.s).0,
            c.geodesic_destination(330.,self.s).0,
            tp.0.clone()
        ]);
        Polygon::new(line,vec![])
    }

    ///
    /// 根据固定的角度创建邻居六边形
    /// 0   上
    /// 60  下右
    /// 120 上右
    /// 180
    /// 240
    /// 300
    ///
    ///
    pub fn neighbor(&self,bearing:f64)->Self{
        let move_distance = self.s*(30.*PI/180.).cos()*2.;
        let p = point! {x:self.x,y:self.y};
        let target_p = p.geodesic_destination(bearing,move_distance);
        Self{
            x: target_p.x(),
            y: target_p.y(),
            s: self.s
        }
    }
}

