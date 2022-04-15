import {Badge, Button, Card, Group} from "@mantine/core";
import React from "react";


interface Props {
    image: string,
    title: string,
    description: string
    buttonText: string
}

export const MyCard = (props: Props) => {
    const {image, title, description, buttonText} = props;
    return (     <div style={{ width: 340 }}>
        <Card shadow="sm" p="lg">
            <Card.Section>
                <img style={{width: 300}} src={image} />
            </Card.Section>

            <Group position="apart" style={{marginBottom: 5}}>
                {title}
                <Badge color="pink" variant="light">
                    On Sale
                </Badge>
            </Group>

            {description}

            <Button variant="light" color="blue" fullWidth style={{marginTop: 14}}>
                {buttonText}
            </Button>
        </Card>
        </div>
    )
}