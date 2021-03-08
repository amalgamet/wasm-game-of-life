// import { withRouter } from 'next/router'
import dynamic from 'next/dynamic';
// import Link from 'next/link'

const Canvas = dynamic(() => import('../components/Canvas'), { ssr: false });

export default function Page() {
  const draw = (ctx, frameCount) => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.fillStyle = '#000000';
    ctx.beginPath();
    ctx.arc(50, 100, 20 * Math.sin(frameCount * 0.05) ** 2, 0, 2 * Math.PI);
    ctx.fill();
  };

  return <Canvas draw={draw} />;
}

// const RustComponent = dynamic({
//   loader: async () => {
//     // Import the wasm module
//     const rustModule = await import('../add.wasm')
//     // Return a React component that calls the add_one method on the wasm module
//     return (props) => <div>{rustModule.add_one(props.number)}</div>
//   },
// })

// const Page = ({ router: { query } }) => {
//   const number = parseInt(query.number || 30)
//   return (
//     <div>
//       <RustComponent number={number} />
//       <Link href={`/?number=${number + 1}`}>
//         <a>+</a>
//       </Link>
//     </div>
//   )
// }

// export default withRouter(Page)
