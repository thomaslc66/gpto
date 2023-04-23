import "./styles.css";
import CustomTitleBar from "./CustomTitleBar";
import { Layout, Row, Col } from "antd";

const { Header, Content } = Layout;

function AboutApp() {
  return (
    <Layout>
      <CustomTitleBar />
      <Header style={{ height: "40px", lineHeight: "40px" }}>
        <span
          style={{
            color: "white",
            height: "40px",
            fontSize: "30px",
            fontWeight: "bold",
          }}
        >
          About
        </span>
      </Header>
      <Content style={{ padding: "40px" }}>
        <Row align={"center"}>
          <div>
            <h1>Welcome to GPTo!</h1>
            <a href="https://tauri.app" target="_blank">
              <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
            </a>
          </div>
        </Row>
        <Row align={"center"} style={{ marginTop: "40px" }}>
          <Col>
            Thank you for using and helping GPTo. This tools is made OpenSource
            in order to help everybody to get the advantages of chat GPT in all
            you computer environement
          </Col>
        </Row>
      </Content>
    </Layout>
  );
}

export default AboutApp;
