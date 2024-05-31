import * as THREE from'three'
//饼图继承Group(对象组合)类
class Pie extends THREE.Group {
    constructor (option) {
        super()

        this.option = Object.assign({
            inRadius:1500,
            outRadius:5000,
            intervalAngle: 0,//每个扇形之间间距度数，代表每个扇形之间的间距，为0的话则无间距
            interval: 0,//每个扇形距离原本中心坐标点的额外距离，如果过高会导致扇形离中心点散开
        }, option)
        //总值
        let sum = 0
        //总间距=扇形数量x每个扇形之间间距度数
        this.totalInterval = this.option.intervalAngle * this.option.items.length
        //遍历传入的数组，获取总值
        this.option.items.forEach(item =>{sum += item.value})
        //绘制扇形总角度为
        this.totalAngle = 360 - this.totalInterval
        //随机起始度数
        let current = Math.random() * 360
        //根据传入的数组绘制多个扇形
        for (let i = 0, len = this.option.items.length; i < len; i++) {
            let item = this.option.items[i]
            //扇形
            let geometry = this.sector(this.option.inRadius, this.option.outRadius, current, current + item.value / sum * this.totalAngle, this.option.height)
            // 赋予扇形材质
            let mesh = new THREE.Mesh(geometry, new THREE.MeshPhongMaterial({ color: item.color, opacity: 1, transparent: true }))
            //扇形中心坐标点=(起始度数+最终度数）/2*Π/180
            let center = (current + current + item.value / sum * this.totalAngle) / 2 * Math.PI / 180
            //扇形坐标点=扇形中心坐标点*离中心点距离
            mesh.position.x += this.option.interval * Math.cos(center);
            mesh.position.y += this.option.interval * Math.sin(center);

            this.add(mesh)
            //起始度数加上值对应的间隔度数（值对应的间隔度数=总间隔度数*（值与总值的比例））
            current += item.value / sum * this.totalAngle
            //起始度数加上单位间隔度数
            current += this.option.intervalAngle
        }
        //调整整体饼图的旋转角度,可加。也可在外部调整
        // this.rotation.x = -Math.PI / 3
    }
    //绘制扇形(内边圆半径，外边圆半径，起始角度，终止角度，高度)
    sector (inRadius, outRadius, begin, end, height) {
        const shape = new THREE.Shape()
        //如果内边圆半径为0
        if (inRadius == 0) {
            //那么绘制扇形的圆心点为0
            shape.moveTo(0, 0)
        } else {
            //否则设置绘制扇形的圆心点内边圆半径
            shape.moveTo(inRadius, 0)
            //绘制内边框扇形圆弧
            shape.absarc(0, 0, inRadius, begin / 180 * Math.PI, end / 180 * Math.PI, false)
        }

        shape.lineTo(outRadius * Math.cos(end / 180 * Math.PI), outRadius * Math.sin(end / 180 * Math.PI))
        //绘制外边框扇形圆弧
        shape.absarc(0, 0, outRadius, end / 180 * Math.PI, begin / 180 * Math.PI, true)

        shape.lineTo(inRadius * Math.cos(begin / 180 * Math.PI), inRadius * Math.sin(begin / 180 * Math.PI))
        //将绘制的平面扇形挤出设置的高度，使其变成3D扇形(shape平面扇形，amount挤压高度，bevelEnabled是否设置斜角，steps指定拉伸体沿深度方向分成多少段)
        return new THREE.ExtrudeGeometry(shape, { depth:1000,amount: 0, bevelEnabled: false, steps: 1 })
    }
    //添加文字（暂无）
    addText (text, x, y, z) {

    }
}

export default Pie