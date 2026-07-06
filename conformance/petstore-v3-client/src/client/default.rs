impl crate::client::Client {
    /// Add a new pet to the store.
    ///
    /// POST /pet
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /pet",
                otel.kind = "client",
                url.template = "/pet",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn add_pet(
        &self,
        request: impl Into<crate::types::Pet>,
    ) -> Result<crate::types::Pet, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet");
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .post(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Update an existing pet by Id.
    ///
    /// PUT /pet
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "PUT /pet",
                otel.kind = "client",
                url.template = "/pet",
                http.request.method = "PUT",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn update_pet(
        &self,
        request: impl Into<crate::types::Pet>,
    ) -> Result<crate::types::Pet, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet");
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .put(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Multiple status values can be provided with comma separated strings.
    ///
    /// GET /pet/findByStatus
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /pet/findByStatus",
                otel.kind = "client",
                url.template = "/pet/findByStatus",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn find_pets_by_status(
        &self,
        query: &parameters::FindPetsByStatusQuery,
    ) -> Result<::std::vec::Vec<crate::types::Pet>, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["pet", "findByStatus"]);
                let url = ::ploidy_util::serde::Serialize::serialize(
                    query,
                    ::ploidy_util::QuerySerializer::new(
                        url,
                        parameters::FindPetsByStatusQuery::STYLES,
                    ),
                )?;
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Multiple tags can be provided with comma separated strings. Use tag1, tag2,
    /// tag3 for testing.
    ///
    /// GET /pet/findByTags
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /pet/findByTags",
                otel.kind = "client",
                url.template = "/pet/findByTags",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn find_pets_by_tags(
        &self,
        query: &parameters::FindPetsByTagsQuery,
    ) -> Result<::std::vec::Vec<crate::types::Pet>, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["pet", "findByTags"]);
                let url = ::ploidy_util::serde::Serialize::serialize(
                    query,
                    ::ploidy_util::QuerySerializer::new(
                        url,
                        parameters::FindPetsByTagsQuery::STYLES,
                    ),
                )?;
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Returns a single pet.
    ///
    /// GET /pet/{petId}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /pet/{petId}",
                otel.kind = "client",
                url.template = "/pet/{petId}",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                pet_id = %pet_id
            )
        )
    )]
    pub async fn get_pet_by_id(
        &self,
        pet_id: &str,
    ) -> Result<crate::types::Pet, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet")
                    .push(pet_id);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Updates a pet resource based on the form data.
    ///
    /// POST /pet/{petId}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /pet/{petId}",
                otel.kind = "client",
                url.template = "/pet/{petId}",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                pet_id = %pet_id
            )
        )
    )]
    pub async fn update_pet_with_form(
        &self,
        pet_id: &str,
        query: &parameters::UpdatePetWithFormQuery,
    ) -> Result<crate::types::Pet, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet")
                    .push(pet_id);
                let url = ::ploidy_util::serde::Serialize::serialize(
                    query,
                    ::ploidy_util::QuerySerializer::new(
                        url,
                        parameters::UpdatePetWithFormQuery::STYLES,
                    ),
                )?;
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.post(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Delete a pet.
    ///
    /// DELETE /pet/{petId}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "DELETE /pet/{petId}",
                otel.kind = "client",
                url.template = "/pet/{petId}",
                http.request.method = "DELETE",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                pet_id = %pet_id
            )
        )
    )]
    pub async fn delete_pet(&self, pet_id: &str) -> Result<(), crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet")
                    .push(pet_id);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.delete(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let _ = response;
            Ok(())
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Upload image of the pet.
    ///
    /// POST /pet/{petId}/uploadImage
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /pet/{petId}/uploadImage",
                otel.kind = "client",
                url.template = "/pet/{petId}/uploadImage",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                pet_id = %pet_id
            )
        )
    )]
    pub async fn upload_file(
        &self,
        pet_id: &str,
        query: &parameters::UploadFileQuery,
        request: impl Into<::ploidy_util::serde_json::Value>,
    ) -> Result<crate::types::ApiResponse, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("pet")
                    .push(pet_id)
                    .push("uploadImage");
                let url = ::ploidy_util::serde::Serialize::serialize(
                    query,
                    ::ploidy_util::QuerySerializer::new(url, parameters::UploadFileQuery::STYLES),
                )?;
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .post(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Returns a map of status codes to quantities.
    ///
    /// GET /store/inventory
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /store/inventory",
                otel.kind = "client",
                url.template = "/store/inventory",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn get_inventory(
        &self,
    ) -> Result<::std::collections::BTreeMap<::std::string::String, i32>, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["store", "inventory"]);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Place a new order in the store.
    ///
    /// POST /store/order
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /store/order",
                otel.kind = "client",
                url.template = "/store/order",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn place_order(
        &self,
        request: impl Into<crate::types::Order>,
    ) -> Result<crate::types::Order, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["store", "order"]);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .post(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// For valid response try integer IDs with value <= 5 or > 10. Other values will
    /// generate exceptions.
    ///
    /// GET /store/order/{orderId}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /store/order/{orderId}",
                otel.kind = "client",
                url.template = "/store/order/{orderId}",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                order_id = %order_id
            )
        )
    )]
    pub async fn get_order_by_id(
        &self,
        order_id: &str,
    ) -> Result<crate::types::Order, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["store", "order"])
                    .push(order_id);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// For valid response try integer IDs with value < 1000. Anything above 1000 or
    /// non-integers will generate API errors.
    ///
    /// DELETE /store/order/{orderId}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "DELETE /store/order/{orderId}",
                otel.kind = "client",
                url.template = "/store/order/{orderId}",
                http.request.method = "DELETE",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                order_id = %order_id
            )
        )
    )]
    pub async fn delete_order(&self, order_id: &str) -> Result<(), crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["store", "order"])
                    .push(order_id);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.delete(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let _ = response;
            Ok(())
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// This can only be done by the logged in user.
    ///
    /// POST /user
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /user",
                otel.kind = "client",
                url.template = "/user",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn create_user(
        &self,
        request: impl Into<crate::types::User>,
    ) -> Result<crate::types::User, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("user");
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .post(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Creates list of users with given input array.
    ///
    /// POST /user/createWithList
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "POST /user/createWithList",
                otel.kind = "client",
                url.template = "/user/createWithList",
                http.request.method = "POST",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn create_users_with_list_input(
        &self,
        request: impl Into<::std::vec::Vec<crate::types::User>>,
    ) -> Result<crate::types::User, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["user", "createWithList"]);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .post(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Log into the system.
    ///
    /// GET /user/login
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /user/login",
                otel.kind = "client",
                url.template = "/user/login",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn login_user(
        &self,
        query: &parameters::LoginUserQuery,
    ) -> Result<::std::string::String, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["user", "login"]);
                let url = ::ploidy_util::serde::Serialize::serialize(
                    query,
                    ::ploidy_util::QuerySerializer::new(url, parameters::LoginUserQuery::STYLES),
                )?;
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Log user out of the system.
    ///
    /// GET /user/logout
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /user/logout",
                otel.kind = "client",
                url.template = "/user/logout",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type
            )
        )
    )]
    pub async fn logout_user(&self) -> Result<(), crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .extend(&["user", "logout"]);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let _ = response;
            Ok(())
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// Get user detail based on username.
    ///
    /// GET /user/{username}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "GET /user/{username}",
                otel.kind = "client",
                url.template = "/user/{username}",
                http.request.method = "GET",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                username = %username
            )
        )
    )]
    pub async fn get_user_by_name(
        &self,
        username: &str,
    ) -> Result<crate::types::User, crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("user")
                    .push(username);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.get(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let body = response.bytes().await?;
            let deserializer = &mut ::ploidy_util::serde_json::Deserializer::from_slice(&body);
            let result = ::ploidy_util::serde_path_to_error::deserialize(deserializer)?;
            Ok(result)
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// This can only be done by the logged in user.
    ///
    /// PUT /user/{username}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "PUT /user/{username}",
                otel.kind = "client",
                url.template = "/user/{username}",
                http.request.method = "PUT",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                username = %username
            )
        )
    )]
    pub async fn update_user(
        &self,
        username: &str,
        request: impl Into<crate::types::User>,
    ) -> Result<(), crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("user")
                    .push(username);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self
                    .client
                    .put(url)
                    .headers(self.headers.clone())
                    .json(&request.into());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let _ = response;
            Ok(())
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
    /// This can only be done by the logged in user.
    ///
    /// DELETE /user/{username}
    #[cfg_attr(
        feature = "tracing",
        ::tracing::instrument(
            skip_all,
            fields(
                otel.name = "DELETE /user/{username}",
                otel.kind = "client",
                url.template = "/user/{username}",
                http.request.method = "DELETE",
                server.address,
                server.port,
                url.full,
                http.response.status_code,
                error.type,
                username = %username
            )
        )
    )]
    pub async fn delete_user(&self, username: &str) -> Result<(), crate::error::Error> {
        let result: Result<_, crate::error::Error> = async move {
            let url = {
                let mut url = self.base_url.clone();
                url.path_segments_mut()
                    .map_err(|()| ::ploidy_util::url::PathAndQueryError::UrlCannotBeABase)?
                    .pop_if_empty()
                    .push("user")
                    .push(username);
                #[cfg(feature = "tracing")]
                {
                    ::tracing::record_all!(
                        ::tracing::Span::current(),
                        server.address = url.host_str(),
                        server.port = url.port_or_known_default(),
                        url.full = url.as_str(),
                    );
                }
                url
            };
            let request = {
                let request = self.client.delete(url).headers(self.headers.clone());
                #[cfg(feature = "trace-context")]
                let request = ::ploidy_util::trace::propagate(::tracing::Span::current(), request);
                request
            };
            let response = request.send().await?;
            #[cfg(feature = "tracing")]
            {
                ::tracing::record_all!(
                    ::tracing::Span::current(),
                    http.response.status_code = response.status().as_u16()
                );
            }
            let response = response.error_for_status()?;
            let _ = response;
            Ok(())
        }
        .await;
        #[cfg(feature = "tracing")]
        if let Err(err) = &result {
            ::tracing::record_all!(
                ::tracing::Span::current(), error. type = % err.category(),
            );
        }
        result
    }
}
pub mod parameters {
    mod find_pets_by_status_query {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            Default,
            ::ploidy_util::serde::Serialize,
            ::ploidy_util::serde::Deserialize,
        )]
        #[serde(crate = "::ploidy_util::serde")]
        pub struct FindPetsByStatusQuery {
            pub status: crate::client::default::types::FindPetsByStatusQueryStatus,
        }
        impl FindPetsByStatusQuery {
            pub const STYLES: &[(&str, ::ploidy_util::QueryStyle)] =
                &[("status", ::ploidy_util::QueryStyle::Form { exploded: true })];
        }
    }
    pub use find_pets_by_status_query::*;
    mod find_pets_by_tags_query {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            Default,
            ::ploidy_util::serde::Serialize,
            ::ploidy_util::serde::Deserialize,
        )]
        #[serde(crate = "::ploidy_util::serde")]
        pub struct FindPetsByTagsQuery {
            pub tags: ::std::vec::Vec<::std::string::String>,
        }
        impl FindPetsByTagsQuery {
            pub const STYLES: &[(&str, ::ploidy_util::QueryStyle)] =
                &[("tags", ::ploidy_util::QueryStyle::Form { exploded: true })];
        }
    }
    pub use find_pets_by_tags_query::*;
    mod update_pet_with_form_query {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            Default,
            ::ploidy_util::serde::Serialize,
            ::ploidy_util::serde::Deserialize,
        )]
        #[serde(crate = "::ploidy_util::serde")]
        pub struct UpdatePetWithFormQuery {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status: ::std::option::Option<::std::string::String>,
        }
        impl UpdatePetWithFormQuery {
            pub const STYLES: &[(&str, ::ploidy_util::QueryStyle)] = &[];
        }
    }
    pub use update_pet_with_form_query::*;
    mod upload_file_query {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            Default,
            ::ploidy_util::serde::Serialize,
            ::ploidy_util::serde::Deserialize,
        )]
        #[serde(crate = "::ploidy_util::serde")]
        pub struct UploadFileQuery {
            #[serde(
                rename = "additionalMetadata",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub additional_metadata: ::std::option::Option<::std::string::String>,
        }
        impl UploadFileQuery {
            pub const STYLES: &[(&str, ::ploidy_util::QueryStyle)] = &[];
        }
    }
    pub use upload_file_query::*;
    mod login_user_query {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            Default,
            ::ploidy_util::serde::Serialize,
            ::ploidy_util::serde::Deserialize,
        )]
        #[serde(crate = "::ploidy_util::serde")]
        pub struct LoginUserQuery {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub username: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub password: ::std::option::Option<::std::string::String>,
        }
        impl LoginUserQuery {
            pub const STYLES: &[(&str, ::ploidy_util::QueryStyle)] = &[];
        }
    }
    pub use login_user_query::*;
}
pub mod types {
    mod find_pets_by_status_query_status {
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            PartialEq,
            ::ploidy_util::pointer::JsonPointee,
            ::ploidy_util::pointer::JsonPointerTarget,
        )]
        #[ploidy(pointer(crate = "::ploidy_util::pointer"))]
        pub enum FindPetsByStatusQueryStatus {
            Available,
            Pending,
            Sold,
            OtherFindPetsByStatusQueryStatus(String),
        }
        impl ::std::default::Default for FindPetsByStatusQueryStatus {
            fn default() -> Self {
                Self::OtherFindPetsByStatusQueryStatus(::std::string::String::default())
            }
        }
        impl ::std::fmt::Display for FindPetsByStatusQueryStatus {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(match self {
                    Self::Available => "available",
                    Self::Pending => "pending",
                    Self::Sold => "sold",
                    Self::OtherFindPetsByStatusQueryStatus(s) => s.as_str(),
                })
            }
        }
        impl ::std::str::FromStr for FindPetsByStatusQueryStatus {
            type Err = ::std::convert::Infallible;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                ::std::result::Result::Ok(match s {
                    "available" => Self::Available,
                    "pending" => Self::Pending,
                    "sold" => Self::Sold,
                    _ => Self::OtherFindPetsByStatusQueryStatus(s.to_owned()),
                })
            }
        }
        impl<'de> ::ploidy_util::serde::Deserialize<'de> for FindPetsByStatusQueryStatus {
            fn deserialize<D: ::ploidy_util::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                struct Visitor;
                impl<'de> ::ploidy_util::serde::de::Visitor<'de> for Visitor {
                    type Value = FindPetsByStatusQueryStatus;
                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str("a variant of `FindPetsByStatusQueryStatus`")
                    }
                    fn visit_str<E: ::ploidy_util::serde::de::Error>(
                        self,
                        s: &str,
                    ) -> ::std::result::Result<Self::Value, E> {
                        let ::std::result::Result::Ok(v) = ::std::str::FromStr::from_str(s);
                        Ok(v)
                    }
                }
                ::ploidy_util::serde::Deserializer::deserialize_str(deserializer, Visitor)
            }
        }
        impl ::ploidy_util::serde::Serialize for FindPetsByStatusQueryStatus {
            fn serialize<S: ::ploidy_util::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }
    }
    pub use find_pets_by_status_query_status::*;
}
