import React, {useState} from 'react';
import {Table, Button, Row, Col, Modal, Form} from 'react-bootstrap';

export default function Step1() {
    const groups = [{id: 1, name: "Admin", priviledge: 9, default: false, editable: false},
                    {id: 2, name: "User", priviledge: 3, default: true, editable: true}];

    return (
        <>
        <Row className="justify-content-md-center my-2">
            <div className="flex">
            <Table striped bordered hover>
                <thead>
                    <tr>
                    <th>#</th>
                    <th>Group Name</th>
                    <th>Priviledge</th>
                    <th>Default Group</th>
                    <th>Edit</th>
                    </tr>
                </thead>
                <tbody>
                    {groups.map((group) => 
                        <tr key={group.id}>
                            <td>{group.id}</td>
                            <td>{group.name}</td>
                            <td>{group.priviledge}</td>
                            <td>
                                {group.default && <span>Yes</span>}
                                {!group.default && <span>No</span>}
                            </td>
                            <td>
                                {group.editable && <Button variant="outline-info" className="btn-sm">Edit</Button>}
                                {!group.editable && <span>-</span>}
                            </td>
                        </tr>)}
                </tbody>
            </Table>
            </div>
        </Row>
        <Row className="justify-content-md-center my-2">
            <AddItemButton />
        </Row>
        </>
    );
}

function AddItemButton() {
    const [show, setShow] = useState(false);

    const handleClose = () => setShow(false);
    const handleShow = () => setShow(true);

    return (
        <Row className="justify-content-md-center my-2">
            <Col md="auto">
                <Button variant="info" onClick={handleShow}>
                    Add Group
                </Button>

                <Modal show={show} onHide={handleClose} animation={false}>
                    <Modal.Header closeButton>
                    <Modal.Title>Add Group</Modal.Title>
                    </Modal.Header>
                    <Modal.Body>
                    <Form>
                        <Form.Group controlId="formBasicEmail">
                        <Form.Label>Group name</Form.Label>
                        <Form.Control type="text" placeholder="Enter group name" />
                        <Form.Label>Group priviledge</Form.Label>
                        <Form.Control type="text" placeholder="Enter group priviledge" />
                        <Form.Label>Default Group</Form.Label>
                        <Form.Control as="select">
                            <option>No</option>
                            <option>Yes</option>
                        </Form.Control>
                        </Form.Group>
                    </Form>
                    </Modal.Body>
                    <Modal.Footer>
                    <Button variant="secondary" onClick={handleClose}>
                        Close
                    </Button>
                    <Button variant="primary" onClick={handleClose}>
                        Save Changes
                    </Button>
                    </Modal.Footer>
                </Modal>
            </Col>
        </Row>
    );
}
